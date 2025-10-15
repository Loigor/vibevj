# VibeVJ 🎨✨

> A powerful Visual Jockey application for creating stunning real-time visualizations

VibeVJ is a modern VJ application built in Rust that combines a GPU-accelerated 3D rendering engine with audio analysis and a node-based visual programming interface. Create mesmerizing audio-reactive visualizations for performances, streams, or artistic expression.

![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![Status](https://img.shields.io/badge/status-alpha-yellow

> **Disclaimer:**  
> This project has been programmed with significant assistance from AI agents, primarily Claude Sonnet 4.5. VibeVJ is the third major iteration, developed with enhanced attention to performance and code quality.

## ✨ Features

### Current (Phase 1)
- 🎨 **Modern GPU Rendering**: WGPU-based rendering engine with shader support
- 🎵 **Audio Analysis**: Real-time FFT analysis with frequency band extraction
- 🖥️ **Intuitive GUI**: Three-panel egui interface for efficient workflow
- 🔧 **Node-Based Programming**: Visual programming for creating scenes
- 📝 **Scripting Support**: Rhai scripting for programmatic control
- 🎬 **Scene Management**: Hierarchical scene graph with component system

### Planned Features
- Scene sequencer for animations and transitions
- 2D visualization support (images, videos, GIFs)
- Advanced shader effects
- Beat detection and audio reactivity
- Video recording and export
- Plugin system for extensibility

See [ROADMAP.md](ROADMAP.md) for detailed development plans.

## 🏗️ Architecture

VibeVJ is built as a modular Rust workspace with clean separation of concerns:

```
vibevj/
├── crates/
│   ├── vibevj-common/      # Shared types and utilities
│   ├── vibevj-engine/      # 3D rendering engine (WGPU)
│   ├── vibevj-gui/         # User interface (egui)
│   ├── vibevj-audio/       # Audio analysis (FFT, frequency bands)
│   ├── vibevj-scene/       # Scene graph and node system
│   └── vibevj-scripting/   # Rhai scripting integration
└── src/                    # Main application entry point
```

### Module Overview

- **vibevj-engine**: Core rendering with WGPU, pipeline management, shaders, cameras, and meshes
- **vibevj-gui**: Three-panel layout with preview, editor, and resource browser
- **vibevj-audio**: Real-time audio capture and FFT analysis for audio reactivity
- **vibevj-scene**: Scene hierarchy, components, and node graph for visual programming
- **vibevj-scripting**: Rhai scripting engine with VibeVJ API bindings
- **vibevj-common**: Shared types, errors, and utilities used across all modules

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://rustup.rs/))
- **Git** for cloning the repository
- A GPU with Vulkan, DirectX 12, or Metal support

### Building

```bash
# Clone the repository
git clone https://github.com/Loigor/vibevj.git
cd vibevj

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Development Build

```bash
# Build in debug mode (faster compilation)
cargo build

# Run with logging
RUST_LOG=info cargo run
```

## 🎮 Usage

### GUI Layout

VibeVJ features a three-panel layout:

- **Left Panel**: Main render preview and playback controls
- **Center Panel**: Switchable between Preview, Scene Editor, and Sequencer
- **Right Panel**: Resource browser for prefabs, shaders, and media

### Creating a Scene

1. **Visual Programming**: Use the Scene Editor in the center panel to create node-based visualizations
2. **Scripting**: Write Rhai scripts for procedural animations
3. **Audio Reactivity**: Connect frequency bands to visual parameters

### Example Script

```rhai
// Create an audio-reactive rotating cube
let cube = create_cube();

fn update(time) {
    let bass = get_bass();
    let scale = 1.0 + bass * 2.0;
    
    set_scale(cube, scale, scale, scale);
    set_rotation(cube, 0.0, time * 2.0, 0.0);
}
```

## 🛠️ Development

### Project Structure

```
crates/
├── vibevj-common/
│   ├── src/
│   │   ├── types.rs       # Common types (Color, Transform, Rect, etc.)
│   │   ├── error.rs       # Error types
│   │   └── lib.rs
│   └── Cargo.toml
├── vibevj-engine/
│   ├── src/
│   │   ├── renderer.rs    # WGPU renderer
│   │   ├── pipeline.rs    # Pipeline management
│   │   ├── shader.rs      # Shader system
│   │   ├── camera.rs      # Camera controls
│   │   ├── mesh.rs        # 3D meshes
│   │   ├── texture.rs     # Texture management
│   │   └── lib.rs
│   └── Cargo.toml
├── vibevj-gui/
│   ├── src/
│   │   ├── app.rs         # Main GUI application
│   │   ├── panels.rs      # Three-panel layout
│   │   ├── widgets.rs     # Custom widgets
│   │   └── lib.rs
│   └── Cargo.toml
├── vibevj-audio/
│   ├── src/
│   │   ├── input.rs       # Audio input capture
│   │   ├── analyzer.rs    # FFT analysis
│   │   ├── frequency.rs   # Frequency bands
│   │   └── lib.rs
│   └── Cargo.toml
├── vibevj-scene/
│   ├── src/
│   │   ├── node.rs        # Scene nodes
│   │   ├── scene.rs       # Scene management
│   │   ├── component.rs   # Component system
│   │   ├── graph.rs       # Node graph for visual programming
│   │   └── lib.rs
│   └── Cargo.toml
└── vibevj-scripting/
    ├── src/
    │   ├── engine.rs      # Rhai engine wrapper
    │   ├── api.rs         # VibeVJ API bindings
    │   └── lib.rs
    └── Cargo.toml
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p vibevj-engine

# Run tests with logging
RUST_LOG=debug cargo test
```

### Code Style

This project follows standard Rust conventions:
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common issues
- Use meaningful variable and function names
- Document public APIs

## 📚 Documentation

- [ROADMAP.md](ROADMAP.md) - Development roadmap and feature planning
- API Documentation: Run `cargo doc --open` to view API docs

## 🤝 Contributing

Contributions are welcome! Whether you're fixing bugs, adding features, or improving documentation, your help is appreciated.

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Write tests for new features
- Update documentation as needed
- Follow Rust best practices
- Keep commits focused and atomic
- Add comments for complex logic

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license for your use.

## 🙏 Acknowledgments

VibeVJ is built with amazing open-source technologies:

- [wgpu](https://wgpu.rs/) - Modern GPU graphics API
- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [winit](https://github.com/rust-windowing/winit) - Window handling
- [cpal](https://github.com/RustAudio/cpal) - Audio I/O
- [rustfft](https://github.com/ejmahler/RustFFT) - FFT implementation
- [rhai](https://rhai.rs/) - Scripting language
- [glam](https://github.com/bitshifter/glam-rs) - Math library

## 🔗 Links

- [GitHub Repository](https://github.com/Loigor/vibevj)
- [Issue Tracker](https://github.com/Loigor/vibevj/issues)
- [Discussions](https://github.com/Loigor/vibevj/discussions)

## 🎯 Project Status

VibeVJ is currently in **alpha** development. The core architecture is in place, and we're actively building out features. Expect breaking changes as we refine the API and architecture.

Current focus: Phase 1 completion and Phase 2 rendering features.

---

**Made with ❤️ and Rust**
