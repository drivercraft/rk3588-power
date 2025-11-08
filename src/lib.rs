//! # RK3588 Power Management Driver
//!
//! This library provides power management functionality for RK3588 series SoCs,
//! particularly for power domain control including NPU, GPU, VPU, and other domains.
//!
#![no_std]

extern crate alloc;

use rdif_base::DriverGeneric;

use crate::{power_sequencer::PowerSequencer, registers::PmuRegs, variants::RockchipPmuInfo};
use core::ptr::NonNull;

// Make dependency_manager public for testing
pub mod dependency_manager;
mod idle_control;
mod memory_control;
mod power_sequencer;
mod qos_control;
mod registers;
mod variants;

// Re-export PowerDomain type
pub use variants::PowerDomain;

// Re-export chip-specific power domain constants as modules
pub use variants::rk3568 as RK3568;
pub use variants::rk3588 as RK3588;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RkBoard {
    Rk3568,
    Rk3588,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    /// Power domain not found
    DomainNotFound,
    /// Timeout error
    Timeout,
    /// Hardware error
    HardwareError,
    /// Memory power timeout
    MemoryPowerTimeout,
    /// Idle request timeout
    IdleRequestTimeout,
    /// Idle acknowledgment timeout
    IdleAckTimeout,
    /// Repair timeout
    RepairTimeout,
    /// Invalid operation
    InvalidOperation,
    /// Dependency not met (parent not powered or child still active)
    DependencyNotMet,
    /// QoS save/restore error
    QoSError,
    /// Invalid QoS configuration
    InvalidQoSConfig,
}

pub type PowerResult<T> = Result<T, PowerError>;

pub struct RockchipPM {
    _board: RkBoard,
    reg: PmuRegs,
    info: RockchipPmuInfo,
    dep_manager: dependency_manager::DependencyManager,
    /// QoS state storage for persistence across power cycles
    qos_states: alloc::collections::BTreeMap<PowerDomain, qos_control::QoSControl>,
}

impl RockchipPM {
    pub fn new(base: NonNull<u8>, board: RkBoard) -> Self {
        Self {
            _board: board,
            info: RockchipPmuInfo::new(board),
            reg: PmuRegs::new(base),
            dep_manager: dependency_manager::DependencyManager::new(),
            qos_states: alloc::collections::BTreeMap::new(),
        }
    }

    /// Check if QoS state exists for a domain
    ///
    /// # Arguments
    /// * `domain` - Power domain to check
    ///
    /// # Returns
    /// true if QoS state has been saved for this domain
    pub fn has_qos_state(&self, domain: PowerDomain) -> bool {
        self.qos_states.contains_key(&domain)
    }

    /// Clear QoS state for a domain
    ///
    /// # Arguments
    /// * `domain` - Power domain to clear state for
    pub fn clear_qos_state(&mut self, domain: PowerDomain) {
        self.qos_states.remove(&domain);
    }

    /// Clear all QoS states
    pub fn clear_all_qos_states(&mut self) {
        self.qos_states.clear();
    }

    /// Power on the specified power domain
    pub fn power_domain_on(&mut self, domain: PowerDomain) -> PowerResult<()> {
        let mut sequencer = PowerSequencer::new(&mut self.reg, &self.info);
        sequencer.power_on_sequence(domain)
    }

    /// Power off the specified power domain
    pub fn power_domain_off(&mut self, domain: PowerDomain) -> PowerResult<()> {
        let mut sequencer = PowerSequencer::new(&mut self.reg, &self.info);
        sequencer.power_off_sequence(domain)
    }

    /// Power on domain with dependency checking
    ///
    /// This method checks that all parent dependencies are satisfied before
    /// powering on the domain. If successful, the domain is marked as active.
    ///
    /// # Arguments
    /// * `domain` - Power domain to enable
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError::DependencyNotMet)` if parent dependencies not satisfied
    /// * `Err(PowerError)` for other power-on failures
    pub fn power_domain_on_with_deps(&mut self, domain: PowerDomain) -> PowerResult<()> {
        let domain_info = self
            .info
            .domains
            .get(&domain)
            .ok_or(PowerError::DomainNotFound)?;

        // Check dependencies
        self.dep_manager.can_power_on(domain, domain_info)?;

        // Execute power on
        let mut sequencer = PowerSequencer::new(&mut self.reg, &self.info);
        sequencer.power_on_sequence(domain)?;

        // Mark as active
        self.dep_manager.mark_powered_on(domain);

        Ok(())
    }

    /// Power off domain with dependency checking
    ///
    /// This method checks that all child dependencies are inactive before
    /// powering off the domain. If successful, the domain is marked as inactive.
    ///
    /// # Arguments
    /// * `domain` - Power domain to disable
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError::DependencyNotMet)` if child dependencies still active
    /// * `Err(PowerError)` for other power-off failures
    pub fn power_domain_off_with_deps(&mut self, domain: PowerDomain) -> PowerResult<()> {
        let domain_info = self
            .info
            .domains
            .get(&domain)
            .ok_or(PowerError::DomainNotFound)?;

        // Check dependencies
        self.dep_manager.can_power_off(domain, domain_info)?;

        // Execute power off
        let mut sequencer = PowerSequencer::new(&mut self.reg, &self.info);
        sequencer.power_off_sequence(domain)?;

        // Mark as inactive
        self.dep_manager.mark_powered_off(domain);

        Ok(())
    }

    /// Get currently active power domains
    ///
    /// Returns a reference to the set of domains that are currently powered on
    /// and tracked by the dependency manager.
    ///
    /// # Returns
    /// Reference to set of active power domains
    pub fn get_active_domains(&self) -> &alloc::collections::BTreeSet<PowerDomain> {
        self.dep_manager.get_active_domains()
    }

    /// Check if power domain is on
    pub fn is_domain_on(&self, domain: &PowerDomain) -> PowerResult<bool> {
        let domain_info = self
            .info
            .domains
            .get(domain)
            .ok_or(PowerError::DomainNotFound)?;

        if domain_info.repair_status_mask != 0 {
            // Use repair status register
            let val = self.reg.read_u32(self.info.repair_status_offset as usize);
            // 1'b1: power on, 1'b0: power off
            return Ok((val & (domain_info.repair_status_mask as u32)) != 0);
        }

        if domain_info.status_mask == 0 {
            // Check idle status only for domains without status mask
            return Ok(!self.is_domain_idle(domain)?);
        }

        let val = self.reg.read_u32(self.info.status_offset as usize);
        // 1'b0: power on, 1'b1: power off
        Ok((val & (domain_info.status_mask as u32)) == 0)
    }

    /// Check if power domain is idle
    pub fn is_domain_idle(&self, domain: &PowerDomain) -> PowerResult<bool> {
        let domain_info = self
            .info
            .domains
            .get(domain)
            .ok_or(PowerError::DomainNotFound)?;

        let val = self.reg.read_u32(self.info.idle_offset as usize);
        Ok((val & (domain_info.idle_mask as u32)) == (domain_info.idle_mask as u32))
    }
}

impl DriverGeneric for RockchipPM {
    fn open(&mut self) -> Result<(), rdif_base::KError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), rdif_base::KError> {
        Ok(())
    }
}
