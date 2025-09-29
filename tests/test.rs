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

    #[test]
    fn test_pm() {
        let reg = get_syscon_addr();
        let board = RkBoard::Rk3588;

        let mut pm = RockchipPM::new(reg, board);
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
}
