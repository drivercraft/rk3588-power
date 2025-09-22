#![no_std]
#![no_main]

extern crate alloc;

use log::{info, warn};
use rk3588_power::*;

/// 演示如何使用 RK3588 电源管理驱动
fn main() {
    // 初始化日志系统
    env_logger::init();

    info!("=== RK3588 电源管理驱动演示 ===");

    // 创建电源管理器实例
    let mut power_manager = create_default_power_manager();

    // 初始化电源管理系统
    match power_manager.init() {
        Ok(_) => info!("电源管理系统初始化成功"),
        Err(e) => {
            warn!("电源管理系统初始化失败: {}", e);
            return;
        }
    }

    // 获取初始状态
    demo_power_status(&mut power_manager);

    // 演示 CPU 频率控制
    demo_cpu_frequency_control(&mut power_manager);

    // 演示电源域控制
    demo_power_domain_control(&mut power_manager);

    // 演示热管理
    demo_thermal_management(&mut power_manager);

    // 演示睡眠模式
    demo_sleep_modes(&mut power_manager);

    info!("=== 演示完成 ===");
}

/// 演示电源状态查询
fn demo_power_status(power_manager: &mut Rk3588PowerManager<MmioRegisterAccess>) {
    info!("\n--- 电源状态查询演示 ---");

    match power_manager.get_power_status() {
        Ok(status) => {
            info!("当前电源状态:");
            info!(
                "  CPU 小核: {} MHz @ {} mV",
                status.cpu_little_freq.freq_mhz, status.cpu_little_freq.voltage_mv
            );
            info!(
                "  CPU 大核: {} MHz @ {} mV",
                status.cpu_big_freq.freq_mhz, status.cpu_big_freq.voltage_mv
            );
            info!("  功耗: {:.1} mW", status.power_consumption);
            info!("  温度: {:.1}°C", status.temperature);
        }
        Err(e) => warn!("获取电源状态失败: {}", e),
    }
}

/// 演示 CPU 频率控制
fn demo_cpu_frequency_control(power_manager: &mut Rk3588PowerManager<MmioRegisterAccess>) {
    info!("\n--- CPU 频率控制演示 ---");

    // 设置不同的频率档位
    let freq_configs = [
        (PowerDomain::CpuLittle, cpu_freqs::FREQ_408M, "节能模式"),
        (PowerDomain::CpuLittle, cpu_freqs::FREQ_1200M, "平衡模式"),
        (PowerDomain::CpuBig, cpu_freqs::FREQ_1800M, "性能模式"),
        (PowerDomain::CpuBig, cpu_freqs::FREQ_2400M, "高性能模式"),
    ];

    for (domain, freq, mode) in &freq_configs {
        match power_manager.set_cpu_frequency(*domain, *freq) {
            Ok(_) => {
                info!(
                    "设置 {:?} 为 {} ({} MHz @ {} mV)",
                    domain, mode, freq.freq_mhz, freq.voltage_mv
                );

                // 验证设置
                if let Ok(current_freq) = power_manager.get_cpu_frequency(*domain) {
                    info!(
                        "  验证: {} MHz @ {} mV",
                        current_freq.freq_mhz, current_freq.voltage_mv
                    );
                }
            }
            Err(e) => warn!("设置 {:?} 频率失败: {}", domain, e),
        }
    }
}

/// 演示电源域控制
fn demo_power_domain_control(power_manager: &mut Rk3588PowerManager<MmioRegisterAccess>) {
    info!("\n--- 电源域控制演示 ---");

    let domains_to_test = [
        (PowerDomain::Gpu, "GPU 图形处理器"),
        (PowerDomain::Npu, "NPU 神经网络处理器"),
        (PowerDomain::Vpu, "VPU 视频处理器"),
    ];

    for (domain, description) in &domains_to_test {
        info!("测试 {} 电源控制", description);

        // 关闭电源域
        match power_manager.control_power_domain(*domain, PowerState::Off) {
            Ok(_) => info!("  {} 已关闭", description),
            Err(e) => warn!("  关闭 {} 失败: {}", description, e),
        }

        // 设置为待机模式
        match power_manager.control_power_domain(*domain, PowerState::Standby) {
            Ok(_) => info!("  {} 进入待机模式", description),
            Err(e) => warn!("  {} 进入待机模式失败: {}", description, e),
        }

        // 重新开启
        match power_manager.control_power_domain(*domain, PowerState::On) {
            Ok(_) => info!("  {} 已开启", description),
            Err(e) => warn!("  开启 {} 失败: {}", description, e),
        }

        // 检查当前状态
        let state = power_manager.get_power_domain_state(*domain);
        info!("  {} 当前状态: {:?}", description, state);
    }
}

/// 演示热管理
fn demo_thermal_management(power_manager: &mut Rk3588PowerManager<MmioRegisterAccess>) {
    info!("\n--- 热管理演示 ---");

    // 获取当前温度
    match power_manager.get_temperature() {
        Ok(temp) => info!("当前系统温度: {:.1}°C", temp),
        Err(e) => warn!("获取温度失败: {}", e),
    }

    // 执行热管理检查
    match power_manager.thermal_management() {
        Ok(_) => info!("热管理检查完成"),
        Err(PowerError::OverTemperature) => {
            warn!("系统温度过高，已启动保护措施");
        }
        Err(e) => warn!("热管理检查失败: {}", e),
    }

    // 再次获取温度
    if let Ok(temp) = power_manager.get_temperature() {
        info!("热管理后温度: {:.1}°C", temp);
    }
}

/// 演示睡眠模式
fn demo_sleep_modes(power_manager: &mut Rk3588PowerManager<MmioRegisterAccess>) {
    info!("\n--- 睡眠模式演示 ---");

    // 获取进入睡眠前的状态
    if let Ok(status_before) = power_manager.get_power_status() {
        info!("睡眠前功耗: {:.1} mW", status_before.power_consumption);
    }

    // 模拟进入浅睡眠
    info!("准备进入浅睡眠模式...");
    match power_manager.enter_sleep_mode(PowerState::Sleep) {
        Ok(_) => info!("系统进入浅睡眠模式"),
        Err(e) => warn!("进入浅睡眠失败: {}", e),
    }

    // 模拟进入深度睡眠
    info!("准备进入深度睡眠模式...");
    match power_manager.enter_sleep_mode(PowerState::DeepSleep) {
        Ok(_) => {
            info!("系统进入深度睡眠模式");
            info!("非必要组件已关闭以降低功耗");
        }
        Err(e) => warn!("进入深度睡眠失败: {}", e),
    }

    // 检查睡眠后的功耗
    if let Ok(status_after) = power_manager.get_power_status() {
        info!("睡眠后功耗: {:.1} mW", status_after.power_consumption);
    }
}

/// 错误处理演示
fn demo_error_handling() {
    info!("\n--- 错误处理演示 ---");

    let power_manager = create_default_power_manager();

    // 尝试对无效域进行 CPU 频率操作
    match power_manager.get_cpu_frequency(PowerDomain::Gpu) {
        Ok(_) => info!("不应该到达这里"),
        Err(PowerError::InvalidDomain) => info!("正确捕获了无效域错误"),
        Err(e) => warn!("意外的错误类型: {}", e),
    }

    // 展示不同的错误类型
    let error_types = [
        PowerError::InvalidDomain,
        PowerError::Timeout,
        PowerError::HardwareError,
        PowerError::Unsupported,
        PowerError::VoltageUnstable,
        PowerError::OverTemperature,
    ];

    for error in &error_types {
        info!("错误类型: {}", error);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
