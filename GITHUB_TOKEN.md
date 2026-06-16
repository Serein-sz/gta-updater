# GitHub Token 配置指南

## 为什么需要 GitHub Token？

GitHub API 对未认证的请求有速率限制：
- **未认证**: 每小时 60 次请求
- **使用 Token**: 每小时 5000 次请求

如果你经常使用本工具或管理多个应用，建议配置 GitHub Token。

## 如何创建 GitHub Token

1. 访问 GitHub Settings: https://github.com/settings/tokens
2. 点击 "Generate new token" → "Generate new token (classic)"
3. 填写信息：
   - **Note**: 给 token 起个名字，如 "gta-updater"
   - **Expiration**: 选择过期时间（建议选择 "No expiration" 或更长时间）
   - **Scopes**: 勾选 `public_repo`（用于访问公共仓库）
4. 点击 "Generate token"
5. **重要**: 立即复制生成的 token（格式：`ghp_xxxxxxxxxxxx`），离开页面后将无法再次查看

## 配置方法

### 方法 1: 在配置文件中添加（推荐）

编辑 `~/.config/gta-updater/config.toml`（Windows: `%USERPROFILE%\.config\gta-updater\config.toml`）：

```toml
github_owner = "your-username"
github_token = "ghp_your_token_here"  # 添加这一行
global_path = "/usr/local/bin"

[[apps]]
name = "your-app"
version = "v0.1.0"
```

### 方法 2: 使用环境变量

```bash
# Linux/macOS
export GTA__GITHUB_TOKEN="ghp_your_token_here"

# Windows (PowerShell)
$env:GTA__GITHUB_TOKEN = "ghp_your_token_here"

# Windows (CMD)
set GTA__GITHUB_TOKEN=ghp_your_token_here
```

## 安全建议

1. **不要提交 token 到 Git 仓库**
   - 将包含 token 的配置文件添加到 `.gitignore`
   - 使用环境变量而不是硬编码

2. **定期轮换 token**
   - 建议每 3-6 个月更换一次 token

3. **最小权限原则**
   - 只授予 `public_repo` 权限
   - 不要使用具有更高权限的 token

4. **泄露处理**
   - 如果 token 泄露，立即在 GitHub 撤销该 token
   - 生成新的 token 替换

## 验证配置

运行以下命令测试配置是否生效：

```bash
gta --verbose
```

如果配置正确，工具将使用认证的 API 请求，不会遇到速率限制问题。

## 故障排除

### Token 无效

如果遇到 `401 Unauthorized` 错误：
- 检查 token 格式是否正确（应以 `ghp_` 开头）
- 确认 token 未过期
- 验证 token 具有 `public_repo` 权限

### 仍然遇到速率限制

- 确认配置文件路径正确
- 检查环境变量拼写（必须是 `GTA__GITHUB_TOKEN`，使用双下划线）
- 使用 `--verbose` 参数查看是否加载了 token
