# Konnyaku 🍡

A lightweight, privacy-focused Japanese-English translation desktop application powered by local AI. Built with Tauri, React, and the LiquidAI LFM2-350M-ENJP-MT model running entirely on your device.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![Node](https://img.shields.io/badge/node-18%2B-green)

## Features

- 🔐 **100% Private**: All translations happen locally on your device
- 🚀 **Fast**: GPU-accelerated inference using Metal (macOS) or CUDA (NVIDIA)
- 🎯 **Accurate**: Powered by LiquidAI's specialized Japanese-English model
- 💻 **Native Performance**: Built with Rust and Tauri for minimal resource usage
- 🎨 **Modern UI**: Clean, responsive interface built with React
- 📖 **No Internet Required**: Works completely offline after initial model download

## Prerequisites

Before building Konnyaku, ensure you have the following installed:

- **Rust** (1.70 or later): [Install Rust](https://rustup.rs/)
- **Node.js** (18 or later) and pnpm: [Install Node.js](https://nodejs.org/)
- **Tauri Prerequisites**: 
  - macOS: Xcode Command Line Tools
  - Linux: See [Tauri Linux Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux)
  - Windows: See [Tauri Windows Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-windows)

### Installing pnpm

```bash
npm install -g pnpm
```

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/konnyaku.git
cd konnyaku
```

### 2. Install Dependencies

```bash
# Install frontend dependencies
pnpm install

# The Rust dependencies will be installed automatically when building
```

### 3. Development Mode

Run the app in development mode with hot-reload:

```bash
pnpm tauri dev
```

The app will:
1. Start the Vite dev server for the frontend
2. Compile the Rust backend
3. Open the application window
4. Automatically download the AI model on first run (~200MB)

### 4. Production Build

Create an optimized production build:

```bash
pnpm tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Usage

1. **Launch the app** - The model will download automatically on first run
2. **Select translation direction** - Click the language selector to switch between EN→JA and JA→EN
3. **Enter text** - Type or paste text in the input field (max 5000 characters)
4. **Translate** - Click the translate button or press Cmd/Ctrl+Enter
5. **Copy result** - Click the copy button to copy the translation to clipboard

## Project Structure

```
konnyaku/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/             # Custom React hooks
│   └── App.jsx            # Main app component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Tauri commands
│   │   ├── main.rs        # App entry point
│   │   └── translation.rs # LLM inference logic
│   └── Cargo.toml         # Rust dependencies
├── public/                # Static assets
└── package.json          # Node dependencies
```

## Technical Details

### Model

- **Model**: LiquidAI/LFM2-350M-ENJP-MT-GGUF
- **Size**: ~200MB (4-bit quantized)
- **Format**: GGUF (llama.cpp compatible)
- **Specialization**: Japanese-English translation
- **Context**: 4096 tokens

### Backend Implementation

The backend uses the `llama-cpp-2` crate for model inference:

- **GPU Acceleration**: Metal on macOS, CUDA on NVIDIA GPUs
- **Memory Efficient**: Model loaded once and kept in memory
- **Thread Safe**: Arc<Mutex> for concurrent access
- **Chat Templates**: Uses model's embedded template for proper formatting

### Frontend Stack

- **Framework**: React 18
- **Build Tool**: Vite
- **Styling**: CSS with modern features
- **State Management**: React hooks
- **API**: Tauri commands via IPC

## Contributing

We welcome contributions! Here's how to get started:

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature`
3. **Make your changes**
4. **Test thoroughly**: Ensure translations work in both directions
5. **Commit with descriptive messages**: Follow conventional commits
6. **Push and create a PR**

### Code Style

- **Rust**: Follow standard Rust conventions (use `cargo fmt` and `cargo clippy`)
- **JavaScript/React**: ESLint configuration included
- **CSS**: BEM naming convention for components

### Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (if available)
pnpm test
```

### Areas for Contribution

- [ ] Add support for more language pairs
- [ ] Implement translation history
- [ ] Add dark mode support
- [ ] Optimize model loading time
- [ ] Add batch translation support
- [ ] Implement translation confidence scores
- [ ] Add support for document translation
- [ ] Create automated tests
- [ ] Improve error handling and user feedback

## Troubleshooting

### Model Download Issues

If the model download fails:

1. Check your internet connection
2. The model will be downloaded to:
   - macOS: `~/Library/Caches/konnyaku/models/`
   - Linux: `~/.cache/konnyaku/models/`
   - Windows: `%LOCALAPPDATA%\konnyaku\cache\models\`
3. You can manually download from: [HuggingFace](https://huggingface.co/LiquidAI/LFM2-350M-ENJP-MT-GGUF)
4. Place the `LFM2-350M-ENJP-MT-Q4_K_M.gguf` file in the cache directory

### Build Errors

- **Rust errors**: Ensure you have the latest Rust toolchain: `rustup update`
- **Node errors**: Clear node_modules and reinstall: `rm -rf node_modules && pnpm install`
- **Tauri errors**: Check platform-specific prerequisites

### Performance Issues

- The first translation after app start may be slower (model loading)
- Ensure GPU acceleration is enabled (check console output)
- Close other resource-intensive applications

## Architecture Decisions

See the `docs/` directory for architecture decision records (ADRs) documenting key design choices.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [LiquidAI](https://liquid.ai/) for the LFM2-350M-ENJP-MT model
- [llama.cpp](https://github.com/ggerganov/llama.cpp) for the inference engine
- [Tauri](https://tauri.app/) for the application framework
- The Rust and React communities

## Support

- **Issues**: [GitHub Issues](https://github.com/wmeddie/konnyaku/issues)
- **Discussions**: [GitHub Discussions](https://github.com/wmeddie/konnyaku/discussions)

---

Built with ❤️ for the Japanese learning community
