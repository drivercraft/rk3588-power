//! RK3588 电源管理核心实现模块
//!
//! 本模块包含 RK3588 电源管理的核心功能实现，包括：
//! - 电源域控制
//! - CPU 频率管理
//! - 热管理
//! - 睡眠模式控制

use crate::registers::{RK3588_PMU_BASE, Rk3588Pmu, power_optimization, wakeup_config};
use crate::{
    CpuFreq, PowerDomain, PowerError, PowerResult, PowerState, PowerStatus, RegisterAccess,
    cpu_freqs,
};
use log::{debug, error, info, warn};

/// RK3588 电源管理驱动主结构
pub struct Rk3588PowerManager<R: RegisterAccess> {
    _register_access: R,
    pmu: Rk3588Pmu,
    /// 当前各电源域状态
    domain_states: [PowerState; 12],
    /// 当前 CPU 频率配置
    cpu_little_freq: CpuFreq,
    cpu_big_freq: CpuFreq,
}

impl<R: RegisterAccess> Rk3588PowerManager<R> {
    /// 创建新的电源管理实例
    pub fn new(register_access: R) -> Self {
        Self {
            _register_access: register_access,
            pmu: Rk3588Pmu::new(RK3588_PMU_BASE as *mut u8),
            domain_states: [PowerState::On; 12],
            cpu_little_freq: cpu_freqs::FREQ_1200M,
            cpu_big_freq: cpu_freqs::FREQ_2016M,
        }
    }

    /// 初始化电源管理系统
    pub fn init(&mut self) -> PowerResult<()> {
        info!("Initializing RK3588 power management system");

        // 读取当前电源状态
        let power_st = self.pmu.read_power_status();

        debug!("Current power status: 0x{:08x}", power_st);

        // 初始化各电源域状态
        self.update_domain_states()?;

        // 配置唤醒源
        self.configure_wakeup_sources()?;

        // 设置默认的功耗优化配置
        self.configure_power_optimization()?;

        info!("RK3588 power management initialized successfully");
        Ok(())
    }

    /// 更新电源域状态
    fn update_domain_states(&mut self) -> PowerResult<()> {
        let power_st = self.pmu.read_power_status();

        for (i, state) in self.domain_states.iter_mut().enumerate() {
            let domain_bit = 1 << i;
            *state = if (power_st & domain_bit) != 0 {
                PowerState::On
            } else {
                PowerState::Off
            };
        }

        Ok(())
    }

    /// 配置唤醒源
    fn configure_wakeup_sources(&mut self) -> PowerResult<()> {
        // 使用预定义常量，避免魔数
        self.pmu.configure_wakeup(
            wakeup_config::GPIO_WAKEUP_PIN_0_1, // GPIO 0-1 唤醒使能
            true,                               // RTC 唤醒使能
            false,                              // 网络唤醒禁用
            true,                               // USB 唤醒使能
        );

        debug!("Wakeup sources configured using predefined constants");
        Ok(())
    }

    /// 配置功耗优化
    fn configure_power_optimization(&mut self) -> PowerResult<()> {
        // 使用预定义常量，完全避免魔数
        self.pmu.configure_power_optimization(
            power_optimization::NOC_ALL_AUTO_FEATURES_ENABLED,
            power_optimization::BUS_IDLE_ALL_DISABLED,
        );

        debug!("Power optimization configured: NOC auto features enabled, bus idle disabled");
        Ok(())
    }

    /// 控制电源域
    pub fn control_power_domain(
        &mut self,
        domain: PowerDomain,
        state: PowerState,
    ) -> PowerResult<()> {
        let domain_idx = domain as usize;
        if domain_idx >= self.domain_states.len() {
            return Err(PowerError::InvalidDomain);
        }

        info!("Controlling power domain {:?} to state {:?}", domain, state);

        match state {
            PowerState::On => self.power_on_domain(domain)?,
            PowerState::Off => self.power_off_domain(domain)?,
            PowerState::Standby => self.standby_domain(domain)?,
            _ => return Err(PowerError::Unsupported),
        }

        self.domain_states[domain_idx] = state;
        Ok(())
    }

