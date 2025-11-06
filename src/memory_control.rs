//! Memory power control module for Rockchip power management
//!
//! This module handles memory power domain control, including:
//! - Memory power on/off operations
//! - Memory power state verification
//! - Timeout handling for memory operations

use crate::{registers::PmuRegs, variants::RockchipDomainInfo, PowerError};
use mbarrier::mb;

/// Memory power control timeout (in iterations)
const MEMORY_POWER_TIMEOUT: u32 = 10000;

/// Memory power controller
pub struct MemoryPowerControl {
    mem_pwr_offset: u32,
}

impl MemoryPowerControl {
    /// Create a new memory power controller
    ///
    /// # Arguments
    /// * `mem_pwr_offset` - Base offset for memory power control registers
    pub fn new(mem_pwr_offset: u32) -> Self {
        Self { mem_pwr_offset }
    }

    /// Set memory power state for a domain
    ///
    /// # Arguments
    /// * `reg` - PMU register accessor
    /// * `domain_info` - Domain information containing memory control masks
    /// * `power_on` - True to power on, false to power off
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError)` if domain has no memory control or operation fails
    pub fn set_memory_power(
        &self,
        reg: &mut PmuRegs,
        domain_info: &RockchipDomainInfo,
        power_on: bool,
    ) -> Result<(), PowerError> {
        // Check if domain has memory control
        if domain_info.mem_mask == 0 {
            return Ok(());
        }

        let mem_offset = self.mem_pwr_offset + domain_info.mem_offset;

        if domain_info.mem_w_mask != 0 {
            // Use write enable mask method
            let value = if power_on {
                domain_info.mem_w_mask
            } else {
                domain_info.mem_mask | domain_info.mem_w_mask
            };
            reg.write_u32(mem_offset as usize, value as u32);
        } else {
            // Use read-modify-write method
            let current = reg.read_u32(mem_offset as usize);
            let new_value = if power_on {
                current & !(domain_info.mem_mask as u32)
            } else {
                current | (domain_info.mem_mask as u32)
            };
            reg.write_u32(mem_offset as usize, new_value);
        }

        mb();

        Ok(())
    }

    /// Wait for memory power state to stabilize
    ///
    /// # Arguments
    /// * `reg` - PMU register accessor
    /// * `domain_info` - Domain information containing status masks
    /// * `expected_on` - Expected power state (true = on, false = off)
    /// * `repair_status_offset` - Offset for repair status register
    ///
    /// # Returns
    /// * `Ok(())` if state matches expectation within timeout
    /// * `Err(PowerError::MemoryPowerTimeout)` if timeout occurs
    pub fn wait_memory_stable(
        &self,
        reg: &PmuRegs,
        domain_info: &RockchipDomainInfo,
        expected_on: bool,
        repair_status_offset: u32,
    ) -> Result<(), PowerError> {
        // If no repair status mask, assume immediate success
        if domain_info.repair_status_mask == 0 {
            return Ok(());
        }

        for _ in 0..MEMORY_POWER_TIMEOUT {
            let val = reg.read_u32(repair_status_offset as usize);
            let is_on = (val & (domain_info.repair_status_mask as u32)) != 0;

            if is_on == expected_on {
                return Ok(());
            }
        }

        Err(PowerError::MemoryPowerTimeout)
    }
}
