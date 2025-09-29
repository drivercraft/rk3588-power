//! RK3588 PMU 寄存器定义
//!
//! 使用 tock-registers 库提供的宏来定义 RK3588 PMU（电源管理单元）的寄存器结构
//! 包含详细的位域定义，增强代码可读性和类型安全性

use core::ptr::NonNull;
use tock_registers::{
    interfaces::*,
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

/// RK3588 PMU（电源管理单元）基地址
/// 兼容 RK3588 和 RK3588S，适用于 Orange Pi 5 Plus 等开发板
pub const RK3588_PMU_BASE: u32 = 0xFD8D_0000;

/// 功耗优化配置常量
pub mod power_optimization {
    /// 启用所有 NOC 自动功能（时钟门控和睡眠）
    pub const NOC_ALL_AUTO_FEATURES_ENABLED: u32 = 0xFFFF_FFFF;
    /// 禁用所有 NOC 自动功能
    pub const NOC_ALL_AUTO_FEATURES_DISABLED: u32 = 0x0000_0000;
    /// 启用所有总线空闲请求
    pub const BUS_IDLE_ALL_ENABLED: u32 = 0xFFFF_FFFF;
    /// 禁用所有总线空闲请求
    pub const BUS_IDLE_ALL_DISABLED: u32 = 0x0000_0000;
}

/// 唤醒配置常量
pub mod wakeup_config {
    /// GPIO 唤醒配置
    pub const GPIO_WAKEUP_PIN_0_1: u8 = 0x03; // GPIO 0 和 1 唤醒使能
    pub const GPIO_WAKEUP_PIN_0_3: u8 = 0x0F; // GPIO 0-3 唤醒使能
    pub const GPIO_WAKEUP_ALL_DISABLED: u8 = 0x00; // 所有 GPIO 唤醒禁用
}

register_bitfields! {
    u32,

    /// 唤醒配置寄存器 0 (0x0000)
    pub WAKEUP_CFG0 [
        /// GPIO 唤醒使能
        GPIO_WAKEUP_EN OFFSET(0) NUMBITS(8) [],
        /// RTC 唤醒使能
        RTC_WAKEUP_EN OFFSET(8) NUMBITS(1) [],
        /// 网络唤醒使能
        NET_WAKEUP_EN OFFSET(9) NUMBITS(1) [],
        /// USB 唤醒使能
        USB_WAKEUP_EN OFFSET(10) NUMBITS(1) [],
        /// 保留位
        RESERVED OFFSET(11) NUMBITS(21) []
    ],

    /// 电源关闭控制寄存器 (0x0018)
    pub PWRDN_CON [
        /// CPU 小核心集群电源控制
        CPU_LITTLE_PWRDN OFFSET(0) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// CPU 大核心集群电源控制
        CPU_BIG_PWRDN OFFSET(1) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// GPU 电源控制
        GPU_PWRDN OFFSET(2) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// NPU 电源控制
        NPU_PWRDN OFFSET(3) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// VPU 电源控制
        VPU_PWRDN OFFSET(4) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// RGA 电源控制
        RGA_PWRDN OFFSET(5) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// VI 电源控制
        VI_PWRDN OFFSET(6) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// VO 电源控制
        VO_PWRDN OFFSET(7) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// 音频电源控制
        AUDIO_PWRDN OFFSET(8) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// USB 电源控制
        USB_PWRDN OFFSET(9) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// PCIe 电源控制
        PCIE_PWRDN OFFSET(10) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// SDMMC 电源控制
        SDMMC_PWRDN OFFSET(11) NUMBITS(1) [
            PowerOn = 0,
            PowerOff = 1
        ],
        /// 保留位
        RESERVED OFFSET(12) NUMBITS(20) []
    ],

    /// 电源关闭状态寄存器 (0x0020)
    pub PWRDN_ST [
        /// CPU 小核心集群电源状态
        CPU_LITTLE_ST OFFSET(0) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// CPU 大核心集群电源状态
        CPU_BIG_ST OFFSET(1) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// GPU 电源状态
        GPU_ST OFFSET(2) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// NPU 电源状态
        NPU_ST OFFSET(3) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// VPU 电源状态
        VPU_ST OFFSET(4) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// RGA 电源状态
        RGA_ST OFFSET(5) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// VI 电源状态
        VI_ST OFFSET(6) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// VO 电源状态
        VO_ST OFFSET(7) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// 音频电源状态
        AUDIO_ST OFFSET(8) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// USB 电源状态
        USB_ST OFFSET(9) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// PCIe 电源状态
        PCIE_ST OFFSET(10) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// SDMMC 电源状态
        SDMMC_ST OFFSET(11) NUMBITS(1) [
            PoweredOff = 0,
            PoweredOn = 1
        ],
        /// 保留位
        RESERVED OFFSET(12) NUMBITS(20) []
    ]
}

register_structs! {
    /// RK3588 PMU 寄存器结构
    #[allow(non_snake_case)]
    pub Rk3588PmuRegs {
        /// 唤醒配置寄存器 0
        (0x0000 => pub WAKEUP_CFG0: ReadWrite<u32, WAKEUP_CFG0::Register>),
        /// 唤醒配置寄存器 1
        (0x0004 => pub WAKEUP_CFG1: ReadWrite<u32>),
        /// 保留区域 1
        (0x0008 => _reserved1),
        /// 电源关闭控制寄存器
        (0x0018 => pub PWRDN_CON: ReadWrite<u32, PWRDN_CON::Register>),
        /// 保留区域 2
        (0x001C => _reserved2),
        /// 电源关闭状态寄存器
        (0x0020 => pub PWRDN_ST: ReadOnly<u32, PWRDN_ST::Register>),
        /// 电源模式控制寄存器
        (0x0024 => pub PWRMODE_CON: ReadWrite<u32>),
        /// 软件控制寄存器
        (0x0028 => pub SFT_CON: ReadWrite<u32>),
        /// 中断控制寄存器
        (0x002C => pub INT_CON: ReadWrite<u32>),
        /// 中断状态寄存器
        (0x0030 => pub INT_ST: ReadWrite<u32>),
        /// GPIO0 正沿中断控制寄存器
        (0x0034 => pub GPIO0_POS_INT_CON: ReadWrite<u32>),
        /// GPIO0 负沿中断控制寄存器
        (0x0038 => pub GPIO0_NEG_INT_CON: ReadWrite<u32>),
        /// GPIO1 正沿中断控制寄存器
        (0x003C => pub GPIO1_POS_INT_CON: ReadWrite<u32>),
        /// GPIO1 负沿中断控制寄存器
        (0x0040 => pub GPIO1_NEG_INT_CON: ReadWrite<u32>),
        /// 电源关闭中断使能寄存器
        (0x0044 => pub PWRDN_INTEN: ReadWrite<u32>),
        /// 电源关闭状态寄存器
        (0x0048 => pub PWRDN_STATUS: ReadOnly<u32>),
        /// 唤醒状态寄存器
        (0x004C => pub WAKEUP_STATUS: ReadOnly<u32>),
        /// 总线清除寄存器
        (0x0050 => pub BUS_CLR: WriteOnly<u32>),
        /// 总线空闲请求寄存器
        (0x0054 => pub BUS_IDLE_REQ: ReadWrite<u32>),
        /// 总线空闲状态寄存器
        (0x0058 => pub BUS_IDLE_ST: ReadOnly<u32>),
        /// 总线空闲应答寄存器
        (0x005C => pub BUS_IDLE_ACK: ReadOnly<u32>),
        /// CCI500 控制寄存器
        (0x0060 => pub CCI500_CON: ReadWrite<u32>),
        /// ADB400 控制寄存器
        (0x0064 => pub ADB400_CON: ReadWrite<u32>),
        /// ADB400 状态寄存器
        (0x0068 => pub ADB400_ST: ReadOnly<u32>),
        /// 保留区域 3
        (0x006C => _reserved3),
        /// 电源状态寄存器
        (0x0078 => pub POWER_ST: ReadOnly<u32>),
        /// 核心电源状态寄存器
        (0x007C => pub CORE_PWR_ST: ReadOnly<u32>),
        /// 振荡器计数寄存器
        (0x0080 => pub OSC_CNT: ReadWrite<u32>),
        /// PLL 锁定计数寄存器
        (0x0084 => pub PLLLOCK_CNT: ReadWrite<u32>),
        /// PLL 复位计数寄存器
        (0x0088 => pub PLLRST_CNT: ReadWrite<u32>),
        /// 稳定计数寄存器
        (0x008C => pub STABLE_CNT: ReadWrite<u32>),
        /// DDR IO 上电计数寄存器
        (0x0090 => pub DDRIO_PWRON_CNT: ReadWrite<u32>),
        /// 唤醒复位清除计数寄存器
        (0x0094 => pub WAKEUP_RST_CLR_CNT: ReadWrite<u32>),
        /// DDR 自刷新状态寄存器
        (0x0098 => pub DDR_SREF_ST: ReadOnly<u32>),
        /// SCU L 关机计数寄存器
        (0x009C => pub SCU_L_PWRDN_CNT: ReadWrite<u32>),
        /// SCU L 上电计数寄存器
        (0x00A0 => pub SCU_L_PWRUP_CNT: ReadWrite<u32>),
        /// SCU B 关机计数寄存器
        (0x00A4 => pub SCU_B_PWRDN_CNT: ReadWrite<u32>),
        /// SCU B 上电计数寄存器
        (0x00A8 => pub SCU_B_PWRUP_CNT: ReadWrite<u32>),
        /// GPU 关机计数寄存器
        (0x00AC => pub GPU_PWRDN_CNT: ReadWrite<u32>),
        /// GPU 上电计数寄存器
        (0x00B0 => pub GPU_PWRUP_CNT: ReadWrite<u32>),
        /// 中心关机计数寄存器
        (0x00B4 => pub CENTER_PWRDN_CNT: ReadWrite<u32>),
        /// 中心上电计数寄存器
        (0x00B8 => pub CENTER_PWRUP_CNT: ReadWrite<u32>),
        /// 超时计数寄存器
        (0x00BC => pub TIMEOUT_CNT: ReadWrite<u32>),
        /// CPU0A 电源管理寄存器
        (0x00C0 => pub CPU0APM_CON: ReadWrite<u32>),
        /// CPU1A 电源管理寄存器
        (0x00C4 => pub CPU1APM_CON: ReadWrite<u32>),
        /// CPU2A 电源管理寄存器
        (0x00C8 => pub CPU2APM_CON: ReadWrite<u32>),
        /// CPU3A 电源管理寄存器
        (0x00CC => pub CPU3APM_CON: ReadWrite<u32>),
        /// CPU0B 电源管理寄存器
        (0x00D0 => pub CPU0BPM_CON: ReadWrite<u32>),
        /// CPU1B 电源管理寄存器
        (0x00D4 => pub CPU1BPM_CON: ReadWrite<u32>),
        /// NOC 自动使能寄存器
        (0x00D8 => pub NOC_AUTO_ENA: ReadWrite<u32>),
        /// 电源关闭控制寄存器 1
        (0x00DC => pub PWRDN_CON1: ReadWrite<u32>),
        /// 寄存器结构结束
        (0x00E0 => @END),
    }
}

/// RK3588 PMU 寄存器访问结构
pub struct Rk3588Pmu {
    base: NonNull<Rk3588PmuRegs>,
}

impl Rk3588Pmu {
    /// 创建新的 PMU 实例
    ///
    /// # Safety
    /// 调用者必须确保 base 指向有效的 PMU 寄存器地址
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    /// 获取寄存器引用
    const fn regs(&self) -> &Rk3588PmuRegs {
        unsafe { self.base.as_ref() }
    }

    /// 读取电源状态寄存器
    pub fn read_power_status(&self) -> u32 {
        self.regs().POWER_ST.get()
    }

    /// 读取电源关闭状态寄存器
    pub fn read_pwrdn_status(&self) -> u32 {
        self.regs().PWRDN_ST.get()
    }

    /// 检查特定电源域是否开启
    /// 使用位域操作，提高代码可读性
    pub fn is_power_domain_on(&self, domain: crate::PowerDomain) -> bool {
        match domain {
            crate::PowerDomain::CpuLittle => self.regs().PWRDN_ST.is_set(PWRDN_ST::CPU_LITTLE_ST),
            crate::PowerDomain::CpuBig => self.regs().PWRDN_ST.is_set(PWRDN_ST::CPU_BIG_ST),
            crate::PowerDomain::Gpu => self.regs().PWRDN_ST.is_set(PWRDN_ST::GPU_ST),
            crate::PowerDomain::Npu => self.regs().PWRDN_ST.is_set(PWRDN_ST::NPU_ST),
            crate::PowerDomain::Vpu => self.regs().PWRDN_ST.is_set(PWRDN_ST::VPU_ST),
            crate::PowerDomain::Rga => self.regs().PWRDN_ST.is_set(PWRDN_ST::RGA_ST),
            crate::PowerDomain::Vi => self.regs().PWRDN_ST.is_set(PWRDN_ST::VI_ST),
            crate::PowerDomain::Vo => self.regs().PWRDN_ST.is_set(PWRDN_ST::VO_ST),
            crate::PowerDomain::Audio => self.regs().PWRDN_ST.is_set(PWRDN_ST::AUDIO_ST),
            crate::PowerDomain::Usb => self.regs().PWRDN_ST.is_set(PWRDN_ST::USB_ST),
            crate::PowerDomain::Pcie => self.regs().PWRDN_ST.is_set(PWRDN_ST::PCIE_ST),
            crate::PowerDomain::Sdmmc => self.regs().PWRDN_ST.is_set(PWRDN_ST::SDMMC_ST),
        }
    }

    /// 控制电源域开关
    /// 使用位域操作，避免直接地址操作
    pub fn control_power_domain(&self, domain: crate::PowerDomain, enable: bool) {
        match domain {
            crate::PowerDomain::CpuLittle => {
                if enable {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::CPU_LITTLE_PWRDN::PowerOn);
                } else {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::CPU_LITTLE_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::CpuBig => {
                if enable {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::CPU_BIG_PWRDN::PowerOn);
                } else {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::CPU_BIG_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Gpu => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::GPU_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::GPU_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Npu => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::NPU_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::NPU_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Vpu => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VPU_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VPU_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Rga => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::RGA_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::RGA_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Vi => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VI_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VI_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Vo => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VO_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::VO_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Audio => {
                if enable {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::AUDIO_PWRDN::PowerOn);
                } else {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::AUDIO_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Usb => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::USB_PWRDN::PowerOn);
                } else {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::USB_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Pcie => {
                if enable {
                    self.regs().PWRDN_CON.modify(PWRDN_CON::PCIE_PWRDN::PowerOn);
                } else {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::PCIE_PWRDN::PowerOff);
                }
            }
            crate::PowerDomain::Sdmmc => {
                if enable {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::SDMMC_PWRDN::PowerOn);
                } else {
                    self.regs()
                        .PWRDN_CON
                        .modify(PWRDN_CON::SDMMC_PWRDN::PowerOff);
                }
            }
        }
    }

    /// 配置唤醒源
    /// 使用位域操作设置唤醒源
    pub fn configure_wakeup(
        &self,
        gpio_mask: u8,
        rtc_enable: bool,
        net_enable: bool,
        usb_enable: bool,
    ) {
        self.regs().WAKEUP_CFG0.modify(
            WAKEUP_CFG0::GPIO_WAKEUP_EN.val(gpio_mask as u32)
                + WAKEUP_CFG0::RTC_WAKEUP_EN.val(rtc_enable as u32)
                + WAKEUP_CFG0::NET_WAKEUP_EN.val(net_enable as u32)
                + WAKEUP_CFG0::USB_WAKEUP_EN.val(usb_enable as u32),
        );
    }

    /// 设置电源模式
    pub fn set_power_mode(&self, mode: crate::PowerState) {
        let mode_val = match mode {
            crate::PowerState::On => 0,
            crate::PowerState::Sleep => 1,
            crate::PowerState::DeepSleep => 2,
            crate::PowerState::Standby => 3,
            crate::PowerState::Off => 4,
        };

        // 简化实现，直接写入模式值
        self.regs().PWRMODE_CON.set(mode_val);
    }

    /// 触发软件控制
    pub fn trigger_software_control(&self, reset: bool, sleep: bool, deep_sleep: bool) {
        let mut value = 0u32;
        if reset {
            value |= 1;
        }
        if sleep {
            value |= 2;
        }
        if deep_sleep {
            value |= 4;
        }

        self.regs().SFT_CON.set(value);
    }

    /// 配置功耗优化
    pub fn configure_power_optimization(&self, noc_auto: u32, bus_idle: u32) {
        self.regs().NOC_AUTO_ENA.set(noc_auto);
        self.regs().BUS_IDLE_REQ.set(bus_idle);
    }

    // 向后兼容的传统接口

    /// 传统接口：设置电源关闭控制
    pub fn set_pwrdn_control(&self, value: u32) {
        self.regs().PWRDN_CON.set(value);
    }

    /// 传统接口：修改电源关闭控制寄存器的特定位
    pub fn modify_pwrdn_control<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let current = self.regs().PWRDN_CON.get();
        self.regs().PWRDN_CON.set(f(current));
    }
}
