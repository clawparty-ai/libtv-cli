@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

:: Skill wrapper for libtv-cli image generation on Windows
:: Usage: generate_image.bat --prompt "your prompt" --output output.png

set "SCRIPT_DIR=%~dp0"
set "LIBTV_CLI="

:: Try to find libtv-cli binary
if defined LIBTV_CLI_PATH (
    set "LIBTV_CLI=%LIBTV_CLI_PATH%"
) else if exist "%SCRIPT_DIR%..\libtv-cli.exe" (
    set "LIBTV_CLI=%SCRIPT_DIR%..\libtv-cli.exe"
) else if exist "%SCRIPT_DIR%libtv-cli.exe" (
    set "LIBTV_CLI=%SCRIPT_DIR%libtv-cli.exe"
) else (
    where libtv-cli >nul 2>&1
    if !errorlevel! equ 0 (
        set "LIBTV_CLI=libtv-cli"
    ) else (
        echo Error: libtv-cli binary not found. Set LIBTV_CLI_PATH or place libtv-cli.exe in PATH. >&2
        exit /b 1
    )
)

:: Default values
set "PROMPT="
set "TEMPLATE_UUID=bf085132c7134622895b783b520b39ff"
set "CHECKPOINT_ID="
set "NEGATIVE_PROMPT="
set "WIDTH=512"
set "HEIGHT=512"
set "STEPS=20"
set "CFG_SCALE=7.0"
set "SEED=-1"
set "IMG_COUNT=1"
set "OUTPUT=generated.png"
set "ACCESS_KEY=%LIBLIB_ACCESS_KEY%"
set "SECRET_KEY=%LIBLIB_SECRET_KEY%"

:: Parse arguments
:parse_args
if "%~1"=="" goto :check_prompt
if /I "%~1"=="--prompt" set "PROMPT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-p" set "PROMPT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--template-uuid" set "TEMPLATE_UUID=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-t" set "TEMPLATE_UUID=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--checkpoint-id" set "CHECKPOINT_ID=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-c" set "CHECKPOINT_ID=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--negative-prompt" set "NEGATIVE_PROMPT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-n" set "NEGATIVE_PROMPT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--width" set "WIDTH=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-W" set "WIDTH=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--height" set "HEIGHT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-H" set "HEIGHT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--steps" set "STEPS=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-s" set "STEPS=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--cfg-scale" set "CFG_SCALE=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--seed" set "SEED=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--img-count" set "IMG_COUNT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-N" set "IMG_COUNT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--output" set "OUTPUT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="-o" set "OUTPUT=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--access-key" set "ACCESS_KEY=%~2" & shift & shift & goto :parse_args
if /I "%~1"=="--secret-key" set "SECRET_KEY=%~2" & shift & shift & goto :parse_args
echo Unknown option: %~1 >&2
exit /b 1

:check_prompt
if "!PROMPT!"=="" (
    echo Error: --prompt is required >&2
    exit /b 1
)

:: Build command
set "CMD=%LIBTV_CLI%"

if not "!ACCESS_KEY!"=="" (
    set "CMD=!CMD! --access-key !ACCESS_KEY!"
)
if not "!SECRET_KEY!"=="" (
    set "CMD=!CMD! --secret-key !SECRET_KEY!"
)

set "CMD=!CMD! text2img"
set "CMD=!CMD! --prompt !PROMPT!"
set "CMD=!CMD! --template-uuid !TEMPLATE_UUID!"

if not "!CHECKPOINT_ID!"=="" (
    set "CMD=!CMD! --checkpoint-id !CHECKPOINT_ID!"
)
if not "!NEGATIVE_PROMPT!"=="" (
    set "CMD=!CMD! --negative-prompt !NEGATIVE_PROMPT!"
)

set "CMD=!CMD! --width !WIDTH!"
set "CMD=!CMD! --height !HEIGHT!"
set "CMD=!CMD! --steps !STEPS!"
set "CMD=!CMD! --cfg-scale !CFG_SCALE!"
set "CMD=!CMD! --seed=!SEED!"
set "CMD=!CMD! --img-count !IMG_COUNT!"
set "CMD=!CMD! --output !OUTPUT!"

echo Executing: !CMD!
!CMD!
