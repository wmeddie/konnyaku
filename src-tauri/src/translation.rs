use anyhow::{Context, Result};
use directories::ProjectDirs;
use hf_hub::api::tokio::Api;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel, Special};
use llama_cpp_2::sampling::LlamaSampler;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

// Model configuration constants
const MODEL_REPO: &str = "LiquidAI/LFM2-350M-ENJP-MT-GGUF";
const MODEL_FILE: &str = "lfm2-350m-enjp-mt-q4_k_m.gguf";
const SYSTEM_PROMPT_EN_TO_JA: &str = "Translate to Japanese.";
const SYSTEM_PROMPT_JA_TO_EN: &str = "Translate to English.";
const MAX_TOKENS: i32 = 512;
const CONTEXT_SIZE: u32 = 4096;  // Sufficient for translation tasks, model supports up to 128000

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranslationDirection {
    EnglishToJapanese,
    JapaneseToEnglish,
}

// Model state holding the loaded model and context
pub struct ModelState {
    backend: LlamaBackend,
    model: Option<LlamaModel>,
    is_loaded: bool,
}

pub struct TranslationService {
    model_state: Arc<Mutex<ModelState>>,
    model_path: PathBuf,
}

impl TranslationService {
    /// Create a new TranslationService instance
    pub fn new() -> Result<Self> {
        // Get the cache directory for storing the model
        let cache_dir = Self::get_cache_dir()?;
        let model_path = cache_dir.join(MODEL_FILE);
        
        // Initialize the LlamaBackend
        let backend = LlamaBackend::init()
            .context("Failed to initialize LlamaBackend")?;
        
        let model_state = ModelState {
            backend,
            model: None,
            is_loaded: false,
        };
        
        Ok(Self {
            model_state: Arc::new(Mutex::new(model_state)),
            model_path,
        })
    }
    
