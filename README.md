# meatshell-custom

**简体中文** | [English](./README.en.md)

一个轻量级、低内存占用的 SSH / 终端客户端，灵感来自 FinalShell，但完全由
**Rust + [Slint](https://slint.dev)** 实现。目标是保留 FinalShell 的核心体验
（资源监控侧栏、会话管理、多标签页终端）的同时，把内存占用从 400 MB+ 的
JVM 压到几十 MB 原生级别。

## Custom 版本说明

本分支基于原作者的 [meatshell](https://github.com/jeff141/meatshell) 项目，做了一些定制改动，包含功能增强、交互优化和问题修复；README 其余内容尽量保持与上游原版一致。

### 本分支变更与改进

以下内容均为本分支相对于 `upstream/main` 的新增功能、交互改进和问题修复：

- 增加“江南”主题，并完善主题颜色 token；SFTP 斑马纹、悬浮状态、选中状态和危险操作均使用主题配色。
- 快速连接、运行状态和快捷指令面板统一支持四边停靠；同一边的面板共享停靠区域，通过侧边图标切换或收起，面板宽度和拖拽条保持一致。
- 快捷指令支持独立侧栏、分组管理、持久化设置和向所有会话广播；欢迎页与运行状态面板也可独立停靠。
- 增加远程 NVIDIA 多 GPU 显存监控（已用 / 总量）、System Information 窗口和中英文界面；资源面板隐藏时暂停采样。
- SSH 连接增加 X11 forwarding；本地 PowerShell / CMD 会话使用专用启动逻辑并禁用无效的 SFTP 控件。
- 增加可选的终端输出高亮（日志 / DevOps 预设和自定义规则）以及 Windows / macOS 渲染器选择。
- 增加终端 `Ctrl+C` 复制偏好、拖选自动复制、光标自动颜色、光标不透明度和未聚焦光标显示设置。
- 优化终端光标：输入或方向键操作后保持亮态并重新开始闪烁；修复拖选、tmux 和不同宽字符场景下的光标位置问题。
- 会话 Tab 支持同组拖拽排序、横向滚轮浏览、可拖动滚动条、标题宽度上限和始终可用的新标签页。
- SFTP 文件列表增加键盘操作（方向键、Backspace、Enter、Delete）、路径栏文件打开、确认删除和 Esc 取消；无效路径不会写入历史。
- SFTP 支持 Windows 风格多选、批量下载 / 移动 / 删除、打包下载、可视化移动目标目录和目录树右键菜单。
- SFTP 文件列表支持按名称、大小、类型、修改时间、权限和用户/用户组排序，并在进入目录、返回、刷新和重连后保持排序及选中状态。
- 每个会话保存最多 50 条 SFTP 路径历史，并恢复目录树展开、选中文件和滚动位置；目录列表缓存最多保留 64 个最近目录。
- SFTP 大目录使用分页、虚拟列表和请求合并，目录树支持加载全部 / 收起，降低大目录打开和连续导航的开销。
- SFTP 支持“文件 / 端口转发”页签，可创建、停止和清理运行时 `-L` 本地转发及 `-D` SOCKS5 动态转发。
- SFTP 侧面停靠时工具栏会按优先级自适应收缩和裁切，路径栏、上级目录、刷新及收起按钮保持正确位置；所有工具栏按钮提供悬浮提示。
- SFTP 支持 tmux 中跟随终端 `cd` 同步路径、会话间上传同步，以及中英文文件类型和界面文案。
- 新建 / 编辑会话及 SFTP 弹窗支持 Esc 关闭和拖动定位；输入框会自动跟随光标，长路径默认显示末尾。
- 终端滚动历史和 emoji 图像缓存均设置上限，避免长时间运行或大量输出造成缓存持续增长。

## 截图

<p align="center">
  <img src="docs/screenshots/01-welcome.png" alt="欢迎页 / 会话管理" width="800"><br>
  <em>欢迎页：会话管理 + 左侧本机资源监控</em>
</p>

<p align="center">
  <img src="docs/screenshots/02-terminal-htop.png" alt="终端 + SFTP" width="800"><br>
  <em>多标签页终端（htop 全屏渲染）+ 底部 SFTP 文件浏览 + 远端资源监控</em>
</p>

## 下载与安装

每次打 `v*` 标签，GitHub Actions 会自动构建 **Windows / Linux / macOS** 三平台二进制，
发布到 [Releases](https://github.com/jeff141/meatshell/releases) 页面。

### Windows

下载 `meatshell-*-windows-x86_64.zip`，解压后双击 `meatshell.exe`。

### Linux

```bash
tar -xzf meatshell-*-linux-x86_64.tar.gz
cd meatshell-*-linux-x86_64
./meatshell                                  # 直接运行
# 可选：装应用图标 + 启动器入口（Dock / 应用列表里显示图标，无需传参）
chmod +x install-linux.sh && ./install-linux.sh
```

> 需要 glibc ≥ 2.35（Ubuntu 22.04+ / Debian 12+）。Wayland 下首次装完图标可能要注销重登一次。

从源码 `cargo run`（Linux Mint / Ubuntu / Debian）需要先安装 Slint/winit/rfd 等用到的系统开发包：

```bash
sudo apt update
sudo apt install -y --no-install-recommends \
  build-essential pkg-config cmake \
  libfontconfig1-dev libfreetype6-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgtk-3-dev \
  libudev-dev
```

### macOS

下载得到的是 `.zip`，里面是 `meatshell.app` 应用程序包：

```bash
# 解压(aarch64 = Apple 芯片，x86_64 = Intel)
unzip meatshell-*-macos-*.zip
# 移到「应用程序」(可选，留在原地也行)
mv meatshell.app /Applications/
# 去掉「未签名应用」的隔离属性，否则会提示「meatshell 已损坏，无法打开」
xattr -dr com.apple.quarantine /Applications/meatshell.app
# 打开(或在「访达」里双击)
open /Applications/meatshell.app
```

> 若未移到 `/Applications`，把上面两条路径换成 `.app` 实际所在位置(如 `~/Downloads/meatshell.app`)即可。

> 从源码构建见下方 [运行](#运行)。

## 功能

### 已实现

- [x] FinalShell 风格 UI，深色 / 浅色 / 江南主题
- [x] 本机 + 远端资源监控（CPU / 内存 / 交换 / 网络 / 磁盘）
- [x] 远端进程监控（按 CPU 排序、PID 复制与权限确认后结束进程）
- [x] 完整 VT/ANSI 终端模拟（btop / htop / vim 全屏正常渲染）
- [x] 彩色 emoji（支持肤色、旗帜及 ZWJ 组合序列）
- [x] 多标签页（欢迎页 + 多个会话）
- [x] 会话管理：新建 / 编辑 / 删除 / 分组，本地 JSON 持久化，导出 / 导入
  - 配置位置：`%APPDATA%/meatshell/sessions.json`（Windows）
    / `~/.config/meatshell/sessions.json`（Linux）
    / `~/Library/Application Support/meatshell/sessions.json`（macOS）
- [x] SSH（`russh`，纯 Rust）：密码 / 私钥 / 加密私钥（密码短语）
- [x] SFTP 文件浏览 + 上传 / 下载（拖拽）+ 终端内 ZMODEM（`sz`）接收
- [x] SSH 端口转发 / 隧道：本地 -L / 远程 -R / 动态 -D（SOCKS5）
- [x] 快捷命令 + 命令输入框（可群发到所有会话）+ 命令历史
- [x] 串口 / Telnet 会话
- [x] 出站代理（SOCKS5 / HTTP）
- [x] 导入 `~/.ssh/config`
- [x] 会话密码加密存储（ChaCha20-Poly1305）
- [x] 已知主机（`known_hosts`）校验 + 首次连接确认
- [x] 多标签页终端分屏

彩色 emoji 图形来自 [Twemoji](https://github.com/jdecked/twemoji)，按
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) 使用；完整署名见
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

### 计划中

- [ ] 会话密码改用 OS 钥匙串存储

## 技术栈

| 模块          | 选型                                                              |
| ------------- | ----------------------------------------------------------------- |
| UI            | [Slint](https://slint.dev)（纯 Rust 编译，无 GC）                 |
| 异步运行时    | [`tokio`](https://tokio.rs)                                       |
| SSH 协议      | [`russh`](https://crates.io/crates/russh)（无 libssh 依赖）       |
| 系统指标      | [`sysinfo`](https://crates.io/crates/sysinfo)                     |
| 序列化        | `serde` + `serde_json`                                            |
| 日志          | `tracing` + `tracing-subscriber`                                  |

## 运行

```bash
cargo run --release
```

首次启动会在 `%APPDATA%/meatshell/sessions.json` 建立空的会话库。点击右上
角 **“＋ 新建会话”** 添加第一台服务器。

## 项目布局

```
meatshell/
├── Cargo.toml
├── build.rs                 # Slint 编译器入口
├── ui/
│   ├── app.slint            # 顶层窗口
│   ├── theme.slint          # 设计 tokens
│   ├── widgets.slint        # 可复用按钮 / 输入框 / sparkline
│   ├── sidebar.slint        # 左侧系统监控面板
│   ├── tabs.slint           # 顶部标签栏
│   ├── welcome.slint        # 欢迎页 / 快速连接
│   ├── session_dialog.slint # 新建 / 编辑会话弹框
│   └── terminal_view.slint  # 终端视图（v0.1 行缓冲）
└── src/
    ├── main.rs
    ├── app.rs               # UI ↔ 后端桥接
    ├── config.rs            # 会话 JSON 持久化
    ├── system.rs            # CPU / 内存 / 网络采样
    └── ssh.rs               # SSH 会话 worker
```

## 开发提示

- Slint 控件有非常严格的布局 DSL，改 `.slint` 后 `cargo check` 是最快的
  反馈方式。
- 应用事件循环是单线程（Slint 要求），所有跨线程 UI 更新通过
  `slint::invoke_from_event_loop` 回调。
- SSH / SFTP 共享 `known_hosts` 校验逻辑：首次连接会确认并记住主机密钥，
  后续密钥变化会再次提示。

## 发版

不要直接手动修改 `Cargo.toml` 后再打标签。使用发布脚本，让 Git tag 指向的提交本身就已经包含正确版本号：

```powershell
.\scripts\release.ps1 v0.6.0 -Push
```

脚本会更新 `Cargo.toml` / `Cargo.lock`，运行 `cargo check --locked`，验证 `meatshell --version`，提交 `Release v0.6.0`，创建 annotated tag，并推送当前分支和 tag。更多细节见 [docs/release.md](docs/release.md)。

## 相关群组

<p align="center">
  <img src="docs/QR/QQ_Group_QR_Code.jpg" alt="QQ群二维码" width="300"><br>
  <em>扫描二维码加入 QQ 群，与其他用户交流使用经验、反馈问题或获取最新动态</em>
</p>

## License

MIT OR Apache-2.0（双许可）。
