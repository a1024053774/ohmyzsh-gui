# ohmyzsh-gui

<p align="center">
  <a href="./README.md">English</a> ·
  <a href="./README.zh-CN.md">简体中文</a>
</p>

桌面端管理 Oh My Zsh 插件、主题和 `.zshrc`。每次修改都会先预览、备份，并通过 `zsh -n` 后再写入。

**1.0** · MIT · Tauri 2 · macOS / Windows / Linux

<p align="center">
  <img src="src-tauri/icons/128x128.png" width="72" alt="ohmyzsh-gui">
</p>

## 截图

概览

<img src="docs/screenshots/overview.png" width="920" alt="概览">

已安装插件

<img src="docs/screenshots/installed.png" width="920" alt="已安装插件">

在 GitHub 上发现

<img src="docs/screenshots/discover.png" width="920" alt="发现插件">

配置

<img src="docs/screenshots/configuration.png" width="920" alt="配置">

更新

<img src="docs/screenshots/updates.png" width="920" alt="插件更新">

设置

<img src="docs/screenshots/settings.png" width="920" alt="设置">

## 能做什么

- 读取真实的 `~/.zshrc`，只改已识别的 `ZSH_THEME` 和 `plugins=()`，其余内容原样保留
- 启用 / 停用官方 Oh My Zsh 插件
- 在发现页搜索并安装 GitHub 插件和主题（主题进入 `custom/themes`，不会误写入 `plugins=()`）
- 应用前先看 diff，确认后备份并做 zsh 语法检查
- 应用成功后打开一个**新终端**加载配置（已打开的窗口不会自动刷新）
- 在活动页查看备份并撤销
- 可选 GitHub Token，保存在系统钥匙串；不填也能匿名搜索
- 界面语言可跟随系统，或固定为英文 / 简体中文

## 安全

没有明确的「应用」操作时，应用不会写入 `.zshrc`。它会先备份，写入临时文件，运行 `zsh -n`，校验通过后才替换原文件。GitHub 来源仅接受 `owner/repository`。管理器本身不会执行插件代码。

## 安装

从 [Releases](https://github.com/a1024053774/ohmyzsh-gui/releases/tag/v1.0.0) 下载 **1.0.0**，按系统和芯片选文件：

| 平台 | 架构 | 文件 |
| --- | --- | --- |
| macOS | Apple Silicon | [ohmyzsh-gui_1.0.0_aarch64.dmg](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_aarch64.dmg) |
| macOS | Intel | [ohmyzsh-gui_1.0.0_x64.dmg](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_x64.dmg) |
| Linux | x86_64 | [AppImage](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_amd64.AppImage) · [deb](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_amd64.deb) · [rpm](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui-1.0.0-1.x86_64.rpm) |
| Linux | ARM64 | [AppImage](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_aarch64.AppImage) · [deb](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_arm64.deb) · [rpm](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui-1.0.0-1.aarch64.rpm) |
| Windows | x86_64 | [MSI](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_x64_en-US.msi) · [setup.exe](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_x64-setup.exe) |
| Windows | ARM64 | [MSI](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_arm64_en-US.msi) · [setup.exe](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/ohmyzsh-gui_1.0.0_arm64-setup.exe) |

SHA-256 校验文件：[SHA256SUMS.txt](https://github.com/a1024053774/ohmyzsh-gui/releases/download/v1.0.0/SHA256SUMS.txt)。

未签名的 macOS 包第一次需要右键 → 打开。Windows SmartScreen 也可能提示。

## 从源码运行

需要已安装 [Oh My Zsh](https://ohmyz.sh)、`git`、`zsh`，以及 [Rust](https://rustup.rs)。

```bash
cargo install tauri-cli --version "^2"
cargo tauri dev
```

打包本地应用：

```bash
cargo tauri build
```

产物在 `src-tauri/target/release/bundle/`。签名和公证需要额外的 Apple / 平台证书。

浏览器预览（只读，不会改你的 shell）：

```bash
npm run dev
```

然后打开 `http://localhost:1420`。

## 注意

- 正在运行的终端不会自动 `source ~/.zshrc`。看效果请用应用弹出的新窗口。
- 若 `~/.zshrc` 里还有 Powerlevel10k instant prompt 和 `source ~/.p10k.zsh`，只改 `ZSH_THEME` 不会换掉 p10k 提示符。
- `zsh-snap`、`zinit`、`antigen` 这类管理器不是 Oh My Zsh 插件，不能放进 `plugins=()`。

## 许可证

[MIT](LICENSE)