    /// Get the cache directory for storing models
    fn get_cache_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "konnyaku", "konnyaku")
            .context("Failed to determine project directories")?;
        
        let cache_dir = proj_dirs.cache_dir().join("models");
        std::fs::create_dir_all(&cache_dir)
            .context("Failed to create cache directory")?;
        
        Ok(cache_dir)
    }
    
    /// Download the model from HuggingFace if not cached
    pub async fn ensure_model_downloaded(&self) -> Result<()> {
        if self.model_path.exists() {
            println!("Model already cached at: {:?}", self.model_path);
            return Ok(());
        }
        
        println!("Downloading model from HuggingFace...");
        println!("Model: {}/{}", MODEL_REPO, MODEL_FILE);
        
        // Ensure the parent directory exists
        if let Some(parent) = self.model_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create model directory")?;
        }
        
        // Try direct download first as it's often faster
        let direct_url = format!(
            "https://huggingface.co/{}/resolve/main/{}",
            MODEL_REPO, MODEL_FILE
        );
        
        println!("Attempting direct download from: {}", direct_url);
        
        match self.download_file_direct(&direct_url).await {
            Ok(()) => {
                println!("Model downloaded successfully via direct download");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Direct download failed: {}, trying HuggingFace API...", e);
            }
        }
        
        // Fallback to HuggingFace API
        let download_timeout = std::time::Duration::from_secs(300);
        
        let api = Api::new()
            .context("Failed to create HuggingFace API")?;
        let repo = api.model(MODEL_REPO.to_string());
        
        let download_future = async {
            println!("Starting HuggingFace API download...");
            let model_file = repo.get(MODEL_FILE).await
                .context("Failed to download model from HuggingFace")?;
            
            println!("Download complete, copying to cache...");
            
            // Copy to cache location
            tokio::fs::copy(&model_file, &self.model_path)
                .await
                .context("Failed to copy model to cache")?;
            
            Ok::<(), anyhow::Error>(())
        };
        
        match tokio::time::timeout(download_timeout, download_future).await {
            Ok(Ok(())) => {
                println!("Model downloaded successfully to: {:?}", self.model_path);
                Ok(())
            }
            Ok(Err(e)) => {
                eprintln!("HuggingFace API download failed: {}", e);
                eprintln!("\nPlease try downloading the model manually:");
                eprintln!("1. Download from: {}", direct_url);
                eprintln!("2. Save to: {:?}", self.model_path);
                Err(e)
            }
            Err(_) => {
                let err = anyhow::anyhow!("Model download timed out after 5 minutes");
                eprintln!("{}", err);
                eprintln!("\nPlease try downloading the model manually:");
                eprintln!("1. Download from: {}", direct_url);
                eprintln!("2. Save to: {:?}", self.model_path);
                Err(err)
            }
        }
    }
    
    /// Direct download using reqwest (simpler than HuggingFace API)
    async fn download_file_direct(&self, url: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;
        
        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to start download")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }
        
        let total_size = response
            .content_length()
            .unwrap_or(0);
        
        println!("Download size: {} MB", total_size / 1_048_576);
        
        let mut file = tokio::fs::File::create(&self.model_path)
            .await
            .context("Failed to create file")?;
        
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();
        
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Error while downloading chunk")?;
            file.write_all(&chunk)
                .await
                .context("Failed to write to file")?;
            
            downloaded += chunk.len() as u64;
            
            // Print progress every 10MB
            if downloaded % (10 * 1_048_576) == 0 || downloaded == total_size {
                let progress = if total_size > 0 {
                    (downloaded as f64 / total_size as f64 * 100.0) as u32
                } else {
                    0
                };
                println!("Download progress: {} MB / {} MB ({}%)",
                         downloaded / 1_048_576,
                         total_size / 1_048_576,
                         progress);
            }
        }
        
        file.flush().await?;
        println!("Download complete!");
        
        Ok(())
    }
    
    /// Initialize the model if not already loaded
    pub async fn ensure_model_loaded(&self) -> Result<()> {
        let mut state = self.model_state.lock().await;
        
        if state.is_loaded {
            return Ok(());
        }
        
        // Ensure model is downloaded
        drop(state); // Release lock temporarily
        self.ensure_model_downloaded().await?;
        state = self.model_state.lock().await; // Re-acquire lock
        
        println!("Loading model from: {:?}", self.model_path);
        
        // Configure model parameters (use Metal for GPU acceleration on macOS)
        let model_params = LlamaModelParams::default()
            .with_n_gpu_layers(1000); // Offload all layers to GPU if available
        
        // Load the model
        let model = LlamaModel::load_from_file(
            &state.backend,
            &self.model_path,
            &model_params,
        )
        .context("Failed to load model")?;
        
        state.model = Some(model);
        state.is_loaded = true;
        
        println!("Model loaded successfully");
        Ok(())
    }
    
    /// Translate text based on the specified direction
    pub async fn translate(
        &self,
        text: &str,
        direction: TranslationDirection,
    ) -> Result<String> {
        // Ensure model is loaded
        self.ensure_model_loaded().await?;
        
        let state = self.model_state.lock().await;
        let model = state.model.as_ref()
            .context("Model not loaded")?;
        
        // Get the appropriate system prompt
        let system_prompt = match direction {
            TranslationDirection::EnglishToJapanese => SYSTEM_PROMPT_EN_TO_JA,
            TranslationDirection::JapaneseToEnglish => SYSTEM_PROMPT_JA_TO_EN,
        };
        
        // Format the prompt for single-turn translation model
        // The model expects: system prompt with "Translate to [Language]." followed by the text
        let full_prompt = format!("{}\n{}", system_prompt, text);
        
        println!("Translating with prompt: {}", full_prompt);
        
        // Create context parameters
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(NonZeroU32::new(CONTEXT_SIZE).unwrap()))
            .with_n_threads(4); // Use 4 threads for CPU inference
        
        // Create a new context for this translation
        let mut ctx = model
            .new_context(&state.backend, ctx_params)
            .context("Failed to create context")?;
        
        // Tokenize the prompt
        let tokens_list = model
            .str_to_token(&full_prompt, AddBos::Always)
            .context("Failed to tokenize prompt")?;
        
        // Create a batch for processing
        let mut batch = LlamaBatch::new(512, 1);
        
        // Add all prompt tokens to the batch
        let last_index = (tokens_list.len() - 1) as i32;
        for (i, token) in (0_i32..).zip(tokens_list.iter()) {
            let is_last = i == last_index;
            batch.add(*token, i, &[0], is_last)?;
        }
        
        // Process the prompt
        ctx.decode(&mut batch)
            .context("Failed to decode prompt")?;
        
        // Initialize the decoder for UTF-8 output
        let mut decoder = encoding_rs::UTF_8.new_decoder();
        
        // Create a sampler for token generation
        // Using greedy sampling for deterministic output (best for translation)
        let mut sampler = LlamaSampler::greedy();
        
        // Generate the translation
        let mut translation = String::new();
        let mut n_cur = batch.n_tokens();
        let max_new_tokens = MAX_TOKENS - tokens_list.len() as i32;
        
        for _ in 0..max_new_tokens {
            // Sample the next token
            let token = sampler.sample(&ctx, n_cur - 1);
            sampler.accept(token);
            
            // Check for end of sequence
            if model.is_eog_token(token) {
                break;
            }
            
            // Convert token to text
            let output_bytes = model
                .token_to_bytes(token, Special::Tokenize)
                .context("Failed to convert token to bytes")?;
            
            // Decode bytes to string
            let mut output_string = String::with_capacity(32);
            let _decode_result = decoder.decode_to_string(&output_bytes, &mut output_string, false);
            
            // Add to translation
            translation.push_str(&output_string);
            
            // Prepare for next token
            batch.clear();
            batch.add(token, n_cur, &[0], true)?;
            
            n_cur += 1;
            
            // Process the new token
            ctx.decode(&mut batch)
                .context("Failed to decode token")?;
        }
        
        // Clean up the translation (remove any extra whitespace)
        let translation = translation.trim().to_string();
        
        println!("Translation complete: {}", translation);
        
        Ok(translation)
    }
    
    /// Check if the model is currently loaded
    pub async fn is_model_loaded(&self) -> bool {
        self.model_state.lock().await.is_loaded
    }
}

// Make TranslationService thread-safe and Send
unsafe impl Send for TranslationService {}
unsafe impl Sync for TranslationService {}

// Implementation notes:
// 1. Using LlamaBackend::init() to initialize the backend once
// 2. Loading model with LlamaModel::load_from_file()
// 3. Creating a new context for each translation to ensure clean state
// 4. Using greedy sampling for deterministic translations
// 5. Processing tokens in batches using LlamaBatch
// 6. Properly handling UTF-8 decoding for Japanese text
// 7. Using Metal acceleration on macOS when available