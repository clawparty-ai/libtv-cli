# libtv-cli

一个跨平台的 Rust CLI 工具，用于 **LiblibAI**（哩布哩布AI，liblib.art）的文生图和图生图。

`libtv-cli` 直接使用 HMAC-SHA1 签名认证，与官方 `openapi.liblibai.cloud` API 通信。支持异步生成任务提交、状态轮询和自动图片下载 —— 全部打包在一个静态二进制文件中，**零运行时依赖**。

---

## 快速开始

### 1. 获取 API 密钥

你需要 LiblibAI 提供的 **AccessKey** 和 **SecretKey**。这些通常通过商务合作渠道或测试账号获得。

### 2. 设置凭证

```bash
export LIBLIB_ACCESS_KEY="你的-access-key"
export LIBLIB_SECRET_KEY="你的-secret-key"
```

或者通过 CLI 参数传入（安全性较低，仅用于调试）：

```bash
./libtv-cli --access-key "xxx" --secret-key "yyy" text2img ...
```

### 3. 生成图片

```bash
./libtv-cli text2img \
  --prompt "一只可爱的猫漂浮在太空中，数字艺术，色彩丰富" \
  --template-uuid "bf085132c7134622895b783b520b39ff" \
  --width 512 --height 512 \
  -o output.png
```

你会看到如下输出：

```
🎨 发起文生图任务...
   Prompt: 一只可爱的猫漂浮在太空中，数字艺术，色彩丰富
   Template: bf085132c7134622895b783b520b39ff
   Size: 512x512
✅ 任务已提交，generateUuid=81f7bcbf85b64c888a9a31e791d34266
⏳ 等待生成完成...
  [81f7bcbf...] status=RUNNING, progress=1%
  [81f7bcbf...] status=RUNNING, progress=36%
  [81f7bcbf...] status=RUNNING, progress=100%
📥 下载 1 张图片...
💾 已保存: /.../output.png
```

---

## 从源码编译

### 前置要求

