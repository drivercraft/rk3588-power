#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;
#[macro_use]
extern crate log;

use rockchip_pm::*;

#[bare_test::tests]
mod tests {

    use core::ptr::NonNull;

    use super::*;
    use alloc::vec::Vec;
    use bare_test::{
        globals::{PlatformInfoKind, global_val},
        mem::{iomap, page_size},
    };

    // ========================================
    // Unit Tests for DependencyManager
    // ========================================

    #[test]
    fn test_dependency_manager_creation() {
        let manager = rockchip_pm::dependency_manager::DependencyManager::new();
        let active = manager.get_active_domains();
        assert!(
            active.is_empty(),
            "New manager should have no active domains"
        );
    }

    #[test]
    fn test_dependency_manager_mark_active() {
        let mut manager = rockchip_pm::dependency_manager::DependencyManager::new();
        let domain = RK3588::NPUTOP;

        manager.mark_powered_on(domain);
        let active = manager.get_active_domains();
        assert!(
            active.contains(&domain),
            "Domain should be marked as active"
        );
    }

    #[test]
    fn test_dependency_manager_mark_inactive() {
        let mut manager = rockchip_pm::dependency_manager::DependencyManager::new();
        let domain = RK3588::NPUTOP;

        manager.mark_powered_on(domain);
        manager.mark_powered_off(domain);
        let active = manager.get_active_domains();
        assert!(
            !active.contains(&domain),
            "Domain should be marked as inactive"
        );
    }

    #[test]
    fn test_dependency_manager_multiple_domains() {
        let mut manager = rockchip_pm::dependency_manager::DependencyManager::new();

        manager.mark_powered_on(RK3588::NPUTOP);
        manager.mark_powered_on(RK3588::NPU1);
        manager.mark_powered_on(RK3588::NPU2);

        let active = manager.get_active_domains();
        assert_eq!(active.len(), 3, "Should have 3 active domains");
        assert!(active.contains(&RK3588::NPUTOP));
        assert!(active.contains(&RK3588::NPU1));
        assert!(active.contains(&RK3588::NPU2));
    }

    // ========================================
    // Unit Tests for QoS State Management
    // ========================================

    #[test]
    fn test_qos_state_management() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Initially no QoS state
        assert!(
            !pm.has_qos_state(RK3588::GPU),
            "Should have no QoS state initially"
        );

        // Clear non-existent state should not panic
        pm.clear_qos_state(RK3588::GPU);

