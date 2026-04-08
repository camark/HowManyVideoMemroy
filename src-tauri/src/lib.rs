use serde::Serialize;

/// Model database with base VRAM requirements (FP16, in GB)
static MODEL_DATABASE: &[(&str, f64)] = &[
    ("llama-3-8b", 16.0),
    ("llama-3-70b", 140.0),
    ("mistral-7b", 14.0),
    ("mixtral-8x7b", 47.0),
    ("qwen-72b", 144.0),
    ("yi-34b", 68.0),
    ("gemma-7b", 14.0),
    ("gemma2-27b", 54.0),
    ("falcon-180b", 360.0),
];

#[derive(Debug, Clone, Serialize)]
pub struct VramResult {
    pub model_name: String,
    pub found: bool,
    pub fp16_gb: f64,
    pub fp32_gb: f64,
    pub int8_gb: f64,
    pub int4_gb: f64,
    pub with_kv_cache_fp16: f64,
}

#[tauri::command]
fn calculate_vram(model_name: String) -> VramResult {
    let normalized = model_name.to_lowercase().replace(['_', ' '], "-");

    match MODEL_DATABASE
        .iter()
        .find(|(name, _)| name.contains(&normalized) || normalized.contains(*name))
    {
        Some((_, &base_gb)) => VramResult {
            model_name,
            found: true,
            fp16_gb: base_gb,
            fp32_gb: base_gb * 2.0,
            int8_gb: base_gb * 0.5,
            int4_gb: base_gb * 0.25,
            with_kv_cache_fp16: base_gb * 1.2,
        },
        None => VramResult {
            model_name,
            found: false,
            fp16_gb: 0.0,
            fp32_gb: 0.0,
            int8_gb: 0.0,
            int4_gb: 0.0,
            with_kv_cache_fp16: 0.0,
        },
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, calculate_vram])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_model() {
        let result = calculate_vram("Llama-3-8B".to_string());
        assert!(result.found);
        assert_eq!(result.fp16_gb, 16.0);
    }

    #[test]
    fn test_unknown_model() {
        let result = calculate_vram("unknown-model".to_string());
        assert!(!result.found);
        assert_eq!(result.fp16_gb, 0.0);
    }

    #[test]
    fn test_quantization_calculations() {
        let result = calculate_vram("mistral-7b".to_string());
        assert_eq!(result.fp32_gb, 28.0);
        assert_eq!(result.int8_gb, 7.0);
        assert_eq!(result.int4_gb, 3.5);
    }

    #[test]
    fn test_kv_cache_overhead() {
        let result = calculate_vram("llama-3-8b".to_string());
        assert_eq!(result.with_kv_cache_fp16, 19.2);
    }
}
