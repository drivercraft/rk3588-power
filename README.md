# RK3588 电源管理驱动

适用于瑞芯微 RK3588 芯片的电源管理驱动库，基于 Rust 编写，支持裸机环境（no-std）。

## 兼容性

本驱动支持以下平台：

### SoC 支持
- **RK3588** (完整版本) - 8核 CPU, 支持完整的 45 个电源域
- **RK3588S** (简化版本) - 8核 CPU, 部分电源域

## 功能特性

### 🔋 电源域管理
- 支持 12 个电源域的独立控制
- CPU 小核心集群（Cortex-A55）
- CPU 大核心集群（Cortex-A76）
- GPU（Mali-G610）电源控制
- NPU、VPU、RGA 等专用处理器电源管理
- 音频、USB、PCIe、SDMMC 等外设电源控制

### ⚡ CPU 频率管理
- 支持 CPU 大小核心集群独立调频
- 预定义的频率档位（408MHz ~ 2.4GHz）
- 动态电压调节（DVFS）
- 智能热节流机制

### 🌡️ 热管理
- 实时温度监控
- 过热保护机制
- 动态频率调节
- 关键温度下自动关闭非必要组件

### 😴 睡眠模式
- 浅睡眠模式
- 深睡眠模式（关闭非必要电源域）
- 可配置的唤醒源
- GPIO 唤醒支持

### 📊 功耗监控
- 实时功耗计算
- 基于频率和电压的功耗估算
- 各电源域功耗分析

## 架构设计

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

## 核心组件

### PowerDomain 枚举
定义了 RK3588 支持的所有电源域：
- `CpuLittle`: A55 小核心集群
- `CpuBig`: A76 大核心集群
- `Gpu`: Mali-G610 GPU
- `Npu`: 神经网络处理器
- `Vpu`: 视频处理器
- `Rga`: 2D 图形加速器
- `Vi/Vo`: 视频输入/输出
- `Audio`: 音频处理器
- `Usb/Pcie/Sdmmc`: 外设控制器

### CPU 频率档位
预定义的 CPU 频率和电压配置：

| 频率 | 电压 | 应用场景 |
|------|------|----------|
| 408MHz | 900mV | 超低功耗 |
| 816MHz | 950mV | 低功耗应用 |
| 1.2GHz | 1050mV | 日常应用 |
| 1.8GHz | 1200mV | 中等性能 |
| 2.4GHz | 1400mV | 高性能应用 |

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

### 安全性
- 基于 Rust 语言，保证内存安全
- 类型安全的电源域和状态管理
- 编译时错误检查

### 可扩展性
- 基于 trait 的寄存器访问抽象
- 支持不同的硬件访问实现
- 易于添加新的电源域和功能

### 测试友好
- 提供 Mock 实现用于单元测试
- 完整的测试覆盖
- 支持 QEMU 仿真测试

### 嵌入式友好
- `no-std` 支持，适用于裸机环境
- 最小化内存占用
- 高效的寄存器访问

## 依赖项

- `log`: 日志记录
- `dma-api`: DMA 操作支持
- `mbarrier`: 内存屏障
- `bare-test`: 裸机测试框架（开发依赖）

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

## 支持

- RK3588 芯片及其变种
- ARM Cortex-A55/A76 架构
- U-Boot 引导环境
- 裸机运行环境

---

**注意**: 本驱动为底层系统软件，使用时请确保对硬件寄存器的操作符合芯片规格要求。在生产环境中使用前，请进行充分的测试验证。
