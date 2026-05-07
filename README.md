# libtv-cli

A cross-platform Rust CLI tool for **LiblibAI** (liblib.art) text-to-image & image-to-image generation.

`libtv-cli` talks directly to the official `openapi.liblibai.cloud` API using HMAC-SHA1 signed requests. It supports asynchronous generation, status polling, and automatic image downloading — all in a single static binary with zero runtime dependencies.

---

## Quick Start

### 1. Get your API keys

You need an **AccessKey** and **SecretKey** from LiblibAI. These are usually provided via business/cooperation channels or testing accounts.

### 2. Set credentials

```bash
export LIBLIB_ACCESS_KEY="your-access-key"
export LIBLIB_SECRET_KEY="your-secret-key"
```

Or pass them as CLI flags (less secure, useful for debugging):

```bash
./libtv-cli --access-key "xxx" --secret-key "yyy" text2img ...
```

### 3. Generate an image

```bash
./libtv-cli text2img \
  --prompt "a cute cat floating in space, digital art, colorful" \
  --template-uuid "bf085132c7134622895b783b520b39ff" \
  --width 512 --height 512 \
  -o output.png
```

You will see output like:

```
🎨 Initiating text-to-image task...
   Prompt: a cute cat floating in space, digital art, colorful
   Template: bf085132c7134622895b783b520b39ff
   Size: 512x512
✅ Task submitted, generateUuid=81f7bcbf85b64c888a9a31e791d34266
⏳ Waiting for generation to complete...
  [81f7bcbf...] status=RUNNING, progress=1%
  [81f7bcbf...] status=RUNNING, progress=36%
  [81f7bcbf...] status=RUNNING, progress=100%
📥 Downloading 1 image(s)...
💾 Saved: /.../output.png
```

---

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ (tested on 1.94)

### Compile

```bash
cd libtv-cli
cargo build --release
```

The binary will be at:

```
target/release/libtv-cli
```

### Cross-compilation for Windows

On macOS/Linux with the appropriate target:

```bash
# Install Windows target (once)
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

The Windows executable:

```
target/x86_64-pc-windows-gnu/release/libtv-cli.exe
```

> **Tip:** On Windows, simply double-click `libtv-cli.exe` or run it from Command Prompt / PowerShell. No Python or other runtime is required.

---

## CLI Reference

### Global Options

| Option | Description |
|--------|-------------|
| `--access-key <KEY>` | LiblibAI AccessKey (or env `LIBLIB_ACCESS_KEY`) |
| `--secret-key <KEY>` | LiblibAI SecretKey (or env `LIBLIB_SECRET_KEY`) |

### `text2img` — Text to Image

```bash
libtv-cli text2img \
  --prompt "your prompt text" \
  --template-uuid "your-template-uuid" \
  [OPTIONS]
```

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--prompt` | `-p` | *(required)* | Positive prompt describing the desired image |
| `--template-uuid` | `-t` | *(required)* | Generation template UUID |
| `--checkpoint-id` | `-c` | — | Base model / Checkpoint ID |
| `--negative-prompt` | `-n` | — | Negative prompt (what to avoid) |
| `--width` | `-W` | `512` | Image width in pixels |
| `--height` | `-H` | `512` | Image height in pixels |
| `--steps` | `-s` | `20` | Sampling steps |
| `--cfg-scale` | — | `7.0` | CFG Scale (guidance scale) |
| `--seed` | — | `-1` | Random seed (`-1` = random) |
| `--img-count` | `-N` | `1` | Number of images to generate (1–4) |
| `--output` | `-o` | `generated.png` | Output file path |
| `--poll-interval` | — | `3` | Polling interval in seconds |
| `--max-wait` | — | `300` | Maximum wait time in seconds |

**Example — detailed parameters:**

```bash
./libtv-cli text2img \
  --prompt "a futuristic city at night, neon lights, cinematic" \
  --template-uuid "5d7e67009b344550bc1aa6ccbfa1d7f4" \
  --checkpoint-id "some-checkpoint-id" \
  --negative-prompt "blurry, low quality, distorted" \
  --width 768 --height 1024 \
  --steps 25 --cfg-scale 8.0 \
  --seed 42 \
  --img-count 1 \
  -o cityscape.png
```

