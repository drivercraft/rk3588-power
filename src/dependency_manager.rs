//! Dependency Manager Module
//!
//! This module manages power domain dependencies to ensure safe power sequencing.
//! It tracks active domains and validates dependency constraints before power operations.

use crate::{PowerDomain, PowerError, PowerResult, variants::RockchipDomainInfo};
use alloc::collections::BTreeSet;

/// Dependency manager for power domain relationships
///
/// Tracks currently active power domains and enforces dependency rules:
/// - Parent domains must be powered on before children
/// - Child domains must be powered off before parents
pub struct DependencyManager {
    /// Set of currently active (powered on) domains
    active_domains: BTreeSet<PowerDomain>,
}

impl DependencyManager {
    /// Create a new dependency manager
    ///
    /// Initializes with no active domains
    pub fn new() -> Self {
        Self {
            active_domains: BTreeSet::new(),
        }
    }

    /// Check if a domain can be safely powered on
    ///
    /// Validates that all parent dependencies are satisfied:
    /// - If domain has a parent, the parent must be active
    ///
    /// # Arguments
    /// * `domain` - The domain to check
    /// * `info` - Domain information containing dependency configuration
    ///
    /// # Returns
    /// * `Ok(())` if domain can be powered on
    /// * `Err(PowerError::DependencyNotMet)` if parent is not active
    pub fn can_power_on(&self, domain: PowerDomain, info: &RockchipDomainInfo) -> PowerResult<()> {
        // Optional: Check if domain is already active (idempotent check)
        // This allows the same domain to be powered on multiple times without error
        if self.active_domains.contains(&domain) {
            // Already active - this is not an error, just return success
            return Ok(());
        }

        // Check if domain has a parent dependency
        if let Some(ref dependency) = info.dependency {
            if let Some(parent) = dependency.parent {
                // Parent must be active
                if !self.active_domains.contains(&parent) {
                    return Err(PowerError::DependencyNotMet);
                }
            }
        }

        Ok(())
    }

    /// Check if a domain can be safely powered off
    ///
    /// Validates that no child dependencies are active:
    /// - All child domains must be powered off first
    ///
    /// # Arguments
    /// * `domain` - The domain to check
    /// * `info` - Domain information containing dependency configuration
    ///
    /// # Returns
    /// * `Ok(())` if domain can be powered off
    /// * `Err(PowerError::DependencyNotMet)` if any child is still active
    pub fn can_power_off(&self, domain: PowerDomain, info: &RockchipDomainInfo) -> PowerResult<()> {
        // Optional: Check if domain is already inactive (idempotent check)
        // This allows the same domain to be powered off multiple times without error
        if !self.active_domains.contains(&domain) {
            // Already inactive - this is not an error, just return success
            return Ok(());
        }

        // Check if domain has child dependencies
        if let Some(ref dependency) = info.dependency {
            for child in &dependency.children {
                // No child should be active
                if self.active_domains.contains(child) {
                    return Err(PowerError::DependencyNotMet);
                }
            }
        }

        Ok(())
    }

    /// Mark a domain as powered on
    ///
    /// Adds the domain to the active set after successful power-on
    ///
    /// # Arguments
    /// * `domain` - The domain that was powered on
    pub fn mark_powered_on(&mut self, domain: PowerDomain) {
        self.active_domains.insert(domain);
    }

    /// Mark a domain as powered off
    ///
    /// Removes the domain from the active set after successful power-off
    ///
    /// # Arguments
    /// * `domain` - The domain that was powered off
    pub fn mark_powered_off(&mut self, domain: PowerDomain) {
        self.active_domains.remove(&domain);
    }

    /// Check if a domain is currently active
    ///
    /// # Arguments
    /// * `domain` - The domain to check
    ///
    /// # Returns
    /// `true` if the domain is powered on, `false` otherwise
    pub fn is_active(&self, domain: &PowerDomain) -> bool {
        self.active_domains.contains(domain)
    }

    /// Get a reference to all currently active domains
    ///
    /// # Returns
    /// A reference to the set of active power domains
    pub fn get_active_domains(&self) -> &BTreeSet<PowerDomain> {
        &self.active_domains
    }
}

impl Default for DependencyManager {
    fn default() -> Self {
        Self::new()
    }
}
