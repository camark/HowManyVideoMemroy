# How Much Video Memory?

A Tauri v2 desktop application that calculates VRAM requirements for AI/LLM models across different quantization levels.

## Features

- **Quick VRAM Estimates** — Get memory requirements for popular LLM models
- **Multiple Quantization Levels** — FP16, FP32, INT8, INT4 calculations
- **KV Cache Support** — Includes option for context memory overhead
- **Dark Mode** — Native system theme support

## Installation

### Prerequisites

- Node.js (v18 or later)
- Rust (latest stable)
- Cargo Tauri CLI

### Setup

```bash
# Install dependencies
npm install

# Build CSS
npm run build:css

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## Usage

1. Enter a model name (e.g., "Llama-3-8B", "Mistral-7B", "GPT-2")
2. Select quantization level (FP16, FP32, INT8, INT4)
3. Optionally include KV cache memory
4. Click "Calculate" to see VRAM requirements

## How It Works

The app calculates VRAM using:
1. **Database Lookup** — Checks internal model database for known models
2. **Parameter Extraction** — Falls back to parsing parameters from model name (e.g., "8b" → 8 billion)
3. **Base Calculation** — FP16 = parameters_billion × 2.0 GB
4. **Quantization Multipliers**:
   - FP32 = FP16 × 2.0
   - FP16 = base value
   - INT8 = FP16 × 0.5
   - INT4 = FP16 × 0.25

## Tech Stack

- **Frontend:** Vanilla HTML/CSS/JavaScript + Tailwind CSS v4
- **Backend:** Rust (Tauri v2 commands)
- **Styling:** PostCSS with `@tailwindcss/postcss`

## Project Structure

```
HowManyVideoMemory/
├── src/                    # Frontend source files
│   ├── index.html          # Main UI
│   ├── main.js             # Tauri command calls
│   ├── styles.css          # Tailwind imports + custom styles
│   └── dist.css            # Generated CSS
├── src-tauri/              # Rust backend
│   ├── src/
│   │   └── lib.rs          # VRAM calculation logic
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── CLAUDE.md               # Project documentation
└── package.json            # Node dependencies
```

## License

MIT
