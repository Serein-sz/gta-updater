# GTA Updater

一个用于自动更新 GitHub Release 资产的命令行工具。

## 功能特性

- 🚀 自动检测最新的 GitHub Release 版本
- 📦 根据操作系统和架构自动选择合适的资产包
- ⬇️ 下载并安装更新
- 🔧 支持配置多个应用程序
- 🌍 跨平台支持 (Linux, macOS, Windows)

## 支持的平台

- **操作系统**: Linux, macOS, Windows
- **架构**: x86_64 (amd64), aarch64 (arm64)

## 安装

### 从源码构建

```bash
git clone https://github.com/Serein-sz/gta-updater.git
cd gta-updater
cargo build --release
```

编译后的二进制文件位于 `target/release/gta-updater`。

## 配置

创建配置文件 `config.toml`，位置：

- **Windows/macOS**: `~/.config/gta-updater/config.toml`
- **Linux**: `$XDG_CONFIG_HOME/gta-updater/config.toml` 或 `~/.config/gta-updater/config.toml`

你也可以通过环境变量 `CPA_CONFIG_DIR` 自定义配置目录。

### 配置示例

```toml
github_owner = "Serein-sz"
global_path = "/usr/local/bin"  # 全局安装路径

[[apps]]
name = "body-recorder"          # GitHub 仓库名称
alias = "br"                    # 可选：命令别名
version = "v0.1.0"              # 当前版本
path = "/custom/path"           # 可选：自定义安装路径
```

### 配置字段说明

- `github_owner`: GitHub 用户名或组织名
- `global_path`: 默认的全局安装路径
- `apps`: 要管理的应用列表
  - `name`: GitHub 仓库名称
  - `alias`: (可选) 命令别名
  - `version`: 当前安装的版本
  - `path`: (可选) 自定义安装路径，优先于 `global_path`

## 使用方法

### 基本用法

```bash
# 检查并更新所有配置的应用
gta

# 更新指定的应用
gta --app body-recorder
gta -a br  # 使用别名

# 强制更新（即使已是最新版本）
gta --force

# 预览更新（不实际下载）
gta --dry-run

# 显示详细信息
gta --verbose

# 组合使用
gta -a body-recorder -v --dry-run
```

### 命令行参数

| 参数 | 短选项 | 说明 |
|------|--------|------|
| `--app <NAME>` | `-a` | 只更新指定的应用（支持名称或别名） |
| `--force` | `-f` | 强制更新，即使已是最新版本 |
| `--dry-run` | `-n` | 预览模式，不实际下载 |
| `--verbose` | `-v` | 显示详细输出信息 |
| `--help` | `-h` | 显示帮助信息 |
| `--version` | `-V` | 显示版本信息 |

### 环境变量

所有配置项都可以通过环境变量覆盖，使用 `GTA__` 前缀：

```bash
GTA__GITHUB_OWNER=Serein-sz
GTA__GLOBAL_PATH=/usr/local/bin
GTA__APPS__0__NAME=body-recorder
GTA__APPS__0__VERSION=v0.1.0
```

## 工作原理

1. 读取配置文件，获取要管理的应用列表
2. 通过 GitHub API 查询每个应用的最新 release
3. 根据当前系统的操作系统和架构，筛选匹配的资产文件
4. 下载并安装到指定路径

## 开发

### 前置要求

- Rust 1.70 或更高版本
- Cargo

### 运行测试

```bash
cargo test
```

### 本地开发运行

```bash
# 直接运行
cargo run

# 带参数运行
cargo run -- --verbose --dry-run
cargo run -- -a body-recorder

# 构建 release 版本
cargo build --release
```

### 代码结构

```
src/
├── main.rs          # 主程序入口和更新逻辑
├── cli.rs           # 命令行参数解析
├── updater.rs       # 核心更新功能（下载、版本比较等）
└── conf/
    └── mod.rs       # 配置文件管理
```

## 优化改进

最近的优化包括：

- ✅ 模块化代码结构，提高可维护性
- ✅ 完善的错误处理，避免 panic
- ✅ 丰富的 CLI 参数支持
- ✅ 彩色终端输出，更好的用户体验
- ✅ 详细的日志和进度显示
- ✅ 版本比较错误处理
- ✅ 找不到匹配资产时的友好提示
- ✅ 使用具体的依赖版本（非通配符）

## 依赖项

- `tokio`: 异步运行时
- `reqwest`: HTTP 客户端
- `serde`: 序列化/反序列化
- `config`: 配置管理
- `anyhow`: 错误处理
- `semver`: 版本比较
- `dirs`: 系统目录

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request！

## 作者

Serein-sz