        // Clear all should not panic
        pm.clear_all_qos_states();
    }

    // ========================================
    // Integration Tests for Dependency Enforcement
    // ========================================

    #[test]
    fn test_parent_child_dependency_power_on_order() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Try to power on child (NPU1) without parent (NPUTOP) - should fail
        let result = pm.power_domain_on_with_deps(RK3588::NPU1);
        match result {
            Err(PowerError::DependencyNotMet) => {
                info!("✓ Correctly prevented child power-on without parent");
            }
            _ => panic!("Should have failed with DependencyNotMet error"),
        }

        // Power on parent first
        pm.power_domain_on_with_deps(RK3588::NPUTOP).unwrap();
        info!("✓ Parent (NPUTOP) powered on successfully");

        // Now child should power on successfully
        pm.power_domain_on_with_deps(RK3588::NPU1).unwrap();
        info!("✓ Child (NPU1) powered on successfully after parent");

        // Verify both domains are active
        let active = pm.get_active_domains();
        assert!(active.contains(&RK3588::NPUTOP), "Parent should be active");
        assert!(active.contains(&RK3588::NPU1), "Child should be active");
    }

    #[test]
    fn test_parent_child_dependency_power_off_order() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Power on parent and child
        pm.power_domain_on_with_deps(RK3588::VCODEC).unwrap();
        pm.power_domain_on_with_deps(RK3588::VENC0).unwrap();
        info!("✓ VCODEC and VENC0 powered on");

        // Try to power off parent with active child - should fail
        let result = pm.power_domain_off_with_deps(RK3588::VCODEC);
        match result {
            Err(PowerError::DependencyNotMet) => {
                info!("✓ Correctly prevented parent power-off with active child");
            }
            _ => panic!("Should have failed with DependencyNotMet error"),
        }

        // Power off child first
        pm.power_domain_off_with_deps(RK3588::VENC0).unwrap();
        info!("✓ Child (VENC0) powered off successfully");

        // Now parent should power off successfully
        pm.power_domain_off_with_deps(RK3588::VCODEC).unwrap();
        info!("✓ Parent (VCODEC) powered off successfully after child");

        // Verify both domains are inactive
        let active = pm.get_active_domains();
        assert!(
            !active.contains(&RK3588::VCODEC),
            "Parent should be inactive"
        );
        assert!(!active.contains(&RK3588::VENC0), "Child should be inactive");
    }

    #[test]
    fn test_multi_level_dependency() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test VOP → VO0/VO1 hierarchy
        // Power on in correct order: VOP → VO0
        pm.power_domain_on_with_deps(RK3588::VOP).unwrap();
        info!("✓ VOP (parent) powered on");

        pm.power_domain_on_with_deps(RK3588::VO0).unwrap();
        info!("✓ VO0 (child) powered on");

        pm.power_domain_on_with_deps(RK3588::VO1).unwrap();
        info!("✓ VO1 (child) powered on");

        // Verify all are active
        let active = pm.get_active_domains();
        assert_eq!(active.len(), 3, "Should have 3 active domains");

        // Power off in correct order: children first
        pm.power_domain_off_with_deps(RK3588::VO1).unwrap();
        pm.power_domain_off_with_deps(RK3588::VO0).unwrap();
        pm.power_domain_off_with_deps(RK3588::VOP).unwrap();
        info!("✓ All domains powered off in correct order");

        let active_after = pm.get_active_domains();
        assert!(active_after.is_empty(), "All domains should be inactive");
    }

    #[test]
    fn test_complex_vcodec_hierarchy() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // VCODEC has 4 children: VENC0, VENC1, RKVDEC0, RKVDEC1
        pm.power_domain_on_with_deps(RK3588::VCODEC).unwrap();
        pm.power_domain_on_with_deps(RK3588::VENC0).unwrap();
        pm.power_domain_on_with_deps(RK3588::VENC1).unwrap();
        pm.power_domain_on_with_deps(RK3588::RKVDEC0).unwrap();
        pm.power_domain_on_with_deps(RK3588::RKVDEC1).unwrap();
        info!("✓ VCODEC and all 4 children powered on");

        let active = pm.get_active_domains();
        assert_eq!(active.len(), 5, "Should have 5 active domains");

        // Try to power off parent - should fail
        let result = pm.power_domain_off_with_deps(RK3588::VCODEC);
        assert!(
            matches!(result, Err(PowerError::DependencyNotMet)),
            "Should fail to power off parent with active children"
        );

        // Power off all children
        pm.power_domain_off_with_deps(RK3588::VENC0).unwrap();
        pm.power_domain_off_with_deps(RK3588::VENC1).unwrap();
        pm.power_domain_off_with_deps(RK3588::RKVDEC0).unwrap();
        pm.power_domain_off_with_deps(RK3588::RKVDEC1).unwrap();
        info!("✓ All 4 children powered off");

        // Now parent can be powered off
        pm.power_domain_off_with_deps(RK3588::VCODEC).unwrap();
        info!("✓ Parent powered off after all children");
    }

    // ========================================
    // Integration Test: Original PM Test
    // ========================================

    #[test]
    fn test_pm() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;

        let mut pm = RockchipPM::new(reg, board);

        let npu = get_npu_info();

        pm.power_domain_on(RK3588::NPUTOP).unwrap();
        pm.power_domain_on(RK3588::NPU).unwrap();
        pm.power_domain_on(RK3588::NPU1).unwrap();
        pm.power_domain_on(RK3588::NPU2).unwrap();

        unsafe {
            let ptr = npu.base.as_ptr() as *mut u32;
            let version = ptr.read_volatile();
            info!("NPU Version: {version:#x}");
        }
    }

    struct NpuInfo {
        base: NonNull<u8>,
        _domains: Vec<PowerDomain>,
    }

    fn get_npu_info() -> NpuInfo {
        let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
        let fdt = fdt.get();

        let node = fdt
            .find_compatible(&["rockchip,rk3588-rknpu"])
            .next()
            .expect("Failed to find npu0 node");

        info!("Found node: {}", node.name());

        let regs = node.reg().unwrap().collect::<Vec<_>>();
        let start = regs[0].address as usize;
        let end = start + regs[0].size.unwrap_or(0);
        info!("NPU0 address range: 0x{:x} - 0x{:x}", start, end);
        let start = start & !(page_size() - 1);
        let end = (end + page_size() - 1) & !(page_size() - 1);
        info!("Aligned NPU0 address range: 0x{:x} - 0x{:x}", start, end);
        let base = iomap(start.into(), end - start);

        let mut domains = Vec::new();

        let pd_prop = node
            .find_property("power-domains")
            .expect("Failed to find power-domains property");
        let pd_ls = pd_prop.u32_list().collect::<Vec<_>>();
        for pd in pd_ls.chunks(2) {
            let phandle = pd[0];
            let pd = PowerDomain::new(pd[1] as usize);
            let pm_node = node
                .fdt()
                .get_node_by_phandle(phandle.into())
                .expect("Failed to find power domain node");
            info!("Found power domain node: {}", pm_node.name());

            domains.push(pd);
        }

        NpuInfo {
            base,
            _domains: domains,
        }
    }

    fn get_syscon_addr() -> NonNull<u8> {
        let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
        let fdt = fdt.get();

        let node = fdt
            .find_compatible(&["syscon"])
            .find(|n| n.name().contains("power-manage"))
            .expect("Failed to find syscon node");

        info!("Found node: {}", node.name());

        let regs = node.reg().unwrap().collect::<Vec<_>>();
        let start = regs[0].address as usize;
        let end = start + regs[0].size.unwrap_or(0);
        info!("Syscon address range: 0x{:x} - 0x{:x}", start, end);
        let start = start & !(page_size() - 1);
        let end = (end + page_size() - 1) & !(page_size() - 1);
        info!("Aligned Syscon address range: 0x{:x} - 0x{:x}", start, end);
        iomap(start.into(), end - start)
    }

    // ========================================
    // Additional Integration Tests
    // ========================================

    #[test]
    fn test_independent_domains() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test independent domains (no dependencies)
        // GPU, RGA30, AV1, FEC, RGA31, etc. have no parent/child relationships

        pm.power_domain_on_with_deps(RK3588::GPU).unwrap();
        info!("✓ GPU powered on (independent domain)");

        pm.power_domain_on_with_deps(RK3588::RGA30).unwrap();
        info!("✓ RGA30 powered on (independent domain)");

        // These can be powered off in any order
        pm.power_domain_off_with_deps(RK3588::GPU).unwrap();
        pm.power_domain_off_with_deps(RK3588::RGA30).unwrap();
        info!("✓ Independent domains powered off successfully");
    }

    #[test]
    fn test_get_active_domains_tracking() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Start with no active domains
        assert_eq!(
            pm.get_active_domains().len(),
            0,
            "Should start with 0 active domains"
        );

        // Power on some domains
        pm.power_domain_on_with_deps(RK3588::VI).unwrap();
        assert_eq!(
            pm.get_active_domains().len(),
            1,
            "Should have 1 active domain"
        );

        pm.power_domain_on_with_deps(RK3588::ISP1).unwrap();
        assert_eq!(
            pm.get_active_domains().len(),
            2,
            "Should have 2 active domains"
        );

        // Power off in correct order
        pm.power_domain_off_with_deps(RK3588::ISP1).unwrap();
        assert_eq!(
            pm.get_active_domains().len(),
            1,
            "Should have 1 active domain"
        );

        pm.power_domain_off_with_deps(RK3588::VI).unwrap();
        assert_eq!(
            pm.get_active_domains().len(),
            0,
            "Should have 0 active domains"
        );

        info!("✓ Active domain tracking works correctly");
    }

    #[test]
    fn test_power_domain_without_deps_api() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test the non-dependency API (power_domain_on/off)
        // These should work but won't enforce dependencies
        pm.power_domain_on(RK3588::AV1).unwrap();
        info!("✓ AV1 powered on via non-deps API");

        pm.power_domain_off(RK3588::AV1).unwrap();
        info!("✓ AV1 powered off via non-deps API");

        // Note: The non-deps API bypasses dependency checking
        // so it can power on children without parents, which may cause issues
    }

    #[test]
    fn test_qos_configured_domains() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test domains that have QoS configuration
        // GPU, NPU, VCODEC, VENC0, RKVDEC0, VOP, VI have QoS ports

        pm.power_domain_on_with_deps(RK3588::GPU).unwrap();
        info!("✓ GPU (with QoS) powered on");

        // QoS state should not exist yet (only created on power off)
        assert!(
            !pm.has_qos_state(RK3588::GPU),
            "QoS state should not exist before power off"
        );

        pm.power_domain_off_with_deps(RK3588::GPU).unwrap();
        info!("✓ GPU (with QoS) powered off");
    }

    #[test]
    fn test_error_handling_invalid_domain() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test with non-existent domain ID
        let invalid_domain = PowerDomain::new(9999);
        let result = pm.power_domain_on_with_deps(invalid_domain);

        match result {
            Err(PowerError::DomainNotFound) => {
                info!("✓ Correctly returned DomainNotFound for invalid domain");
            }
            _ => panic!("Should have returned DomainNotFound error"),
        }
    }

    #[test]
    fn test_vi_isp1_dependency() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test VI (Video Input) → ISP1 (Image Signal Processor) dependency

        // Try child first - should fail
        let result = pm.power_domain_on_with_deps(RK3588::ISP1);
        assert!(
            matches!(result, Err(PowerError::DependencyNotMet)),
            "ISP1 should fail without VI parent"
        );

        // Parent first
        pm.power_domain_on_with_deps(RK3588::VI).unwrap();
        pm.power_domain_on_with_deps(RK3588::ISP1).unwrap();
        info!("✓ VI → ISP1 dependency enforced correctly");

        // Power off in correct order
        pm.power_domain_off_with_deps(RK3588::ISP1).unwrap();
        pm.power_domain_off_with_deps(RK3588::VI).unwrap();
        info!("✓ VI → ISP1 powered off in correct order");
    }

    #[test]
    fn test_qos_state_clear_methods() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;
        let mut pm = RockchipPM::new(reg, board);

        // Test QoS state management methods

        // Clear on empty state should not panic
        pm.clear_qos_state(RK3588::GPU);
        pm.clear_all_qos_states();

        info!("✓ QoS state clear methods work correctly");
    }
}