- [Rust](https://rustup.rs/) 1.70+（在 1.94 上测试通过）

### 编译

```bash
cd libtv-cli
cargo build --release
```

编译产物位于：

```
target/release/libtv-cli
```

### 为 Windows 跨平台编译

在 macOS/Linux 上安装对应 target 后编译：

```bash
# 安装 Windows target（仅需一次）
rustup target add x86_64-pc-windows-gnu

# 编译
cargo build --release --target x86_64-pc-windows-gnu
```

Windows 可执行文件位于：

```
target/x86_64-pc-windows-gnu/release/libtv-cli.exe
```

> **提示：** 在 Windows 上，直接双击 `libtv-cli.exe` 或在命令提示符 / PowerShell 中运行即可。**不需要安装 Python 或其他运行时。**

---

## CLI 使用详解

### 全局选项

| 选项 | 说明 |
|------|------|
| `--access-key <KEY>` | LiblibAI AccessKey（或环境变量 `LIBLIB_ACCESS_KEY`） |
| `--secret-key <KEY>` | LiblibAI SecretKey（或环境变量 `LIBLIB_SECRET_KEY`） |

### `text2img` — 文生图

```bash
libtv-cli text2img \
  --prompt "你的提示词" \
  --template-uuid "你的模板UUID" \
  [其他选项]
```

| 选项 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--prompt` | `-p` | *(必填)* | 正向提示词，描述你想要的画面 |
| `--template-uuid` | `-t` | *(必填)* | 生成模板 UUID |
| `--checkpoint-id` | `-c` | — | 基础模型 / Checkpoint ID |
| `--negative-prompt` | `-n` | — | 负向提示词（不想出现的内容） |
| `--width` | `-W` | `512` | 图片宽度（像素） |
| `--height` | `-H` | `512` | 图片高度（像素） |
| `--steps` | `-s` | `20` | 采样步数 |
| `--cfg-scale` | — | `7.0` | CFG Scale（提示词引导强度） |
| `--seed` | — | `-1` | 随机种子（`-1` 表示随机） |
| `--img-count` | `-N` | `1` | 生成图片数量（1–4） |
| `--output` | `-o` | `generated.png` | 输出文件路径 |
| `--poll-interval` | — | `3` | 轮询间隔（秒） |
| `--max-wait` | — | `300` | 最大等待时间（秒） |

**示例 — 完整参数：**

```bash
./libtv-cli text2img \
  --prompt "未来主义城市夜景，霓虹灯光，电影级画面" \
  --template-uuid "5d7e67009b344550bc1aa6ccbfa1d7f4" \
  --checkpoint-id "你的模型ID" \
  --negative-prompt "模糊、低质量、变形" \
  --width 768 --height 1024 \
  --steps 25 --cfg-scale 8.0 \
  --seed 42 \
  --img-count 1 \
  -o cityscape.png
```

**示例 — 快速生成（别名命令）：**

```bash
./libtv-cli generate \
  --prompt "日落时分的山间风景" \
  --template-uuid "bf085132c7134622895b783b520b39ff" \
  -o sunset.png
```

> `generate` 是 `text2img` 的快捷别名。

### `img2img` — 图生图

```bash
libtv-cli img2img \
  --prompt "把它变成赛博朋克风格" \
  --template-uuid "你的模板UUID" \
  --input "input.png" \
  --denoising 0.75 \
  -o output.png
```

| 选项 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--prompt` | `-p` | *(必填)* | 正向提示词 |
| `--template-uuid` | `-t` | *(必填)* | 模板 UUID |
| `--input` | `-i` | *(必填)* | 输入图片路径或 URL |
| `--denoising` | `-d` | `0.75` | 去噪强度（0.0 – 1.0） |
| `--checkpoint-id` | `-c` | — | Checkpoint ID |
| `--negative-prompt` | `-n` | — | 负向提示词 |
| `--width` | `-W` | `512` | 输出宽度 |
| `--height` | `-H` | `512` | 输出高度 |
| `--steps` | `-s` | `20` | 采样步数 |
| `--cfg-scale` | — | `7.0` | CFG Scale |
| `--seed` | — | `-1` | 随机种子 |
| `--output` | `-o` | `generated.png` | 输出文件路径 |

### `status` — 查询生成任务状态

```bash
libtv-cli status --uuid "81f7bcbf85b64c888a9a31e791d34266"
```

输出示例：

```
GenerateUuid:   81f7bcbf85b64c888a9a31e791d34266
Status:         SUCCESS
Progress:       100%
Images:
  [1] https://liblibai-tmp-image.liblib.cloud/img/... (512x512)
```

---

## 已知模板 UUID

模板是 LiblibAI 上预配置的生成工作流。以下是社区整理的已知 UUID：

| 能力 | 模板 UUID |
|------|-----------|
| Star-3 Alpha 文生图 | `5d7e67009b344550bc1aa6ccbfa1d7f4` |
| Star-3 Alpha 图生图 | `07e00af4fc464c7ab55ff906f8acf1b7` |
| Qwen Image 文生图 | `bf085132c7134622895b783b520b39ff` |
| Kontext 文生图 | `fe9928fde1b4491c9b360dd24aa2b115` |
| Kontext 图生图 | `1c0a9712b3d84e1b8a9f49514a46d88c` |
| IMG1 生成 | `86c58ea26e9a45bd9f562c6306c17c0f` |
| IMG1 局部重绘 | `0fb3ddb15a094e74b1241fbda5db3199` |
| LibDream 文生图 | `aa835a39c1a14cfca47c6fc941137c51` |
| LibEdit 图编辑 | `cd3a6751086b4483ba5f0523aef53a79` |
| Kling 文生视频 | `61cd8b60d340404394f2a545eeaf197a` |
| Kling 图生视频 | `180f33c6748041b48593030156d2a71d` |

> 以上 UUID 来自社区项目 `mrknow001/LiblibAI-mcp`，实际可用性取决于你的账号权限。

---

## 技术细节

### 认证方式

LiblibAI 使用 **HMAC-SHA1** 签名：

```
1. timestamp = 当前 Unix 时间戳（毫秒）
2. nonce = 16 位随机字母数字字符串
3. string_to_sign = endpoint + "&" + timestamp + "&" + nonce
4. signature = base64.urlsafe_b64encode( HMAC-SHA1(string_to_sign, SecretKey) )
```

签名作为 URL 查询参数附加：

```
?AccessKey=xxx&Signature=yyy&Timestamp=zzz&SignatureNonce=www
```

### API 端点

| 操作 | 端点 |
|------|------|
| 文生图 | `POST /api/generate/webui/text2img` |
| 图生图 | `POST /api/generate/webui/img2img` |
| 查询状态 | `POST /api/generate/webui/status` |

### 轮询行为

- 默认每 `3` 秒轮询一次（可通过 `--poll-interval` 调整）
- 默认最大等待 `300` 秒（可通过 `--max-wait` 调整）
- 任务判定为**成功**的条件：
  - `generateStatus == 3`，**或**
  - `generateStatus == 5` 且 `images` 数组非空且 `percentCompleted >= 1.0`
- 任务判定为**失败**的条件：`generateStatus == 4`

> **注意：** liblib 的实际 API 与社区 SDK 文档存在差异。`generateStatus` 的 `5` 表示一种“成功但状态编码不同”的情况，`libtv-cli` 已针对此做了适配。

---

## 许可证

MIT
