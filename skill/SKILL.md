---
name: libtv-image-gen
description: Generate images using LiblibAI (liblib.art) API via the libtv-cli Rust tool. Supports text-to-image across platforms (Windows, macOS, Linux x86_64/ARM64) with HMAC-SHA1 authenticated API calls, async polling, and automatic download. No Python required.
---

# LiblibAI Image Generation

Generate images through the LiblibAI (哩布哩布AI) cloud platform using the `libtv-cli` Rust tool. This skill provides cross-platform image generation with zero runtime dependencies.

## Requirements

- `libtv-cli` binary must be available (pre-compiled for your platform)
- LiblibAI API credentials (AccessKey + SecretKey)

## Installation

1. Download the appropriate binary for your platform from GitHub releases:
   - Windows: `libtv-cli-windows-x86_64.zip`
   - macOS M1/M2: `libtv-cli-macos-m1.tar.gz`
   - Linux x86_64: `libtv-cli-linux-x86_64.tar.gz`
   - Linux ARM64: `libtv-cli-linux-arm64.tar.gz`

2. Extract and place `libtv-cli` binary in a directory on your PATH, or set `LIBTV_CLI_PATH`.

3. Set your API credentials:
   ```bash
   export LIBLIB_ACCESS_KEY="your-access-key"
   export LIBLIB_SECRET_KEY="your-secret-key"
   ```

## Usage

### As a Skill (Recommended for Agents)

Use this skill through the agent by describing the image you want:

```
"Generate an image of a cute cat in space"
```

The agent will automatically:
1. Choose an appropriate template UUID
2. Set optimal width/height
3. Call libtv-cli with your prompt
4. Save the generated image to the workspace directory
5. Report the file path

### Direct CLI Usage

```bash
# Text to image
libtv-cli text2img \
  --prompt "a cute cat floating in space, digital art" \
  --template-uuid "bf085132c7134622895b783b520b39ff" \
  --width 512 --height 512 \
  -o output.png

# Image to image
libtv-cli img2img \
  --prompt "make it cyberpunk style" \
  --template-uuid "07e00af4fc464c7ab55ff906f8acf1b7" \
  --input input.png \
  --denoising 0.75 \
  -o output.png

# Check generation status
libtv-cli status --uuid "your-generate-uuid"
```

## Available Templates

| Template | UUID | Description |
|----------|------|-------------|
| Qwen Image | `bf085132c7134622895b783b520b39ff` | General text-to-image |
| Star-3 Alpha | `5d7e67009b344550bc1aa6ccbfa1d7f4` | High quality generation |
| Kontext | `fe9928fde1b4491c9b360dd24aa2b115` | Context-aware generation |

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `--prompt` | (required) | Image description text |
| `--template-uuid` | (required) | Generation template UUID |
| `--checkpoint-id` | - | Base model ID |
| `--negative-prompt` | - | What to avoid in the image |
| `--width` | 512 | Image width (128-1536) |
| `--height` | 512 | Image height (128-1536) |
| `--steps` | 20 | Sampling steps (1-60) |
| `--cfg-scale` | 7.0 | Guidance scale (1.0-15.0) |
| `--seed` | -1 | Random seed (-1 for random) |
| `--img-count` | 1 | Number of images (1-4) |
| `--output` | `generated.png` | Output file path |

## Error Handling

- If API credentials are missing, you'll see: "Missing AccessKey/SecretKey"
- If network fails, check your internet connection and API endpoint reachability
- If generation fails (code=4), retry with different parameters or check your account balance

## Fallback

If LiblibAI service is unavailable, the agent can fall back to the `diffusion-image-gen` skill for local Stable Diffusion generation.
