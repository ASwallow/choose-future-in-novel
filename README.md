# 互动故事引擎 (Choose Future In Novel)

一个数据驱动的互动小说阅读器，基于 Tauri (Rust + WebView) 构建。启动时自动扫描故事文件，选择后即可阅读。

## 功能特性

- 数据驱动：故事以 JSON 格式定义，支持分支选项、条件判断和状态标记
- 无后端依赖：纯前端渲染，Tauri 打包为原生桌面应用
- 自定义外观：支持更换封面背景和阅读背景（颜色/图片），文字颜色自动反色
- 内置 + 外置故事：EXE 内置故事文件，也可在同级 `stories/` 目录添加更多故事
- 液态玻璃 UI：按钮和界面采用毛玻璃风格

## 项目结构

```
├── src/                        # 前端源码
│   ├── index.html              # 主页面
│   └── styles.css              # 样式
├── src-tauri/                  # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/                  # 应用图标
│   ├── stories/                # 内置故事（打包进 EXE）
│   └── src/main.rs             # Rust 后端逻辑
├── package.json
└── README.md
```

## 环境准备

### 1. 安装 Node.js
下载安装：https://nodejs.org （选 LTS 版本）

### 2. 安装 Rust 编译器
```powershell
Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe
.\rustup-init.exe
```
装完后**重启终端**。

> **国内镜像加速：** 在环境变量中添加：
> ```
> RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
> RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
> ```

### 3. WebView2
Win11 自带，Win10 需安装：https://developer.microsoft.com/en-us/microsoft-edge/webview2/

## 运行与打包

```powershell
npm install          # 安装依赖
npm run dev          # 开发模式运行
npm run build        # 正式打包（首次约 5-10 分钟）
```

打包产物在 `src-tauri/target/release/bundle/nsis/` 下。

## 添加故事

### 内置故事（打包进 EXE）

将 `.json` 故事文件放入 `src-tauri/stories/` 目录，重新打包即可。

### 外置故事（无需重新打包）

将 `.json` 故事文件放入 EXE 同级目录下的 `stories/` 文件夹中。

## JSON 故事格式说明

每个故事是一个独立的 JSON 文件。

### 顶层结构

```json
{
  "meta": {
    "title": "故事标题",
    "author": "作者名",
    "version": "1.0.0"
  },
  "startNode": "起始节点ID",
  "nodes": { ... }
}
```

### 节点对象

```json
{
  "content": "展示给读者的剧情文本。",
  "choices": [
    {
      "text": "按钮文字",
      "nextNode": "目标节点ID",
      "conditions": { "标记名": true },
      "setFlags": { "另一个标记": true }
    }
  ]
}
```

| 字段         | 类型   | 必填 | 说明                                   |
|--------------|--------|------|----------------------------------------|
| `content`    | 字符串 | 是   | 当前节点展示的故事文本                  |
| `choices`    | 数组   | 是   | 选项数组，为空时表示结局                |
| `text`       | 字符串 | 是   | 按钮上显示的文字                        |
| `nextNode`   | 字符串 | 是   | 点击后跳转的目标节点 ID                 |
| `conditions` | 对象   | 否   | 前置条件：所有标记匹配时才显示该选项    |
| `setFlags`   | 对象   | 否   | 点击后设置的状态标记                    |

### `conditions` 与 `setFlags`

**`setFlags`** — 点击选项时设置全局状态标记：

```json
{ "text": "拾取钥匙", "nextNode": "door_room", "setFlags": { "has_key": true } }
```

**`conditions`** — 仅当标记匹配时才显示该选项：

```json
{ "text": "打开门锁", "nextNode": "treasure_room", "conditions": { "has_key": true } }
```

**组合使用：**

```json
{
  "text": "进入密道",
  "nextNode": "secret_room",
  "conditions": { "read_diary": true },
  "setFlags": { "found_passage": true }
}
```

### 结局

`choices` 为空数组的节点为结局节点，引擎显示「故事结束。」。

## 已知问题 (TODO)

- [ ] **窗口无法拖动**：自定义标题栏的 `data-tauri-drag-region` 属性在当前 Tauri v2 版本下无法正常工作，窗口位置移动功能暂不可用
- [ ] **更换应用图标**：准备一张 1024x1024 的 PNG 图标，替换 `src-tauri/icons/icon.png`，然后重新打包

## 法律免责声明

本项目**仅提供引擎代码和格式规范**，不包含任何受版权保护的故事内容。

**用户对自己创建和分发的内容承担全部责任。** 如果你使用第三方版权内容（例如来自已出版的小说、电影、游戏、动漫或其他媒体的故事）制作故事模板并公开分发，你需要自行承担因此产生的版权侵权风险和法律责任。

**建议：**
- 使用自己原创的内容
- 使用公有领域作品（例如经典童话、民间传说等）
- 在使用他人内容前，务必获得版权所有者的明确授权

本引擎的开发者不对用户如何使用本引擎承担任何责任。

## 开源许可

本项目为开源项目。引擎代码以"按原样"方式提供，仅用于教育和创意目的。
