# RK3588 电源管理驱动

一个专为瑞芯微 RK3588/RK3588S 芯片设计的高性能电源管理驱动库，基于 Rust 语言开发，支持裸机环境（no-std）运行。该驱动提供了完整的电源域控制、CPU 频率管理、热管理和功耗监控功能，是嵌入式系统开发的理想选择。

## 兼容性

### SoC 支持
- **RK3588** (完整版本) - 8核 CPU（4×A76 + 4×A55），支持完整的电源管理功能
- **RK3588S** (简化版本) - 8核 CPU，兼容大部分电源管理特性
- **Orange Pi 5 Plus** - 完全兼容，基于 RK3588S 设计
- 其他基于 RK3588/RK3588S 的开发板

## 功能特性

### 🔋 电源域管理
- **精确控制**：支持 12 个电源域的独立精确控制
- **CPU 集群**：Cortex-A55 小核心集群和 Cortex-A76 大核心集群独立管理
- **图形处理**：Mali-G610 GPU 电源精细化控制
- **专用处理器**：NPU（神经网络）、VPU（视频）、RGA（2D图形）电源管理
- **外设系统**：音频、USB、PCIe、SDMMC 等外设电源完全控制
- **状态监控**：实时监控各电源域状态和功耗
- **智能优化**：根据负载自动优化电源分配

### ⚡ CPU 频率管理
- **独立调频**：大小核心集群完全独立的频率控制
- **丰富档位**：11 个预定义频率档位（408MHz ~ 2.4GHz），满足各种性能需求
- **DVFS 支持**：动态电压频率调节，自动匹配最优电压
- **智能节流**：基于温度的智能热节流机制
- **实时调节**：毫秒级响应的频率切换
- **能效优化**：智能选择最佳频率电压组合

### 🌡️ 智能热管理
- **实时监控**：高精度温度传感器实时监控
- **多级保护**：三级过热保护机制（警告、限频、关断）
- **动态调节**：根据温度自动调整 CPU 频率和电压
- **智能关断**：关键温度下自动关闭非必要组件
- **预测控制**：基于温度趋势的预测性控制
- **自定义策略**：可配置的热管理策略

### 😴 多级睡眠模式
- **浅睡眠**：保持内存数据，快速唤醒（<10ms）
- **深度睡眠**：关闭非必要电源域，极低功耗模式
- **灵活唤醒**：支持多种唤醒源配置（GPIO、RTC、网络等）
- **状态保存**：自动保存和恢复系统状态
- **快速恢复**：优化的唤醒流程确保快速系统恢复
- **功耗极低**：深度睡眠模式功耗低至微瓦级别

### 📊 精确功耗监控
- **实时计算**：基于频率、电压和负载的实时功耗计算
- **域级分析**：每个电源域的独立功耗分析和监控
- **历史统计**：功耗历史数据统计和趋势分析
- **优化建议**：基于使用模式的功耗优化建议
- **能效评估**：系统整体能效评估和报告
- **预算管理**：支持功耗预算设置和超限告警

## 架构设计

### 系统架构图
```
┌─────────────────────────────────────────────────────────────┐
│                    RK3588PowerManager                       │
├─────────────────────────────────────────────────────────────┤
│  Power Domain Control  │  CPU Frequency  │  Thermal Mgmt   │
│  • CpuLittle/Big      │  • DVFS         │  • Temperature   │
│  • GPU/NPU/VPU        │  • Voltage      │  • Throttling    │
│  • Audio/USB/PCIe     │  • Presets      │  • Protection    │
├─────────────────────────────────────────────────────────────┤
│                    RegisterAccess Trait                     │
├─────────────────────────────────────────────────────────────┤
│        MMIO Implementation    │    Mock Implementation       │
│        (Production)           │    (Testing)                 │
└─────────────────────────────────────────────────────────────┘
```

### 设计原则
- **模块化**：清晰的功能模块分离，每个模块专注于特定功能
- **可扩展**：基于 trait 的抽象接口，易于扩展和自定义
- **类型安全**：利用 Rust 的类型系统确保编译期安全
- **性能优先**：编译时优化，零开销抽象
- **测试友好**：内置 Mock 实现，支持单元测试

## 核心组件

### PowerDomain 枚举 - 电源域定义
完整定义了 RK3588/RK3588S 支持的所有电源域：

