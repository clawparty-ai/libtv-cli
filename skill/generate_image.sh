#!/bin/bash
# Skill wrapper for libtv-cli image generation
# This script provides a simplified interface for the agent to call libtv-cli

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Try to find libtv-cli binary
if [ -n "$LIBTV_CLI_PATH" ]; then
    LIBTV_CLI="$LIBTV_CLI_PATH"
elif [ -f "/Users/caishu/.clawparty/agents/libtv文生图/libtv-cli/target/release/libtv-cli" ]; then
    LIBTV_CLI="/Users/caishu/.clawparty/agents/libtv文生图/libtv-cli/target/release/libtv-cli"
elif command -v libtv-cli >/dev/null 2>&1; then
    LIBTV_CLI="libtv-cli"
else
    echo "Error: libtv-cli binary not found. Set LIBTV_CLI_PATH or install it." >&2
    exit 1
fi

# Default values
PROMPT=""
TEMPLATE_UUID="bf085132c7134622895b783b520b39ff"
CHECKPOINT_ID=""
NEGATIVE_PROMPT=""
WIDTH=512
HEIGHT=512
STEPS=20
CFG_SCALE=7.0
SEED=-1
IMG_COUNT=1
OUTPUT="generated.png"
ACCESS_KEY="${LIBLIB_ACCESS_KEY:-}"
SECRET_KEY="${LIBLIB_SECRET_KEY:-}"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --prompt|-p)
            PROMPT="$2"
            shift 2
            ;;
        --template-uuid|-t)
            TEMPLATE_UUID="$2"
            shift 2
            ;;
        --checkpoint-id|-c)
            CHECKPOINT_ID="$2"
            shift 2
            ;;
        --negative-prompt|-n)
            NEGATIVE_PROMPT="$2"
            shift 2
            ;;
        --width|-W)
            WIDTH="$2"
            shift 2
            ;;
        --height|-H)
            HEIGHT="$2"
            shift 2
            ;;
        --steps|-s)
            STEPS="$2"
            shift 2
            ;;
        --cfg-scale)
            CFG_SCALE="$2"
            shift 2
            ;;
        --seed)
            SEED="$2"
            shift 2
            ;;
        --img-count|-N)
            IMG_COUNT="$2"
            shift 2
            ;;
        --output|-o)
            OUTPUT="$2"
            shift 2
            ;;
        --access-key)
            ACCESS_KEY="$2"
            shift 2
            ;;
        --secret-key)
            SECRET_KEY="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

# Validate required parameters
if [ -z "$PROMPT" ]; then
    echo "Error: --prompt is required" >&2
    exit 1
fi

# Build command
CMD="$LIBTV_CLI"

# Add global options
if [ -n "$ACCESS_KEY" ]; then
    CMD="$CMD --access-key '$ACCESS_KEY'"
fi
if [ -n "$SECRET_KEY" ]; then
    CMD="$CMD --secret-key '$SECRET_KEY'"
fi

# Add subcommand and args
CMD="$CMD text2img"
CMD="$CMD --prompt '$PROMPT'"
CMD="$CMD --template-uuid '$TEMPLATE_UUID'"

if [ -n "$CHECKPOINT_ID" ]; then
    CMD="$CMD --checkpoint-id '$CHECKPOINT_ID'"
fi
if [ -n "$NEGATIVE_PROMPT" ]; then
    CMD="$CMD --negative-prompt '$NEGATIVE_PROMPT'"
fi

CMD="$CMD --width $WIDTH"
CMD="$CMD --height $HEIGHT"
CMD="$CMD --steps $STEPS"
CMD="$CMD --cfg-scale $CFG_SCALE"
CMD="$CMD --seed=$SEED"
CMD="$CMD --img-count $IMG_COUNT"
CMD="$CMD --output '$OUTPUT'"

# Execute
echo "Executing: $CMD"
eval $CMD
