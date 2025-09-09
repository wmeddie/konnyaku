mod translation;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use translation::{TranslationDirection, TranslationService};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateRequest {
    text: String,
    direction: String, // "en-ja" or "ja-en"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponse {
    success: bool,
    translation: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStatusResponse {
    loaded: bool,
}

// Wrapper struct for TranslationService to make it manageable by Tauri
pub struct TranslationServiceState(Arc<TranslationService>);

#[tauri::command]
async fn translate(
    request: TranslateRequest,
    state: State<'_, TranslationServiceState>,
) -> Result<TranslateResponse, String> {
    // Parse translation direction
    let direction = match request.direction.as_str() {
        "en-ja" => TranslationDirection::EnglishToJapanese,
        "ja-en" => TranslationDirection::JapaneseToEnglish,
        _ => {
            return Ok(TranslateResponse {
                success: false,
                translation: None,
                error: Some(format!("Invalid translation direction: {}", request.direction)),
            });
        }
    };
    
    // Perform translation
    match state.0.translate(&request.text, direction).await {
        Ok(translated_text) => Ok(TranslateResponse {
            success: true,
            translation: Some(translated_text),
            error: None,
        }),
        Err(e) => Ok(TranslateResponse {
            success: false,
            translation: None,
            error: Some(format!("Translation failed: {}", e)),
        }),
    }
}

#[tauri::command]
async fn get_model_status(state: State<'_, TranslationServiceState>) -> Result<ModelStatusResponse, String> {
    let loaded = state.0.is_model_loaded().await;
    Ok(ModelStatusResponse { loaded })
}

#[tauri::command]
async fn ensure_model_downloaded(state: State<'_, TranslationServiceState>) -> Result<bool, String> {
    match state.0.ensure_model_downloaded().await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Failed to download model: {}", e)),
    }
}

#[tauri::command]
async fn initialize_model(state: State<'_, TranslationServiceState>) -> Result<bool, String> {
    match state.0.ensure_model_loaded().await {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Failed to initialize model: {}", e)),
    }
}

#[tauri::command]
fn get_supported_languages() -> Vec<String> {
    vec!["en-ja".to_string(), "ja-en".to_string()]
}

// Legacy greet command (can be removed later)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the translation service
    let translation_service = match TranslationService::new() {
        Ok(service) => Arc::new(service),
        Err(e) => {
            eprintln!("Failed to initialize translation service: {}", e);
            panic!("Cannot start application without translation service");
        }
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(TranslationServiceState(translation_service))
        .invoke_handler(tauri::generate_handler![
            greet,
            translate,
            get_model_status,
            ensure_model_downloaded,
            initialize_model,
            get_supported_languages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