    /// 开启电源域
    fn power_on_domain(&mut self, domain: PowerDomain) -> PowerResult<()> {
        // 使用新的位域接口
        self.pmu.control_power_domain(domain, true);

        // 等待电源域稳定
        let mut timeout = 1000;
        while timeout > 0 {
            if self.pmu.is_power_domain_on(domain) {
                break;
            }
            timeout -= 1;
            // 简单延时
            for _ in 0..100 {
                core::hint::spin_loop();
            }
        }

        if timeout == 0 {
            error!("Timeout waiting for domain {:?} to power on", domain);
            return Err(PowerError::Timeout);
        }

        debug!("Power domain {:?} powered on successfully", domain);
        Ok(())
    }

    /// 关闭电源域
    fn power_off_domain(&mut self, domain: PowerDomain) -> PowerResult<()> {
        // 使用新的位域接口
        self.pmu.control_power_domain(domain, false);

        // 等待电源域关闭
        let mut timeout = 1000;
        while timeout > 0 {
            if !self.pmu.is_power_domain_on(domain) {
                break;
            }
            timeout -= 1;
            // 简单延时
            for _ in 0..100 {
                core::hint::spin_loop();
            }
        }

        if timeout == 0 {
            error!("Timeout waiting for domain {:?} to power off", domain);
            return Err(PowerError::Timeout);
        }

        debug!("Power domain {:?} powered off successfully", domain);
        Ok(())
    }

    /// 电源域待机
    fn standby_domain(&mut self, domain: PowerDomain) -> PowerResult<()> {
        debug!("Power domain {:?} entering standby mode", domain);

        // 这里可以实现具体的待机逻辑，比如降低时钟频率等
        match domain {
            PowerDomain::CpuLittle | PowerDomain::CpuBig => {
                // CPU 待机时降低频率
                self.set_cpu_frequency(domain, cpu_freqs::FREQ_408M)?;
            }
            PowerDomain::Gpu => {
                // GPU 待机时可以保持最低频率
            }
            _ => {
                // 其他域的待机处理
            }
        }

        Ok(())
    }

    /// 设置 CPU 频率
    pub fn set_cpu_frequency(&mut self, domain: PowerDomain, freq: CpuFreq) -> PowerResult<()> {
        match domain {
            PowerDomain::CpuLittle => {
                info!(
                    "Setting CPU little cluster frequency to {} MHz, {} mV",
                    freq.freq_mhz, freq.voltage_mv
                );
                self.cpu_little_freq = freq;
            }
            PowerDomain::CpuBig => {
                info!(
                    "Setting CPU big cluster frequency to {} MHz, {} mV",
                    freq.freq_mhz, freq.voltage_mv
                );
                self.cpu_big_freq = freq;
            }
            _ => return Err(PowerError::InvalidDomain),
        }

        // 这里应该实现实际的频率设置逻辑
        // 包括 PLL 配置、分频器设置、电压调节等

        Ok(())
    }

    /// 进入系统睡眠模式
    pub fn enter_sleep_mode(&mut self, mode: PowerState) -> PowerResult<()> {
        match mode {
            PowerState::Sleep => self.enter_sleep()?,
            PowerState::DeepSleep => self.enter_deep_sleep()?,
            _ => return Err(PowerError::Unsupported),
        }
        Ok(())
    }

    /// 进入浅睡眠
    fn enter_sleep(&mut self) -> PowerResult<()> {
        info!("Entering sleep mode");

        // 使用新的接口设置睡眠模式
        self.pmu.set_power_mode(PowerState::Sleep);
        self.pmu.trigger_software_control(false, true, false);

        Ok(())
    }

    /// 进入深度睡眠
    fn enter_deep_sleep(&mut self) -> PowerResult<()> {
        info!("Entering deep sleep mode");

        // 先关闭非必要的电源域
        let non_essential_domains = [
            PowerDomain::Gpu,
            PowerDomain::Npu,
            PowerDomain::Vpu,
            PowerDomain::Rga,
        ];

        for domain in &non_essential_domains {
            self.control_power_domain(*domain, PowerState::Off)?;
        }

        // 使用新的接口设置深度睡眠模式
        self.pmu.set_power_mode(PowerState::DeepSleep);
        self.pmu.trigger_software_control(false, false, true);

        Ok(())
    }

