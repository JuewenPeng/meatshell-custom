# meatshell-custom

**简体中文** | [English](./README.en.md)

一个轻量级、低内存占用的 SSH / 终端客户端，灵感来自 FinalShell，但完全由
**Rust + [Slint](https://slint.dev)** 实现。目标是保留 FinalShell 的核心体验
（资源监控侧栏、会话管理、多标签页终端）的同时，把内存占用从 400 MB+ 的
JVM 压到几十 MB 原生级别。

## Custom 版本说明

本分支基于作者原版 meatshell 做了一些定制改动，包含功能增强、交互优化和问题修复；README 其余内容尽量保持与上游原版一致。

### 本分支变更与改进

- 新增“江南”浅色主题，并为顶栏、收纳栏、面板、悬浮状态、选中状态和 SFTP 斑马纹提供独立配色。
- SSH 连接支持 X11 forwarding。
- 终端增加 `Ctrl+C` 复制偏好设置，可按使用习惯切换复制 / 中断行为。
- 会话列表支持在同一分组内拖拽排序。
- 快速连接和运行状态面板支持停靠到左 / 右 / 上 / 下；两个面板同侧展开时共享停靠区域，边缘图标并排显示，可随时点击切换或收起对应面板。
- SFTP 支持右键或工具栏移动文件 / 文件夹，并提供可视化目标目录选择窗口；多选文件 / 文件夹时可批量移动。
- SFTP 目录树支持右键菜单、宽度拖拽和大目录 `加载全部`；移动窗口可自动定位到当前路径。
- SFTP 文件列表支持 Windows 风格多选：`Ctrl` / `Shift` / `Ctrl+A`，并在底部状态栏显示选中数量。
- SFTP 右键菜单支持“下载”和“打包下载”，并支持对多选文件 / 文件夹批量下载、批量移动、批量删除。
- SFTP 大目录支持分页初始显示和 `加载全部`，文件列表、目录树和移动窗口目录树均使用虚拟列表，降低大文件夹打开与滚动压力。
- SFTP 移动窗口的目录展开 / 收起与主目录树相互独立，选择目标目录不会同步改变主面板目录树状态。
- SFTP 目录列表增加会话内缓存，最多保留 64 个最近访问目录；刷新或文件操作后会更新缓存。
- SFTP 文件查看 / 外部编辑的临时下载支持取消；成功传输后自动关闭下载窗口并清理已完成记录。
- 新建 / 编辑会话、SFTP 相关弹窗支持 `Esc` 快速关闭；移动、重命名、权限调整等弹窗可拖动调整位置。
- 优化 SFTP 重命名弹窗的默认聚焦与光标位置。
- 优化路径框、重命名、移动、新建会话等文本输入框的光标跟随行为；内容超出输入框宽度时会自动滚动到光标位置，长路径默认显示末尾。
- 修复默认收起设置的启动状态逻辑，避免与已保存的面板状态冲突。
- 优化 SFTP 文件列表刷新逻辑，减少刷新时滚动位置异常变化。
- 改进 tmux 环境下 SFTP 跟随 `cd` 的路径同步支持，并在顶部工具栏提供快捷开关。
- SFTP 顶部功能按钮增加悬浮提示，提升按钮含义可读性。
- 新增 SFTP 相关自定义文案的中英文切换支持。
- SFTP 文件列表新增可排序表头，支持按名称、大小、类型、修改时间、权限、用户/用户组排序，并保持文件夹与文件分组显示。
- SFTP 文件类型显示改为更接近系统文件管理器的格式，可识别常见源码、配置、日志、图片、音频、视频、压缩包、可执行文件和点文件，并支持中英文切换。
- SFTP 文件列表表头支持拖动调整列宽；左右停靠时表头文字靠左显示，超出可视区域的表头、文件行和工具栏内容会被面板边界裁切。
- 优化 SFTP 表头拖动手感，列宽压缩到最小后反向拖动可立即跟随鼠标，不再积累越界距离。
- SFTP 面板收起后隐藏不可用的路径框和文件操作按钮，并保留展开 / 收起功能。
- SFTP 面板拖拽停靠区域限制在路径栏左侧的拖拽手柄，避免误触文件列表或目录树空白区域。
- SFTP 面板新增“文件 / 端口转发”页签，可在已连接会话中创建运行时 `-L` 本地转发和 `-D` SOCKS5 动态转发，显示启动中 / 运行中 / 启动失败状态，并可停止或清除失败记录。
- SFTP 目录树刷新时保留手动滚动位置；进入当前可视范围外的目录会自动展开并将选中目录定位到可视区域偏上位置。
- SFTP 内容区支持鼠标侧键后退 / 前进：每个会话独立保存最近 20 条路径历史，普通目录跳转会清空前进历史，刷新不会写入历史。
- 大目录加载期间会合并连续积压的目录跳转和刷新请求，只执行最后一次，避免过期视图操作在加载完成后依次执行。

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
