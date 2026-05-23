#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENV_DIR="$SCRIPT_DIR/venv"

# Activate virtual environment
source "$VENV_DIR/bin/activate"

# Ensure CUDA 12.x compat symlinks exist for cuBLAS
if [ ! -L "$VENV_DIR/lib/libcublas.so.12" ]; then
  ln -sf /usr/local/cuda-13.2/targets/x86_64-linux/lib/libcublas.so.13 \
    "$VENV_DIR/lib/libcublas.so.12"
  ln -sf /usr/local/cuda-13.2/targets/x86_64-linux/lib/libcublasLt.so.13 \
    "$VENV_DIR/lib/libcublasLt.so.12"
fi

# Set library path for CUDA compat layer
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}$VENV_DIR/lib"

# Default to GPU if available, fall back to CPU
export INFERENCE_MODE="${INFERENCE_MODE:-auto}"

exec python3 -m uvicorn app.main:app --host 0.0.0.0 --port "${PORT:-8002}"