| 电源域 | 功能描述 | 功耗估算 |
|---------|------------|----------|
| `CpuLittle` | Cortex-A55 小核心集群 (4核) | 0.5-2.0W |
| `CpuBig` | Cortex-A76 大核心集群 (4核) | 1.0-8.0W |
| `Gpu` | Mali-G610 MP4 图形处理器 | 2.0-15.0W |
| `Npu` | 6 TOPS AI 神经网络处理器 | 1.0-6.0W |
| `Vpu` | 8K@60fps 视频编解码器 | 0.5-3.0W |
| `Rga` | 2D 图形加速器 | 0.1-0.5W |
| `Vi` | 视频输入处理器 | 0.2-1.0W |
| `Vo` | 视频输出处理器 | 0.3-1.5W |
| `Audio` | 音频处理器 | 0.05-0.2W |
| `Usb` | USB 3.0/2.0 控制器 | 0.1-0.8W |
| `Pcie` | PCIe 3.0 控制器 | 0.2-1.5W |
| `Sdmmc` | SD/MMC 存储控制器 | 0.1-0.5W |

### CPU 频率档位详情
驱动支持 11 个精心调校的频率电压配置：

| 频率 | 电压 | 应用场景 | 适用核心 | TDP 估算 |
|------|------|----------|------------|----------|
| 408MHz | 900mV | 超低功耗、待机 | 大小核 | 0.2-0.5W |
| 600MHz | 925mV | 节能模式 | 大小核 | 0.4-0.8W |
| 816MHz | 950mV | 低功耗应用 | 大小核 | 0.6-1.2W |
| 1.008GHz | 975mV | 轻载工作 | 大小核 | 0.8-1.5W |
| 1.200GHz | 1050mV | 日常应用 | 大小核 | 1.0-2.0W |
| 1.416GHz | 1100mV | 中等负载 | 主要适用大核 | 1.5-3.0W |
| 1.608GHz | 1150mV | 高效性能 | 主要适用大核 | 2.0-4.0W |
| 1.800GHz | 1200mV | 游戏娱乐 | 主要适用大核 | 2.5-5.0W |
| 2.016GHz | 1300mV | 计算密集 | 主要适用大核 | 3.5-6.5W |
| 2.208GHz | 1350mV | 极限性能 | 主要适用大核 | 4.0-7.5W |
| 2.400GHz | 1400mV | 最大性能 | 主要适用大核 | 5.0-8.5W |

> **注意**：实际 TDP 会根据工作负载、温度和具体应用场景而变化。

## 使用示例

### 类型安全的寄存器操作

```rust
use rk3588_power::*;

// 创建电源管理器实例
let mut power_manager = create_default_power_manager();
power_manager.init().expect("初始化失败");

// 类型安全的寄存器访问 - 编译时保证正确性
// 无法对只读寄存器进行写操作
// 无法访问保留的寄存器区域
// 自动处理寄存器偏移和映射
```

### 电源域控制

```rust
// 关闭 GPU 电源域以节省功耗
power_manager.control_power_domain(PowerDomain::Gpu, PowerState::Off)
    .expect("Failed to control GPU power domain");

// 将 NPU 设置为待机模式
power_manager.control_power_domain(PowerDomain::Npu, PowerState::Standby)
    .expect("Failed to set NPU to standby");

// 重新开启 GPU
power_manager.control_power_domain(PowerDomain::Gpu, PowerState::On)
    .expect("Failed to power on GPU");
```

### CPU 频率调节

```rust
// 设置 CPU 大核心为高性能模式
power_manager.set_cpu_frequency(PowerDomain::CpuBig, cpu_freqs::FREQ_2208M)
    .expect("Failed to set CPU frequency");

// 设置 CPU 小核心为节能模式
power_manager.set_cpu_frequency(PowerDomain::CpuLittle, cpu_freqs::FREQ_816M)
    .expect("Failed to set CPU frequency");

// 获取当前频率
let freq = power_manager.get_cpu_frequency(PowerDomain::CpuBig)
    .expect("Failed to get CPU frequency");
println!("CPU Big frequency: {} MHz @ {} mV", freq.freq_mhz, freq.voltage_mv);
```

### 系统睡眠

```rust
// 进入浅睡眠模式
power_manager.enter_sleep_mode(PowerState::Sleep)
    .expect("Failed to enter sleep mode");

// 进入深睡眠模式（会自动关闭非必要组件）
power_manager.enter_sleep_mode(PowerState::DeepSleep)
    .expect("Failed to enter deep sleep mode");
```

### 热管理

```rust
// 执行热管理检查
power_manager.thermal_management()
    .expect("Thermal management failed");

// 获取当前温度
let temperature = power_manager.get_temperature()
    .expect("Failed to get temperature");
println!("Current temperature: {:.1}°C", temperature);
```

### 状态监控

```rust
// 获取详细的电源状态
let status = power_manager.get_power_status()
    .expect("Failed to get power status");

println!("{}", status);

// 获取功耗信息
let power_consumption = power_manager.get_power_consumption()
    .expect("Failed to get power consumption");
println!("Total power consumption: {:.1} mW", power_consumption);
```

