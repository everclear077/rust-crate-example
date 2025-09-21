# Rust Crate Example

一个功能完整的Rust配置管理库，提供环境感知的配置加载和管理功能。

## 🚀 功能特性

- **环境感知配置**: 支持开发环境（dev）和生产环境（prod）的自动切换
- **TOML配置文件**: 使用标准TOML格式进行配置管理
- **自动配置发现**: 从当前目录或父目录自动查找配置文件
- **类型安全**: 完全类型安全的配置结构定义
- **错误处理**: 完善的错误处理机制
- **默认配置**: 为不同环境提供合理的默认配置

## 📦 安装

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
rust-crate-example = "0.1.0"
```

## 🏗️ 项目结构

```
rust-crate-example/
├── bin/
│   └── main.rs          # 示例可执行文件
├── src/
│   ├── lib.rs           # 库的主入口
│   ├── env.rs           # 环境枚举和管理
│   └── conf/
│       ├── mod.rs       # 配置模块声明
│       ├── config.rs    # 配置结构定义
│       ├── error.rs     # 错误类型定义
│       └── global_config.rs  # 全局配置管理
├── config/
│   └── application.toml # 配置文件示例
└── Cargo.toml
```

## 🔧 使用方法

### 基本用法

```rust
use rust_crate_example::GlobalConfig;

fn main() {
    // 读取配置文件
    let config = GlobalConfig::read_config().unwrap();
    println!("{:#?}", config);
}
```

### 环境配置

库会根据以下规则自动确定运行环境：

1. 检查 `CONFIG_ENV` 环境变量
2. 在 debug 构建中默认使用 `dev` 环境
3. 在 release 构建中默认使用 `prod` 环境

支持的环境值：
- **开发环境**: `dev`, `d`, `development`, `devel`
- **生产环境**: `prod`, `p`, `production`

```bash
# 设置环境变量
export CONFIG_ENV=prod

# 或者在运行时指定
CONFIG_ENV=dev cargo run
```

### 配置文件格式

在项目根目录创建 `config/application.toml` 文件：

```toml
[dev]
address = "localhost"
port = "8080"
workers = 4

[dev.database]
adapter = "mysql"
name = "test"
pool = 5

[prod]
address = "0.0.0.0"
port = "443"

[prod.database]
adapter = "mysql"
name = "prod"
pool = 5
```

### 高级用法

```rust
use rust_crate_example::{Env, Config, GlobalConfig};

// 创建特定环境的配置
let dev_config = Config::new(Env::Dev);

// 检查当前环境
let current_env = Env::active().unwrap();
if current_env.is_dev() {
    println!("运行在开发环境");
}

// 从文件加载配置
let global_config = GlobalConfig::read_config().unwrap();
```

## 📋 API 文档

### 核心类型

#### `Env` 枚举
环境类型枚举，支持开发和生产环境。

```rust
pub enum Env {
    Dev,
    Prod,
}
```

**方法**:
- `active() -> Result<Env, ConfigError>`: 获取当前活动环境
- `is_dev(self) -> bool`: 检查是否为开发环境
- `is_prod(self) -> bool`: 检查是否为生产环境

#### `Config` 结构体
单个环境的配置结构。

```rust
pub struct Config {
    pub env: Env,
    pub address: String,
    pub port: u16,
    pub workers: Option<u16>,
    pub database: Option<Database>,
}
```

#### `Database` 结构体
数据库配置结构。

```rust
pub struct Database {
    pub adapter: String,
    pub name: String,
    pub pool: u16,
}
```

#### `GlobalConfig` 结构体
全局配置管理器，包含所有环境的配置。

**方法**:
- `read_config() -> Result<GlobalConfig>`: 从文件读取配置

### 错误处理

库定义了 `ConfigError` 枚举来处理各种错误情况：

- `NotFound`: 配置文件未找到
- `IoError`: I/O 错误
- `BadFilePath`: 无效文件路径
- `BadEnv`: 无效环境变量
- `ParseError`: 配置解析错误

## 🏃‍♂️ 运行示例

```bash
# 构建项目
cargo build

# 运行示例
cargo run --bin main

# 指定环境运行
CONFIG_ENV=prod cargo run --bin main

# 运行测试
cargo test
```

## 🛠️ 开发

### 构建要求

- Rust 1.70 或更高版本
- Cargo

### 依赖项

- `num_cpus`: CPU 核心数检测
- `serde`: 序列化和反序列化
- `toml`: TOML 文件解析

### 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🤝 致谢

感谢所有为本项目做出贡献的开发者。

---

*如有问题或建议，请提交 Issue 或 Pull Request。*
