#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

use bare_test::println;
use log::{debug, info};
use rk3588_power::*;

/// 模拟寄存器访问（用于测试）
struct MockRegisterAccess {
    power_status: u32,
}

impl MockRegisterAccess {
    fn new() -> Self {
        Self {
            power_status: 0xFFFF_FFFF, // 所有域都开启
        }
    }
}

impl RegisterAccess for MockRegisterAccess {
    unsafe fn read_reg(&self, addr: u32) -> u32 {
        match addr - RK3588_PMU_BASE {
            0x0078 => self.power_status, // PMU_POWER_ST
            0x0020 => 0x0000_0000,       // PMU_PWRDN_ST - 没有域被关闭
            _ => 0x0000_0000,
        }
    }

    unsafe fn write_reg(&self, addr: u32, value: u32) {
        debug!("Mock write to 0x{:08x}: 0x{:08x}", addr, value);
    }
}

#[bare_test::tests]
mod tests {
    use super::*;
    use bare_test::println;
    use log::info;

    #[test]
    fn test_power_manager_creation() {
        info!("Testing RK3588 power manager creation");

        let power_manager = create_default_power_manager();

        // 验证初始状态
        assert_eq!(
            power_manager.get_power_domain_state(PowerDomain::CpuLittle),
            PowerState::On
        );

        assert_eq!(
            power_manager.get_power_domain_state(PowerDomain::CpuBig),
            PowerState::On
        );

        println!("Power manager created successfully!");
    }

    #[test]
    fn test_cpu_frequency_control() {
        info!("Testing CPU frequency control");

        let mut power_manager = Rk3588PowerManager::new(MockRegisterAccess::new());

        // 测试设置 CPU 小核频率
        let result = power_manager.set_cpu_frequency(PowerDomain::CpuLittle, cpu_freqs::FREQ_816M);

        assert!(result.is_ok());

        // 验证频率设置
        let freq = power_manager
            .get_cpu_frequency(PowerDomain::CpuLittle)
            .unwrap();
        assert_eq!(freq.freq_mhz, 816);
        assert_eq!(freq.voltage_mv, 950);

        // 测试设置 CPU 大核频率
        let result = power_manager.set_cpu_frequency(PowerDomain::CpuBig, cpu_freqs::FREQ_2208M);

        assert!(result.is_ok());

        let freq = power_manager
            .get_cpu_frequency(PowerDomain::CpuBig)
            .unwrap();
        assert_eq!(freq.freq_mhz, 2208);
        assert_eq!(freq.voltage_mv, 1350);

        println!("CPU frequency control test passed!");
    }

    #[test]
    fn test_power_consumption_calculation() {
        info!("Testing power consumption calculation");

        let power_manager = Rk3588PowerManager::new(MockRegisterAccess::new());

        let power_consumption = power_manager.get_power_consumption().unwrap();

        // 基本检查：功耗应该大于 0
        assert!(power_consumption > 0.0);

        // 正常情况下，功耗应该在合理范围内（500mW 到 10000mW）
        assert!(power_consumption > 500.0 && power_consumption < 10000.0);

        println!("Power consumption: {:.1} mW", power_consumption);
        println!("Power consumption calculation test passed!");
    }

    #[test]
    fn test_temperature_monitoring() {
        info!("Testing temperature monitoring");

        let power_manager = Rk3588PowerManager::new(MockRegisterAccess::new());

        let temperature = power_manager.get_temperature().unwrap();

        // 温度应该在合理范围内
        assert!(temperature > 20.0 && temperature < 100.0);

        println!("Current temperature: {:.1}°C", temperature);
        println!("Temperature monitoring test passed!");
    }

    #[test]
    fn test_cpu_frequency_presets() {
        info!("Testing CPU frequency presets");

        // 测试预定义频率
        let freq_408m = cpu_freqs::FREQ_408M;
        assert_eq!(freq_408m.freq_mhz, 408);
        assert_eq!(freq_408m.voltage_mv, 900);

        let freq_2400m = cpu_freqs::FREQ_2400M;
        assert_eq!(freq_2400m.freq_mhz, 2400);
        assert_eq!(freq_2400m.voltage_mv, 1400);

        println!("CPU frequency presets test passed!");
    }

    #[test]
    fn test_power_domain_enum() {
        info!("Testing power domain enumeration");

        // 测试电源域枚举
        assert_eq!(PowerDomain::CpuLittle as u32, 0);
        assert_eq!(PowerDomain::CpuBig as u32, 1);
        assert_eq!(PowerDomain::Gpu as u32, 2);
        assert_eq!(PowerDomain::Npu as u32, 3);
        assert_eq!(PowerDomain::Sdmmc as u32, 11);

        println!("Power domain enumeration test passed!");
    }

    #[test]
    fn test_error_handling() {
        info!("Testing error handling");

        let power_manager = Rk3588PowerManager::new(MockRegisterAccess::new());

        // 测试无效的电源域操作
        let result = power_manager.get_cpu_frequency(PowerDomain::Gpu);
        assert_eq!(result.unwrap_err(), PowerError::InvalidDomain);

        println!("Error handling test passed!");
    }

    #[test]
    fn basic_functionality_test() {
        info!("Running basic functionality test");

        let mut power_manager = Rk3588PowerManager::new(MockRegisterAccess::new());

        // 初始化测试
        let init_result = power_manager.init();
        assert!(init_result.is_ok());

        // 获取初始状态
        let status = power_manager.get_power_status().unwrap();
        assert!(status.power_consumption > 0.0);
        assert!(status.temperature > 0.0);

        println!("Basic functionality test passed!");
        println!("RK3588 Power Driver is working correctly!");
    }
}
