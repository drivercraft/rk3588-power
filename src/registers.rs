//! RK3588 PMU 寄存器定义
//! 
//! 使用 tock-registers 库提供的宏来定义 RK3588 PMU（电源管理单元）的寄存器结构

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};
use core::ptr::NonNull;

/// RK3588 PMU（电源管理单元）基地址
/// 兼容 RK3588 和 RK3588S，适用于 Orange Pi 5 Plus 等开发板
pub const RK3588_PMU_BASE: u32 = 0xFD8D_0000;

register_structs! {
    /// RK3588 PMU 寄存器结构
    #[allow(non_snake_case)]
    pub Rk3588PmuRegs {
        /// 唤醒配置寄存器 0
        (0x0000 => pub WAKEUP_CFG0: ReadWrite<u32>),
        /// 唤醒配置寄存器 1  
        (0x0004 => pub WAKEUP_CFG1: ReadWrite<u32>),
        /// 保留区域 1
        (0x0008 => _reserved1),
        /// 电源关闭控制寄存器
        (0x0018 => pub PWRDN_CON: ReadWrite<u32>),
        /// 保留区域 2
        (0x001C => _reserved2),
        /// 电源关闭状态寄存器
        (0x0020 => pub PWRDN_ST: ReadOnly<u32>),
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
    
    /// 设置电源关闭控制
    pub fn set_pwrdn_control(&self, value: u32) {
        self.regs().PWRDN_CON.set(value);
    }
    
    /// 修改电源关闭控制寄存器的特定位
    pub fn modify_pwrdn_control<F>(&self, f: F) 
    where
        F: FnOnce(u32) -> u32,
    {
        let current = self.regs().PWRDN_CON.get();
        self.regs().PWRDN_CON.set(f(current));
    }
    
    /// 配置唤醒源
    pub fn configure_wakeup(&self, cfg0: u32, gpio0_pos: u32) {
        self.regs().WAKEUP_CFG0.set(cfg0);
        self.regs().GPIO0_POS_INT_CON.set(gpio0_pos);
    }
    
    /// 设置电源模式
    pub fn set_power_mode(&self, mode: u32) {
        self.regs().PWRMODE_CON.set(mode);
    }
    
    /// 触发软件控制
    pub fn trigger_software_control(&self, value: u32) {
        self.regs().SFT_CON.set(value);
    }
    
    /// 配置功耗优化
    pub fn configure_power_optimization(&self, noc_auto: u32, bus_idle: u32) {
        self.regs().NOC_AUTO_ENA.set(noc_auto);
        self.regs().BUS_IDLE_REQ.set(bus_idle);
    }
}