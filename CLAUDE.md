# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**How Much Video Memory?** — A Tauri v2 desktop app that calculates VRAM requirements for AI/LLM models across different quantization levels (FP16, FP32, INT8, INT4).

## Tech Stack

- **Frontend:** Vanilla HTML/CSS/JavaScript + Tailwind CSS v4
- **Backend:** Rust (Tauri v2 commands)
- **Styling:** PostCSS with `@tailwindcss/postcss`

## Commands

### Development
```bash
# Build CSS (required before running app)
npm run build:css

# Run Tauri dev server
cargo tauri dev

# Run Rust tests
cargo test
```

### Build
```bash
# Build CSS for production
npm run build:css

# Build Tauri app for distribution
cargo tauri build
```

## Architecture

### Backend (`src-tauri/src/lib.rs`)
- **Model Database:** Static slice of model names → base FP16 VRAM (GB)
- **`calculate_vram(model_name)`:** Returns VRAM for FP16, FP32, INT8, INT4, and FP16+KV cache
- Model matching is case-insensitive and supports partial matches

### Frontend (`src/`)
- `index.html` — Main UI with model input and results card
- `main.js` — Calls `calculate_vram` Tauri command, renders results
- `styles.css` — Tailwind CSS v4 imports + custom component styles
- `dist.css` — Generated CSS (run `npm run build:css` to regenerate)

### Configuration
- `tauri.conf.json` — Window size (800x600), app identifier, CSP
- `Cargo.toml` — Dependencies: tauri, tauri-plugin-opener, serde, serde_json

## Key Patterns

- Tauri commands use `#[tauri::command]` macro and are registered in `run()` via `invoke_handler`
- Frontend calls backend via `window.__TAURI__.core.invoke()`
- Results displayed in a card grid layout with dark mode support