## 构建和测试

### 环境准备

```bash
# 安装所需工具
cargo install ostool

# 添加目标架构支持
rustup target add aarch64-unknown-none-softfloat
```

### 构建项目

```bash
# 构建库
cargo build

# 构建发布版本
cargo build --release
```

### 运行测试

```bash
# 运行单元测试
cargo test --test test -- tests --show-output

# 在开发板上测试（需要 U-Boot 环境）
cargo test --test test -- tests --show-output --uboot
```

## 技术特点

### 🔒 安全性
- **内存安全**：基于 Rust 语言，编译期保证内存安全，无指针悬挂风险
- **类型安全**：强类型的电源域和状态管理，编译期防止错误操作
- **线程安全**：内置的同步机制和竞争条件防护
- **边界检查**：自动防止数组越界和缓冲区溢出

### 🚀 可扩展性
- **模块化设计**：基于 trait 的寄存器访问抽象，支持不同硬件平台
- **易于扩展**：简单添加新的电源域和功能模块
- **插件支持**：支持自定义的电源策略和优化算法
- **平台适配**：可轻松适配其他 RK 系列芯片

### 🧪 测试友好
- **Mock 实现**：提供完整的 Mock 实现用于单元测试
- **测试覆盖**：完整的测试套件和回归测试
- **CI/CD 集成**：支持 GitHub Actions 和其他 CI/CD 平台
- **仿真测试**：支持 QEMU 仿真环境测试

### 📱 嵌入式友好
- **no-std 支持**：适用于裸机环境，无需操作系统
- **小内存占用**：精心优化的内存使用，适合资源受限环境
- **高效访问**：直接内存映射 I/O，最小化开销
- **实时响应**：低延迟的电源控制和快速响应

## 依赖项和版本要求

### 核心依赖
- **log**: 结构化日志记录，支持多级别日志
- **tock-registers**: 类型安全的寄存器访问和位域操作
- **mbarrier**: 内存屏障原语，确保寄存器访问顺序

### 开发依赖
- **bare-test**: 裸机测试框架，支持 no-std 环境
- **rustfmt**: 代码格式化工具
- **clippy**: 代码质量检查工具

### 系统要求
- **Rust 版本**: 1.75.0 或更高
- **目标架构**: aarch64-unknown-none-softfloat
- **开发环境**: Linux/macOS/Windows + Rust 工具链
- **部署环境**: RK3588/RK3588S 开发板或仿真器

## 开发指南

### 添加新的电源域

1. 在 `PowerDomain` 枚举中添加新域
2. 更新 `domain_states` 数组大小
3. 在相关函数中添加处理逻辑
4. 添加对应的测试用例

### 自定义寄存器访问

实现 `RegisterAccess` trait 来支持不同的硬件访问方式：

```rust
struct MyRegisterAccess;

impl RegisterAccess for MyRegisterAccess {
    unsafe fn read_reg(&self, addr: u32) -> u32 {
        // 自定义读取实现
    }
    
    unsafe fn write_reg(&self, addr: u32, value: u32) {
        // 自定义写入实现
    }
}

let power_manager = Rk3588PowerManager::new(MyRegisterAccess);
```

## 许可证

本项目采用开源许可证，详见 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

### 开发环境设置

```bash
# 克隆项目
git clone <repository-url>
cd rk3588-power

# 安装依赖
rustup component add rustfmt clippy

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 支持与兼容性

### 硬件支持
- **主要芯片**: RK3588、RK3588S
- **开发板**: 
  - Orange Pi 5/5 Plus/5B
  - Rock 5A/5B/5C
  - NanoPC-T6
  - 其他基于 RK3588/RK3588S 的开发板
- **CPU 架构**: ARM Cortex-A55/A76 异构构八核
- **GPU**: Mali-G610 MP4
- **NPU**: 6 TOPS AI 加速器

### 软件支持
- **引导环境**: U-Boot、UEFI、直接启动
- **操作系统**: 裸机环境 (no-std)、RTOS
- **仿真器**: QEMU aarch64 系统仿真
- **开发工具**: Rust 1.75+、GDB、OpenOCD

### 特性兼容
- **向下兼容**: RK3588S 功能子集完全支持
- **向上扩展**: 为未来 RK 系列芯片预留扩展接口
- **平台适配**: 可轻松移植到其他 ARM64 平台

---

**注意**: 本驱动为底层系统软件，使用时请确保对硬件寄存器的操作符合芯片规格要求。在生产环境中使用前，请进行充分的测试验证。
