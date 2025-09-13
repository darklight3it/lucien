#!/usr/bin/env bash
set -e

# ---------------------------
# Source Envs
# ---------------------------

set -a
source .env
set a

# ---------------------------
# Clean folders
# ---------------------------

rm -rf "$MODEL_DIR" && rm -rf "$TEMP_DIR"

# ---------------------------
# Create folders
# ---------------------------
mkdir -p "$MODEL_DIR" && mkdir -p "$TEMP_DIR"

# ---------------------------
# Download the model from Hugging Face
# ---------------------------
echo "📥 Downloading model from Hugging Face..."


hf download "$MODEL_NAME" \
    --local-dir "$TEMP_DIR" \
    --include "$MODEL_VERSION"

# if there is the --convert-to-gguf flag, we convert the model
if [ "$1" == "--convert-to-gguf" ]; then
    echo "🔄 Converting model to GGUF..."

    # ---------------------------
    # Clone llama.cpp if necessary
    # ---------------------------
    if [ ! -d "$TEMP_DIR/llama.cpp" ]; then
        echo "🦙 Cloning llama.cpp..."
        git clone
    fi

    # ---------------------------
    # Install Python requirements
    # ---------------------------
    pip install -r "$TEMP_DIR/llama.cpp/requirements.txt"

    # ---------------------------
    # Convert the model to GGUF
    # ---------------------------
    echo "🔄 Converting model to GGUF..."

    python3 "$TEMP_DIR/llama.cpp/convert_hf_to_gguf.py" "$TEMP_DIR" --outfile "$MODEL_DIR/$MODEL_NAME.gguf" --outtype q8_0
fi

# ---------------------------
# Cleanup temporary files
# ---------------------------
echo "🧹 Cleaning up temporary files..."
mv $TEMP_DIR/* $MODEL_DIR
rm -rf "$TEMP_DIR"

echo "✅ Model ready at ${MODEL_DIR}"