    /// 获取电源域状态
    pub fn get_power_domain_state(&self, domain: PowerDomain) -> PowerState {
        let domain_idx = domain as usize;
        if domain_idx < self.domain_states.len() {
            self.domain_states[domain_idx]
        } else {
            PowerState::Off
        }
    }

    /// 获取当前 CPU 频率
    pub fn get_cpu_frequency(&self, domain: PowerDomain) -> PowerResult<CpuFreq> {
        match domain {
            PowerDomain::CpuLittle => Ok(self.cpu_little_freq),
            PowerDomain::CpuBig => Ok(self.cpu_big_freq),
            _ => Err(PowerError::InvalidDomain),
        }
    }

    /// 获取系统功耗信息
    pub fn get_power_consumption(&self) -> PowerResult<f32> {
        // 这里可以实现功耗监测逻辑
        // 基于当前各域状态和频率估算功耗
        let mut total_power = 0.0f32;

        // 基础系统功耗
        total_power += 500.0; // mW

        // CPU 功耗计算
        if self.domain_states[PowerDomain::CpuLittle as usize] == PowerState::On {
            total_power += (self.cpu_little_freq.freq_mhz as f32 * 0.5)
                + (self.cpu_little_freq.voltage_mv as f32 * 0.1);
        }

        if self.domain_states[PowerDomain::CpuBig as usize] == PowerState::On {
            total_power += (self.cpu_big_freq.freq_mhz as f32 * 0.8)
                + (self.cpu_big_freq.voltage_mv as f32 * 0.15);
        }

        // GPU 功耗
        if self.domain_states[PowerDomain::Gpu as usize] == PowerState::On {
            total_power += 2000.0; // GPU 典型功耗
        }

        // NPU 功耗
        if self.domain_states[PowerDomain::Npu as usize] == PowerState::On {
            total_power += 1500.0; // NPU 典型功耗
        }

        Ok(total_power)
    }

    /// 获取芯片温度（模拟实现）
    pub fn get_temperature(&self) -> PowerResult<f32> {
        // 实际实现中应该读取温度传感器
        // 这里返回一个模拟值
        let base_temp = 40.0f32;
        let power_consumption = self.get_power_consumption()?;

        // 基于功耗估算温度增量
        let temp_delta = power_consumption / 1000.0 * 10.0;

        Ok(base_temp + temp_delta)
    }

    /// 温度管理和过热保护（适配 Orange Pi 5 Plus 等开发板）
    pub fn thermal_management(&mut self) -> PowerResult<()> {
        let temperature = self.get_temperature()?;

        debug!("Current temperature: {:.1}°C", temperature);

        // Orange Pi 5 Plus 特定的温度阈值（考虑开发板散热设计）
        if temperature > 80.0 {
            warn!(
                "Temperature high: {:.1}°C, implementing thermal throttling",
                temperature
            );

            // 实施热节流措施
            // 1. 降低 CPU 频率（Orange Pi 5 Plus 适配）
            self.set_cpu_frequency(PowerDomain::CpuBig, cpu_freqs::FREQ_1608M)?;
            self.set_cpu_frequency(PowerDomain::CpuLittle, cpu_freqs::FREQ_1008M)?;

            // 2. 如果温度过高，关闭非必要组件
            if temperature > 85.0 {
                error!(
                    "Critical temperature: {:.1}°C, shutting down non-essential domains",
                    temperature
                );
                self.control_power_domain(PowerDomain::Gpu, PowerState::Off)?;
                self.control_power_domain(PowerDomain::Npu, PowerState::Off)?;

                if temperature > 90.0 {
                    return Err(PowerError::OverTemperature);
                }
            }
        } else if temperature < 65.0 {
            // 温度正常，可以恢复性能
            debug!("Temperature normal, restoring performance");
        }

        Ok(())
    }

    /// 获取详细的电源状态信息
    pub fn get_power_status(&mut self) -> PowerResult<PowerStatus> {
        self.update_domain_states()?;

        Ok(PowerStatus {
            domains: self.domain_states.clone(),
            cpu_little_freq: self.cpu_little_freq,
            cpu_big_freq: self.cpu_big_freq,
            power_consumption: self.get_power_consumption()?,
            temperature: self.get_temperature()?,
        })
    }
}
