# Installation Guide

## Quick Install (One-liner)

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/clawparty-ai/libtv-cli/main/install.sh | bash
```

After installation, add to your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"

# Set API credentials
export LIBLIB_ACCESS_KEY="your-access-key"
export LIBLIB_SECRET_KEY="your-secret-key"
```

### Windows (PowerShell)

```powershell
powershell -c "irm https://raw.githubusercontent.com/clawparty-ai/libtv-cli/main/install.ps1 | iex"
```

Add to PATH and set credentials:
```powershell
# Add to PATH (one-time)
$env:Path += ";$env:LOCALAPPDATA\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path, "User")

# Set API credentials (one-time)
[Environment]::SetEnvironmentVariable("LIBLIB_ACCESS_KEY", "your-access-key", "User")
[Environment]::SetEnvironmentVariable("LIBLIB_SECRET_KEY", "your-secret-key", "User")
```

---

## Manual Install (Pre-built Binary)

1. Go to [GitHub Releases](https://github.com/clawparty-ai/libtv-cli/releases)
2. Download the asset for your platform:
   - **Windows**: `libtv-cli-windows-x86_64.zip`
   - **macOS M1/M2**: `libtv-cli-macos-m1.tar.gz`
   - **Linux x86_64**: `libtv-cli-linux-x86_64.tar.gz`
   - **Linux ARM64**: `libtv-cli-linux-arm64.tar.gz`

3. Extract and move `libtv-cli` (or `libtv-cli.exe`) to a directory in your PATH.

4. Set API credentials as shown above.

5. Verify:
   ```bash
   libtv-cli --version
   ```

---

## Using the Skill in Agents

### Prerequisite

Your agent framework must:
1. Have a `skills/` directory (e.g., `~/.clawparty/skills/`)
2. Read `SKILL.md` files from that directory (ZeroClaw does this)
3. Support `bash` / `shell` tool execution

### Install the Skill

The one-liner installer above **automatically** installs the skill files to:
- **macOS/Linux**: `~/.clawparty/skills/libtv-image-gen/`
- **Windows**: `~\clawparty\skills\libtv-image-gen\`

If you prefer manual install, copy the `skill/` directory from this repo to your global skills directory.

### Enable in Agent

Open your agent's `workspace/SOUL.md` or `workspace/IDENTITY.md`, and add:

```markdown
## Image Generation Capability

- **Primary**: Use `libtv-image-gen` skill via `libtv-cli` for cloud-based image generation
- **Fallback**: Use `diffusion-image-gen` skill if libtv is unavailable
- **Usage**: Set LIBLIB_ACCESS_KEY and LIBLIB_SECRET_KEY environment variables,
  then call `/path/to/libtv-cli text2img --prompt "..." --output output.png`
```

Or simply use the wrapper scripts:
```bash
# macOS/Linux
~/.clawparty/skills/libtv-image-gen/generate_image.sh \
  --prompt "a cute cat in space" \
  --output workspace/cat.png

# Windows
%USERPROFILE%\.clawparty\skills\libtv-image-gen\generate_image.bat ^
  --prompt "a cute cat in space" ^
  --output workspace\cat.png
```

---

## Multi-Agent Setup (Shared Skill)

If you run multiple agents on the same machine, install the skill once to the shared skills directory:

```bash
# All agents will automatically pick it up
export SKILL_DIR="$HOME/.clawparty/skills"
```

The `install.sh` script defaults to this location. Each agent's `SOUL.md` only needs a one-line reference:

```markdown
Use `libtv-image-gen` skill for image generation when needed.
```

---

## Build from Source

See [README.md](README.md) for development setup and cargo build instructions.
