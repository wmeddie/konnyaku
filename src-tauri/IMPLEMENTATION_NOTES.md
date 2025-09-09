# Konnyaku Backend Implementation Notes

## Model Integration Details

### Model Used
- **Model**: LiquidAI/LFM2-350M-ENJP-MT-GGUF
- **File**: lfm2-350m-enjp-mt-q4_k_m.gguf
- **Context Length**: 128,000 tokens (we use 4096 for efficiency)
- **Type**: Single-turn translation model

### Key Implementation Decisions

#### 1. llama-cpp-2 Configuration
- Disabled OpenMP to avoid linking issues on macOS: `default-features = false`
- Enabled Metal feature for GPU acceleration on macOS
- Using version 0.1 of llama-cpp-2

#### 2. Model Loading Strategy
- Model is downloaded from HuggingFace on first use
- Cached in platform-specific directory using `directories` crate
- Loaded into memory on first translation request
- Kept in memory for subsequent translations

#### 3. Translation Prompt Format
The model requires specific prompt formatting:
- **English to Japanese**: `"Translate to Japanese.\n[text]"`
- **Japanese to English**: `"Translate to English.\n[text]"`
- Single-turn only (no conversation history)

#### 4. Sampling Strategy
- Using greedy sampling (deterministic) for consistent translations
- No temperature/top-p randomness for translation tasks
- Max tokens set to 512 for output

#### 5. Memory Management
- Creating new context for each translation to ensure clean state
- Using Arc<Mutex<>> for thread-safe model state
- Backend initialized once and reused

#### 6. UTF-8 Handling
- Using `encoding_rs` for proper UTF-8 decoding
- Essential for Japanese character support
- Decoder handles partial UTF-8 sequences correctly

## Known Issues and Limitations

1. **First Load Time**: Initial model loading can take 10-30 seconds
2. **Memory Usage**: Model uses ~350MB when loaded
3. **Context Reuse**: Currently creates new context per translation (could be optimized)

## Testing Notes

### Test Phrases
- English to Japanese:
  - "Hello, how are you?" → Should produce Japanese greeting
  - "The weather is nice today." → Should produce weather comment in Japanese
  
- Japanese to English:
  - "こんにちは、元気ですか？" → "Hello, how are you?"
  - "今日はいい天気ですね。" → "The weather is nice today."

## Future Improvements

1. **Performance**:
   - Reuse context between translations
   - Implement translation caching
   - Pre-warm model on app start

2. **Features**:
   - Support for batch translations
   - Confidence scores for translations
   - Alternative translations

3. **Error Handling**:
   - Better error messages for model loading failures
   - Graceful degradation if model unavailable
   - Retry logic for downloads

## Debug Commands

To test the model directly:
```bash
# English to Japanese
llama-cli -hf LiquidAI/LFM2-350M-ENJP-MT-GGUF -sys "Translate to Japanese." -st

# Japanese to English  
llama-cli -hf LiquidAI/LFM2-350M-ENJP-MT-GGUF -sys "Translate to English." -st
```

## Architecture Notes

The translation service follows a layered architecture:
1. **Tauri Commands** (lib.rs) - Frontend interface
2. **Translation Service** (translation.rs) - Business logic
3. **llama-cpp-2** - Model inference
4. **llama.cpp** (C++) - Core inference engine

State management uses Tauri's built-in state system with Arc<Mutex<>> for thread safety.