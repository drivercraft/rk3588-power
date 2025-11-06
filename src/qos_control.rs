//! QoS (Quality of Service) Control Module
//!
//! This module provides functionality to save and restore QoS registers
//! for Rockchip power domains. QoS settings control bus priority, bandwidth,
//! and other performance parameters that need to be preserved across
//! power domain transitions.

use crate::{PowerError, PowerResult};
use alloc::vec::Vec;
use core::ptr::NonNull;

/// QoS register offsets
const QOS_PRIORITY: usize = 0x08;
const QOS_MODE: usize = 0x0c;
const QOS_BANDWIDTH: usize = 0x10;
const QOS_SATURATION: usize = 0x14;
const QOS_EXTCONTROL: usize = 0x18;

/// Number of QoS registers to save/restore
const MAX_QOS_REGS: usize = 5;

/// Maximum number of QoS ports per domain
const MAX_QOS_PORTS: usize = 8;

/// QoS Control structure for managing QoS register save/restore
///
/// Each power domain may have multiple QoS ports that need their
/// configuration saved before power off and restored after power on.
pub struct QoSControl {
    /// Base addresses of QoS ports for this domain
    qos_bases: Vec<NonNull<u8>>,

    /// Saved QoS register values [register_index][port_index]
    /// - Index 0: QOS_PRIORITY
    /// - Index 1: QOS_MODE
    /// - Index 2: QOS_BANDWIDTH
    /// - Index 3: QOS_SATURATION
    /// - Index 4: QOS_EXTCONTROL
    saved_regs: [[u32; MAX_QOS_PORTS]; MAX_QOS_REGS],

    /// Flag indicating whether QoS registers have been saved
    is_saved: bool,
}

// SAFETY: QoSControl is used in single-threaded embedded environment
// The NonNull pointers are memory-mapped I/O addresses that are safe to send
unsafe impl Send for QoSControl {}

impl QoSControl {
    /// Create a new QoSControl instance
    ///
    /// # Arguments
    /// * `qos_bases` - Vector of base addresses for QoS ports
    ///
    /// # Returns
    /// A new QoSControl instance, or None if no QoS ports are configured
    pub fn new(qos_bases: Vec<NonNull<u8>>) -> Option<Self> {
        if qos_bases.is_empty() {
            return None;
        }

        if qos_bases.len() > MAX_QOS_PORTS {
            return None;
        }

        Some(Self {
            qos_bases,
            saved_regs: [[0; MAX_QOS_PORTS]; MAX_QOS_REGS],
            is_saved: false,
        })
    }

    /// Save QoS registers for all ports
    ///
    /// Reads and stores the current values of all QoS registers.
    /// This should be called before powering off a domain.
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError::QoSError)` if save fails
    pub fn save(&mut self) -> PowerResult<()> {
        for (port_idx, base) in self.qos_bases.iter().enumerate() {
            // Save QOS_PRIORITY
            self.saved_regs[0][port_idx] =
                unsafe { core::ptr::read_volatile(base.as_ptr().add(QOS_PRIORITY) as *const u32) };

            // Save QOS_MODE
            self.saved_regs[1][port_idx] =
                unsafe { core::ptr::read_volatile(base.as_ptr().add(QOS_MODE) as *const u32) };

            // Save QOS_BANDWIDTH
            self.saved_regs[2][port_idx] =
                unsafe { core::ptr::read_volatile(base.as_ptr().add(QOS_BANDWIDTH) as *const u32) };

            // Save QOS_SATURATION
            self.saved_regs[3][port_idx] = unsafe {
                core::ptr::read_volatile(base.as_ptr().add(QOS_SATURATION) as *const u32)
            };

            // Save QOS_EXTCONTROL
            self.saved_regs[4][port_idx] = unsafe {
                core::ptr::read_volatile(base.as_ptr().add(QOS_EXTCONTROL) as *const u32)
            };
        }

        self.is_saved = true;
        Ok(())
    }

    /// Restore QoS registers for all ports
    ///
    /// Writes back the previously saved QoS register values.
    /// This should be called after powering on a domain.
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(PowerError::QoSError)` if restore fails or registers weren't saved
    pub fn restore(&self) -> PowerResult<()> {
        if !self.is_saved {
            return Err(PowerError::QoSError);
        }

        for (port_idx, base) in self.qos_bases.iter().enumerate() {
            // Restore QOS_PRIORITY
            unsafe {
                core::ptr::write_volatile(
                    base.as_ptr().add(QOS_PRIORITY) as *mut u32,
                    self.saved_regs[0][port_idx],
                );
            }

            // Restore QOS_MODE
            unsafe {
                core::ptr::write_volatile(
                    base.as_ptr().add(QOS_MODE) as *mut u32,
                    self.saved_regs[1][port_idx],
                );
            }

            // Restore QOS_BANDWIDTH
            unsafe {
                core::ptr::write_volatile(
                    base.as_ptr().add(QOS_BANDWIDTH) as *mut u32,
                    self.saved_regs[2][port_idx],
                );
            }

            // Restore QOS_SATURATION
            unsafe {
                core::ptr::write_volatile(
                    base.as_ptr().add(QOS_SATURATION) as *mut u32,
                    self.saved_regs[3][port_idx],
                );
            }

            // Restore QOS_EXTCONTROL
            unsafe {
                core::ptr::write_volatile(
                    base.as_ptr().add(QOS_EXTCONTROL) as *mut u32,
                    self.saved_regs[4][port_idx],
                );
            }
        }

        Ok(())
    }

    /// Check if QoS registers have been saved
    #[allow(unused)]
    pub fn is_saved(&self) -> bool {
        self.is_saved
    }

    /// Get the number of QoS ports
    #[allow(unused)]
    pub fn num_ports(&self) -> usize {
        self.qos_bases.len()
    }
}