**Example — quick generation (alias command):**

```bash
./libtv-cli generate \
  --prompt "mountain landscape at sunset" \
  --template-uuid "bf085132c7134622895b783b520b39ff" \
  -o sunset.png
```

> `generate` is a shortcut alias for `text2img`.

### `img2img` — Image to Image

```bash
libtv-cli img2img \
  --prompt "make it cyberpunk style" \
  --template-uuid "your-template-uuid" \
  --input "input.png" \
  --denoising 0.75 \
  -o output.png
```

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--prompt` | `-p` | *(required)* | Positive prompt |
| `--template-uuid` | `-t` | *(required)* | Template UUID |
| `--input` | `-i` | *(required)* | Input image path or URL |
| `--denoising` | `-d` | `0.75` | Denoising strength (0.0 – 1.0) |
| `--checkpoint-id` | `-c` | — | Checkpoint ID |
| `--negative-prompt` | `-n` | — | Negative prompt |
| `--width` | `-W` | `512` | Output width |
| `--height` | `-H` | `512` | Output height |
| `--steps` | `-s` | `20` | Sampling steps |
| `--cfg-scale` | — | `7.0` | CFG Scale |
| `--seed` | — | `-1` | Random seed |
| `--output` | `-o` | `generated.png` | Output file path |

### `status` — Query Generation Status

```bash
libtv-cli status --uuid "81f7bcbf85b64c888a9a31e791d34266"
```

Output:

```
GenerateUuid:   81f7bcbf85b64c888a9a31e791d34266
Status:         SUCCESS
Progress:       100%
Images:
  [1] https://liblibai-tmp-image.liblib.cloud/img/... (512x512)
```

---

## Known Template UUIDs

Templates are pre-configured generation workflows on LiblibAI. Here are some known UUIDs from the community:

| Capability | Template UUID |
|------------|---------------|
| Star-3 Alpha Text2Img | `5d7e67009b344550bc1aa6ccbfa1d7f4` |
| Star-3 Alpha Img2Img | `07e00af4fc464c7ab55ff906f8acf1b7` |
| Qwen Image Text2Img | `bf085132c7134622895b783b520b39ff` |
| Kontext Text2Img | `fe9928fde1b4491c9b360dd24aa2b115` |
| Kontext Img2Img | `1c0a9712b3d84e1b8a9f49514a46d88c` |
| IMG1 Generation | `86c58ea26e9a45bd9f562c6306c17c0f` |
| IMG1 Inpaint | `0fb3ddb15a094e74b1241fbda5db3199` |
| LibDream Text2Img | `aa835a39c1a14cfca47c6fc941137c51` |
| LibEdit Image Edit | `cd3a6751086b4483ba5f0523aef53a79` |
| Kling Text2Video | `61cd8b60d340404394f2a545eeaf197a` |
| Kling Img2Video | `180f33c6748041b48593030156d2a71d` |

> These UUIDs are provided by the community project `mrknow001/LiblibAI-mcp`. Availability may vary depending on your account permissions.

---

## Technical Details

### Authentication

LiblibAI uses **HMAC-SHA1** signatures:

```
1. timestamp = current Unix time in milliseconds
2. nonce = 16-character random alphanumeric string
3. string_to_sign = endpoint + "&" + timestamp + "&" + nonce
4. signature = base64.urlsafe_b64encode( HMAC-SHA1(string_to_sign, SecretKey) )
```

The signature is appended as query parameters:

```
?AccessKey=xxx&Signature=yyy&Timestamp=zzz&SignatureNonce=www
```

### API Endpoints

| Action | Endpoint |
|--------|----------|
| Text to Image | `POST /api/generate/webui/text2img` |
| Image to Image | `POST /api/generate/webui/img2img` |
| Query Status | `POST /api/generate/webui/status` |

### Polling Behavior

- Polls every `3` seconds by default (configurable via `--poll-interval`)
- Maximum wait time: `300` seconds (configurable via `--max-wait`)
- A task is considered **successful** when:
  - `generateStatus == 3`, **or**
  - `generateStatus == 5` with non-empty `images` array and `percentCompleted >= 1.0`
- A task is considered **failed** when `generateStatus == 4`

---

## License

MIT
