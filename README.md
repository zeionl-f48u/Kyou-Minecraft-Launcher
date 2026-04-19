## 🛠 技术栈

| 角色 | 技术 |
|:---|:---|
| 桌面框架 | [Tauri 2.0](https://tauri.app/) (Rust) |
| 前端框架 | [Vue 3](https://vuejs.org/) (Composition API + `<script setup>`) |
| 类型系统 | [TypeScript](https://www.typescriptlang.org/) |
| 构建工具 | [Vite](https://vitejs.dev/) |
| 包管理器 | [pnpm](https://pnpm.io/) |
| UI 图标 | [lucide-vue-next](https://lucide.dev/) |
| 代码规范 | ESLint + Prettier |

---

## 🚀 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) 18+ 
- [pnpm](https://pnpm.io/installation) 8+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+ 与 Cargo
- 平台构建依赖（详见 [Tauri 文档](https://v2.tauri.app/start/prerequisites/)）

### 克隆项目

```bash
git clone https://github.com/yourname/kyou-launcher.git
cd kyou-launcher
```

### 安装依赖

```bash
pnpm install
```

> ⚠️ 首次安装时 pnpm 可能会提示 `Ignored build scripts: esbuild`。请运行 `pnpm approve-builds`，在交互菜单中勾选 esbuild 后回车确认。

### 开发模式

启动 Tauri 开发窗口（支持热重载）：

```bash
pnpm tauri dev
```

### 生产构建

```bash
pnpm tauri build
```

构建产物将位于 `src-tauri/target/release/` 目录。

---

## 🧑‍💻 开发指南

### 项目结构

```
kyou-launcher/
├── src/                      # Vue 前端源码
│   ├── App.vue               # 主界面组件
│   ├── main.ts               # 应用入口
│   └── ...
├── src-tauri/                # Tauri 后端 (Rust)
│   ├── src/
│   │   └── main.rs           # Rust 主入口
│   ├── Cargo.toml            # Rust 依赖配置
│   └── tauri.conf.json       # Tauri 配置文件
├── public/                   # 静态资源
├── index.html
├── vite.config.ts
├── tsconfig.json
└── package.json
```

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/main.rs` 中定义 Rust 函数并添加 `#[tauri::command]` 宏。
2. 在 `.invoke_handler()` 中注册该命令。
3. 在前端通过 `@tauri-apps/api` 的 `invoke()` 调用。

示例：
```rust
#[tauri::command]
fn get_versions() -> Vec<String> {
    vec!["1.20.4".to_string(), "1.21".to_string()]
}
```

```typescript
import { invoke } from '@tauri-apps/api/tauri'

const versions = await invoke<string[]>('get_versions')
```

### 代码风格

- 使用 `pnpm lint` 检查代码规范。
- 使用 `pnpm format` 自动格式化代码。

---

## 🤝 贡献

我们非常欢迎任何形式的贡献！无论是报告 Bug、提出新功能、改进文档，还是提交代码，都是对 Kyou 的巨大帮助。

### 贡献流程

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

请确保你的代码遵循项目的代码规范，并尽量提供清晰的提交说明。

### 待办事项 (Roadmap)

- [ ] 支持 Modrinth / CurseForge 整合包下载
- [ ] 内置资源包/光影包管理
- [ ] 多国语言国际化 (i18n)
- [ ] 自动更新功能

---

## 📜 开源协议

Kyou Minecraft Launcher 采用 **Apache License 2.0** 开源协议。

### 核心要点

- ✅ **商业友好**：你可以自由地将本软件用于商业目的。
- ✅ **专利保护**：协议明确授予专利权，防止贡献者事后主张专利侵权。
- ✅ **允许闭源**：你可以修改源代码并发布闭源的衍生作品，无需强制开源。
- ℹ️ **保留声明**：分发时必须保留原始版权声明、许可声明和免责条款。
- ℹ️ **修改说明**：若对代码有重大修改，需在源文件中加以说明。

完整协议文本请查看 [LICENSE](./LICENSE) 文件，或访问 [Apache 2.0 官网](http://www.apache.org/licenses/LICENSE-2.0)。

---

## 🙏 致谢

Kyou Launcher 的诞生离不开以下优秀开源项目的启发与支持：

- [Tauri](https://tauri.app/) - 轻量、安全的跨平台桌面框架
- [Vue](https://vuejs.org/) - 渐进式 JavaScript 框架
- [HMCL](https://github.com/HMCL-dev/HMCL) 与 [Prism Launcher](https://prismlauncher.org/) - Minecraft 启动器社区的标杆
- [CreeperHost](https://minotar.net/) - 提供的 Minecraft 头像 API

---

<div align="center">
  <p>✨ 镜中世界，即刻启程 ✨</p>
  <p>Made with ❤️ by Kyou Team</p>
</div>