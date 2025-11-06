//! Bus idle control module for Rockchip power management
//!
//! This module handles bus idle request and acknowledgment, including:
//! - Bus idle request operations
//! - Idle acknowledgment verification
//! - Idle state verification
//! - Timeout handling for idle operations

use crate::{registers::PmuRegs, variants::RockchipDomainInfo, PowerError};
use mbarrier::mb;

/// Idle request timeout (in iterations)
const IDLE_REQUEST_TIMEOUT: u32 = 10000;
/// Idle acknowledgment timeout (in iterations)
const IDLE_ACK_TIMEOUT: u32 = 10000;

/// ACK register offset from idle register base
const IDLE_ACK_OFFSET: usize = 0x0c;

/// Bus idle controller
pub struct BusIdleControl {
    idle_offset: u32,
}

impl BusIdleControl {
    /// Create a new bus idle controller
    ///
    /// # Arguments
    /// * `idle_offset` - Base offset for idle control registers
    pub fn new(idle_offset: u32) -> Self {
        Self { idle_offset }
    }

    /// Request bus idle state
    ///
    /// # Arguments
    /// * `reg` - PMU register accessor
    /// * `domain_info` - Domain information containing idle control masks
    /// * `idle` - True to request idle, false to cancel idle request
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError)` if domain has no idle control or operation fails
    pub fn request_idle(
        &self,
        reg: &mut PmuRegs,
        domain_info: &RockchipDomainInfo,
        idle: bool,
    ) -> Result<(), PowerError> {
        // Check if domain has idle request control
        if domain_info.req_mask == 0 {
            return Ok(());
        }

        // Set idle request bit
        let current = reg.read_u32(self.idle_offset as usize);
        let new_value = if idle {
            current | (domain_info.req_mask as u32)
        } else {
            current & !(domain_info.req_mask as u32)
        };
        reg.write_u32(self.idle_offset as usize, new_value);

        mb();

        // Wait for acknowledgment
        self.wait_idle_ack(reg, domain_info, idle)?;

        // Verify idle state
        self.verify_idle_state(reg, domain_info, idle)?;

        Ok(())
    }

    /// Wait for idle acknowledgment
    ///
    /// # Arguments
    /// * `reg` - PMU register accessor
    /// * `domain_info` - Domain information containing ACK mask
    /// * `expected` - Expected ACK state
    ///
    /// # Returns
    /// * `Ok(())` if ACK received within timeout
    /// * `Err(PowerError::IdleAckTimeout)` if timeout occurs
    fn wait_idle_ack(
        &self,
        reg: &PmuRegs,
        domain_info: &RockchipDomainInfo,
        expected: bool,
    ) -> Result<(), PowerError> {
        // If no ACK mask, skip waiting
        if domain_info.ack_mask == 0 {
            return Ok(());
        }

        let ack_offset = self.idle_offset as usize + IDLE_ACK_OFFSET;

        for _ in 0..IDLE_ACK_TIMEOUT {
            let val = reg.read_u32(ack_offset);
            let ack_set = (val & (domain_info.ack_mask as u32)) == (domain_info.ack_mask as u32);

            if ack_set == expected {
                return Ok(());
            }
        }

        Err(PowerError::IdleAckTimeout)
    }

    /// Verify idle state matches expectation
    ///
    /// # Arguments
    /// * `reg` - PMU register accessor
    /// * `domain_info` - Domain information containing idle mask
    /// * `expected` - Expected idle state
    ///
    /// # Returns
    /// * `Ok(())` if state matches expectation within timeout
    /// * `Err(PowerError::IdleRequestTimeout)` if timeout occurs
    fn verify_idle_state(
        &self,
        reg: &PmuRegs,
        domain_info: &RockchipDomainInfo,
        expected: bool,
    ) -> Result<(), PowerError> {
        // If no idle mask, skip verification
        if domain_info.idle_mask == 0 {
            return Ok(());
        }

        for _ in 0..IDLE_REQUEST_TIMEOUT {
            let val = reg.read_u32(self.idle_offset as usize);
            let is_idle = (val & (domain_info.idle_mask as u32)) == (domain_info.idle_mask as u32);

            if is_idle == expected {
                return Ok(());
            }
        }

        Err(PowerError::IdleRequestTimeout)
    }
}
