#![no_std]
//! # RK3588 电源管理驱动
//!
//! 本库提供了针对 RK3588 系列 SoC 的电源管理功能。
//!

extern crate alloc;

use core::fmt;

// 模块声明
pub mod registers;
pub mod power;

// 重新导出核心模块
pub use registers::RK3588_PMU_BASE;
pub use power::Rk3588PowerManager;

/// RK3588 电源域定义
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerDomain {
    /// CPU 小核心集群 A
    CpuLittle = 0,
    /// CPU 大核心集群 B
    CpuBig = 1,
    /// GPU 图形处理器
    Gpu = 2,
    /// NPU 神经网络处理器
    Npu = 3,
    /// VPU 视频处理器
    Vpu = 4,
    /// RGA 图形加速器
    Rga = 5,
    /// VI 视频输入
    Vi = 6,
    /// VO 视频输出
    Vo = 7,
    /// 音频处理器
    Audio = 8,
    /// USB 控制器
    Usb = 9,
    /// PCIe 控制器
    Pcie = 10,
    /// SDMMC 存储控制器
    Sdmmc = 11,
}

/// 电源状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerState {
    /// 完全开启
    On,
    /// 部分关闭（保持关键功能）
    Standby,
    /// 完全关闭
    Off,
    /// 睡眠状态
    Sleep,
    /// 深度睡眠
    DeepSleep,
}

/// CPU 频率控制
#[derive(Debug, Clone, Copy)]
pub struct CpuFreq {
    pub freq_mhz: u32,
    pub voltage_mv: u32,
}

/// 预定义的 CPU 频率档位（适配 RK3588 全系列，包括 Orange Pi 5 Plus）
pub mod cpu_freqs {
    use super::CpuFreq;
    
    // 小核心 (Cortex-A55) 频率表
    pub const FREQ_408M: CpuFreq = CpuFreq { freq_mhz: 408, voltage_mv: 825 };
    pub const FREQ_600M: CpuFreq = CpuFreq { freq_mhz: 600, voltage_mv: 825 };
    pub const FREQ_816M: CpuFreq = CpuFreq { freq_mhz: 816, voltage_mv: 850 };
    pub const FREQ_1008M: CpuFreq = CpuFreq { freq_mhz: 1008, voltage_mv: 875 };
    pub const FREQ_1200M: CpuFreq = CpuFreq { freq_mhz: 1200, voltage_mv: 925 };
    pub const FREQ_1416M: CpuFreq = CpuFreq { freq_mhz: 1416, voltage_mv: 975 };
    pub const FREQ_1608M: CpuFreq = CpuFreq { freq_mhz: 1608, voltage_mv: 1025 };
    pub const FREQ_1800M: CpuFreq = CpuFreq { freq_mhz: 1800, voltage_mv: 1075 };
    
    // 大核心 (Cortex-A76) 频率表 - 支持更高频率（Orange Pi 5 Plus）
    pub const FREQ_2016M: CpuFreq = CpuFreq { freq_mhz: 2016, voltage_mv: 1125 };
    pub const FREQ_2208M: CpuFreq = CpuFreq { freq_mhz: 2208, voltage_mv: 1200 };
    pub const FREQ_2400M: CpuFreq = CpuFreq { freq_mhz: 2400, voltage_mv: 1300 };
}

/// 电源管理错误类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerError {
    /// 无效的电源域
    InvalidDomain,
    /// 操作超时
    Timeout,
    /// 硬件错误
    HardwareError,
    /// 不支持的操作
    Unsupported,
    /// 电压不稳定
    VoltageUnstable,
    /// 温度过高
    OverTemperature,
}

impl fmt::Display for PowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerError::InvalidDomain => write!(f, "Invalid power domain"),
            PowerError::Timeout => write!(f, "Operation timeout"),
            PowerError::HardwareError => write!(f, "Hardware error"),
            PowerError::Unsupported => write!(f, "Unsupported operation"),
            PowerError::VoltageUnstable => write!(f, "Voltage unstable"),
            PowerError::OverTemperature => write!(f, "Over temperature"),
        }
    }
}

/// 电源管理结果类型
pub type PowerResult<T> = Result<T, PowerError>;

/// 底层寄存器访问 trait
pub trait RegisterAccess {
    /// 读取 32 位寄存器
    unsafe fn read_reg(&self, addr: u32) -> u32;
    /// 写入 32 位寄存器
    unsafe fn write_reg(&self, addr: u32, value: u32);
    /// 修改寄存器位域
    unsafe fn modify_reg(&self, addr: u32, mask: u32, value: u32) {
        unsafe {
            let current = self.read_reg(addr);
            self.write_reg(addr, (current & !mask) | (value & mask));
        }
    }
}

/// 默认的 MMIO 寄存器访问实现
pub struct MmioRegisterAccess;

impl RegisterAccess for MmioRegisterAccess {
    unsafe fn read_reg(&self, addr: u32) -> u32 {
        unsafe { core::ptr::read_volatile(addr as *const u32) }
    }
    
    unsafe fn write_reg(&self, addr: u32, value: u32) {
        unsafe { core::ptr::write_volatile(addr as *mut u32, value); }
    }
}

/// 电源状态信息结构
#[derive(Debug, Clone)]
pub struct PowerStatus {
    pub domains: [PowerState; 12],
    pub cpu_little_freq: CpuFreq,
    pub cpu_big_freq: CpuFreq,
    pub power_consumption: f32,  // mW
    pub temperature: f32,        // °C
}

impl fmt::Display for PowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== RK3588 Power Status ===")?;
        writeln!(f, "CPU Little: {} MHz @ {} mV", 
                self.cpu_little_freq.freq_mhz, self.cpu_little_freq.voltage_mv)?;
        writeln!(f, "CPU Big: {} MHz @ {} mV", 
                self.cpu_big_freq.freq_mhz, self.cpu_big_freq.voltage_mv)?;
        writeln!(f, "Power Consumption: {:.1} mW", self.power_consumption)?;
        writeln!(f, "Temperature: {:.1}°C", self.temperature)?;
        
        writeln!(f, "\nPower Domains:")?;
        let domain_names = [
            "CpuLittle", "CpuBig", "Gpu", "Npu", "Vpu", "Rga",
            "Vi", "Vo", "Audio", "Usb", "Pcie", "Sdmmc"
        ];
        
        for (i, &name) in domain_names.iter().enumerate() {
            if i < self.domains.len() {
                writeln!(f, "  {}: {:?}", name, self.domains[i])?;
            }
        }
        
        Ok(())
    }
}

/// 创建默认的 RK3588 电源管理器实例
pub fn create_default_power_manager() -> Rk3588PowerManager<MmioRegisterAccess> {
    Rk3588PowerManager::new(MmioRegisterAccess)
}

/// 检测当前运行的平台类型
/// 返回值：("SoC类型", "开发板类型")
/// 注意：这是一个简化实现，实际项目中应该读取设备树或其他系统信息
pub fn detect_platform() -> (&'static str, &'static str) {
    // 这里可以添加实际的设备检测逻辑
    // 比如读取 /proc/device-tree/compatible 或 CPUID 等
    ("RK3588", "Generic RK3588 Board")
}

/// 检查当前平台是否支持扩展电源域
/// Orange Pi 5 Plus 等使用完整 RK3588 的平台支持更多电源域
pub fn supports_extended_power_domains() -> bool {
    let (soc_type, _) = detect_platform();
    matches!(soc_type, "RK3588") // RK3588S 可能支持更少的电源域
}