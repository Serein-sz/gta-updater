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

```bash
# 检查并更新所有配置的应用
gta-updater

# 使用环境变量配置
CPA__GITHUB_OWNER=your-org CPA__GLOBAL_PATH=/path/to/bin gta-updater
```

### 环境变量

所有配置项都可以通过环境变量覆盖，使用 `CPA__` 前缀：

```bash
CPA__GITHUB_OWNER=Serein-sz
CPA__GLOBAL_PATH=/usr/local/bin
CPA__APPS__0__NAME=body-recorder
CPA__APPS__0__VERSION=v0.1.0
```

## 工作原理

1. 读取配置文件，获取要管理的应用列表
2. 通过 GitHub API 查询每个应用的最新 release
3. 根据当前系统的操作系统和架构，筛选匹配的资产文件
4. 下载并安装到指定路径

## 开发

### 运行测试

```bash
cargo test
```

### 代码结构

```
src/
├── main.rs          # 主程序逻辑
└── conf/
    └── mod.rs       # 配置管理模块
```

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
