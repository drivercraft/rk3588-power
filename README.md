# Rockchip 电源管理驱动

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
