# One-line installer for libtv-cli (Windows PowerShell)
# Usage: powershell -c "irm https://raw.githubusercontent.com/clawparty-ai/libtv-cli/main/install.ps1 | iex"

$ErrorActionPreference = "Stop"
$Repo = "clawparty-ai/libtv-cli"

function Write-Info ($msg)  { Write-Host "[INFO] $msg" -ForegroundColor Green }
function Write-Warn ($msg)  { Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Error($msg)  { Write-Host "[ERROR] $msg" -ForegroundColor Red }

# ── Detect platform ─────────────────────────
$Arch = switch ($env:PROCESSOR_ARCHITECTURE) {
    "AMD64" { "x86_64" }
    "ARM64" { "arm64" }
    "x86"   { "x86_64" }
    default {
        Write-Error "Unsupported architecture: $env:PROCESSOR_ARCHITECTURE"
        exit 1
    }
}

Write-Info "Detected platform: Windows $Arch"

if ($Arch -eq "arm64") {
    Write-Warn "ARM64 Windows: downloading x86_64 binary (may run via emulation)"
}

$AssetName = "libtv-cli-windows-x86_64"
$Ext = "zip"
$BinaryName = "libtv-cli.exe"

# ── Fetch latest release ────────────────────
Write-Info "Fetching latest release..."
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
try {
    $Release = Invoke-RestMethod -Uri $ApiUrl -UseBasicParsing
    $Latest = $Release.tag_name
} catch {
    Write-Error "Failed to fetch latest release: $_"
    exit 1
}

if (-not $Latest) {
    Write-Error "Could not determine latest version"
    exit 1
}

Write-Info "Latest version: $Latest"

# ── Download ────────────────────────────────
$TmpDir = Join-Path $env:TEMP ("libtv-cli-install-" + [System.Guid]::NewGuid().ToString().Substring(0,8))
New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null

$AssetUrl = "https://github.com/$Repo/releases/download/$Latest/$AssetName.$Ext"
$ArchivePath = Join-Path $TmpDir "archive.$Ext"

Write-Info "Downloading $AssetName.$Ext..."
try {
    Invoke-WebRequest -Uri $AssetUrl -OutFile $ArchivePath -UseBasicParsing
} catch {
    Write-Error "Download failed: $_"
    exit 1
}

# ── Extract ─────────────────────────────────
Write-Info "Extracting..."
Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

$ExtractedDir = Join-Path $TmpDir $AssetName
if (-not (Test-Path $ExtractedDir)) {
    Write-Error "Extraction failed: expected directory not found"
    exit 1
}

# ── Install binary ──────────────────────────
$InstallBin = if ($PSCommandPath) {
    # Try Program Files / local app data
    $LocalBin = Join-Path $env:LOCALAPPDATA "bin"
    $LocalBin
} else {
    Join-Path $env:USERPROFILE "bin"
}

if (-not (Test-Path $InstallBin)) {
    New-Item -ItemType Directory -Path $InstallBin -Force | Out-Null
}

Write-Info "Installing binary to $InstallBin"
Copy-Item -Path (Join-Path $ExtractedDir $BinaryName) -Destination (Join-Path $InstallBin $BinaryName) -Force

# ── Install skill files ─────────────────────
$DefaultSkillDir = Join-Path $env:USERPROFILE ".clawparty\skills\libtv-image-gen"
$SkillDir = if ($env:SKILL_DIR) { $env:SKILL_DIR } else { $DefaultSkillDir }

Write-Info "Installing skill to $SkillDir"
New-Item -ItemType Directory -Path $SkillDir -Force | Out-Null

$BaseRaw = "https://raw.githubusercontent.com/$Repo/main/skill"

Invoke-WebRequest -Uri "$BaseRaw/SKILL.md"            -OutFile (Join-Path $SkillDir "SKILL.md")           -UseBasicParsing
Invoke-WebRequest -Uri "$BaseRaw/generate_image.sh"   -OutFile (Join-Path $SkillDir "generate_image.sh")  -UseBasicParsing
Invoke-WebRequest -Uri "$BaseRaw/generate_image.bat"  -OutFile (Join-Path $SkillDir "generate_image.bat") -UseBasicParsing

# ── Summary ─────────────────────────────────
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Installation Complete!" -ForegroundColor Green -BackgroundColor Black
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Binary:     " -NoNewline; Write-Host "$InstallBin\$BinaryName" -ForegroundColor Cyan
Write-Host "Skill:      " -NoNewline; Write-Host "$SkillDir" -ForegroundColor Cyan
Write-Host "Version:    " -NoNewline; Write-Host "$Latest" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host "1. Add the install directory to PATH (if not already):"
Write-Host "   [Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';$InstallBin', 'User')"
Write-Host ""
Write-Host "2. Set your LiblibAI API credentials:"
Write-Host "   [Environment]::SetEnvironmentVariable('LIBLIB_ACCESS_KEY', 'your-access-key', 'User')"
Write-Host "   [Environment]::SetEnvironmentVariable('LIBLIB_SECRET_KEY', 'your-secret-key', 'User')"
Write-Host ""
Write-Host "3. Restart PowerShell, then verify:"
Write-Host "   $BinaryName --version" -ForegroundColor Cyan
Write-Host ""
Write-Host "4. Generate your first image:"
Write-Host "   $BinaryName text2img `" -ForegroundColor Cyan
Write-Host '     --prompt "a cute cat in space" `' -ForegroundColor Cyan
Write-Host '     --template-uuid "bf085132c7134622895b783b520b39ff" `' -ForegroundColor Cyan
Write-Host '     -o cat.png' -ForegroundColor Cyan
Write-Host ""
Write-Host "For agents: add this to SOUL.md / IDENTITY.md:"
Write-Host '   "Image generation via libtv-image-gen skill"' -ForegroundColor Gray
Write-Host "========================================" -ForegroundColor Green

# ── Cleanup ─────────────────────────────────
Remove-Item -Recurse -Force $TmpDir
