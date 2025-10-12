# Building VibeVJ

## Prerequisites

### Windows
1. **Install Rust**: Download from [rustup.rs](https://rustup.rs/)
2. **Visual Studio Build Tools**: Install "Desktop development with C++" workload
3. **GPU Drivers**: Ensure you have the latest drivers for your GPU

### Linux
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libasound2-dev

# Fedora
sudo dnf install gcc pkg-config alsa-lib-devel

# Arch
sudo pacman -S base-devel alsa-lib
```

### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

## Building

### Development Build (Fast compilation)
```bash
cargo build
```

### Release Build (Optimized for performance)
```bash
cargo build --release
```

### Run Debug Build
```bash
cargo run
```

### Run Release Build
```bash
cargo run --release
```

## Build Troubleshooting

### Issue: "linking with `link.exe` failed" (Windows)
**Solution**: Install Visual Studio Build Tools with C++ workload

### Issue: "could not find `alsa`" (Linux)
**Solution**: Install ALSA development package
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev
```

### Issue: WGPU initialization fails
**Solutions**:
1. Update GPU drivers
2. Check GPU supports Vulkan/DX12/Metal
3. Run with logging:
```bash
$env:RUST_LOG="debug" ; cargo run --release  # Windows
RUST_LOG=debug cargo run --release           # Linux/macOS
```

### Issue: Audio input not working
**Solutions**:
- Windows: Enable "Stereo Mix" in Recording Devices
- Linux: Check PulseAudio/PipeWire is running
- macOS: Grant microphone permissions
- Test with physical microphone if system audio capture unavailable

## Build Times

**First Build** (downloads and compiles all dependencies):
- Debug: ~5-10 minutes
- Release: ~10-15 minutes

**Incremental Builds** (after first build):
- Debug: ~5-30 seconds
- Release: ~30-60 seconds

## Build Optimization

### Faster Debug Builds
Add to `~/.cargo/config.toml` (create if doesn't exist):

```toml
[build]
# Use lld linker for faster linking (if available)
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[profile.dev]
# Optimize dependencies in dev mode
opt-level = 1
```

### Faster Incremental Compiles
```toml
[profile.dev.package."*"]
opt-level = 3
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p vibevj-engine

# Run tests with output
cargo test -- --nocapture
```

## Checking Code

```bash
# Check code compiles without building
cargo check

# Check all workspace members
cargo check --workspace

# Check and fix warnings
cargo clippy

# Auto-fix fixable warnings
cargo fix
```

## Formatting

```bash
# Check formatting
cargo fmt --check

# Auto-format code
cargo fmt
```

## Documentation

```bash
# Build and open documentation
cargo doc --open

# Build docs for all dependencies
cargo doc --open --document-private-items
```

## Build Artifacts

Build artifacts are stored in `target/`:
```
target/
├── debug/          # Debug builds
│   └── vibevj.exe  # Debug executable
└── release/        # Release builds
    └── vibevj.exe  # Optimized executable
```

## Platform-Specific Notes

### Windows
- Use PowerShell or Windows Terminal
- GPU backend: DirectX 12 (primary), Vulkan (fallback)
- Audio backend: WASAPI

### Linux
- GPU backend: Vulkan
- Audio backend: PulseAudio/PipeWire/ALSA
- May need to run with `WAYLAND_DISPLAY` or `DISPLAY` env vars

### macOS
- GPU backend: Metal
- Audio backend: CoreAudio
- May require code signing for distribution

## Environment Variables

Useful environment variables:

```bash
# Enable debug logging
$env:RUST_LOG="debug"      # Windows
export RUST_LOG=debug       # Linux/macOS

# Force specific backend
$env:WGPU_BACKEND="vulkan"  # Windows
export WGPU_BACKEND=vulkan  # Linux/macOS

# Enable backtrace on panic
$env:RUST_BACKTRACE=1       # Windows
export RUST_BACKTRACE=1     # Linux/macOS
```

## Clean Build

```bash
# Remove build artifacts
cargo clean

# Clean and rebuild
cargo clean && cargo build --release
```

## Build Profiles

### Dev (default for `cargo build`)
- Fast compilation
- Debug symbols included
- Some optimizations for dependencies

### Release (use `--release` flag)
- Full optimizations
- LTO enabled
- Longer compile time
- Smaller binary
- Better performance

### Custom Profile
Add to `Cargo.toml`:
```toml
[profile.my-profile]
inherits = "release"
lto = "fat"
codegen-units = 1
strip = true
```

Use with: `cargo build --profile my-profile`

## Binary Size

Release binary size: ~20-50 MB (varies by platform)

To reduce size:
```toml
[profile.release]
strip = true      # Remove symbols
lto = "fat"       # Full LTO
codegen-units = 1 # Single codegen unit
opt-level = "z"   # Optimize for size
```

## Cross-Compilation

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build for target
cargo build --target x86_64-unknown-linux-gnu --release
```

## CI/CD

Example GitHub Actions workflow:

```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: cargo test
```

## Troubleshooting Build Errors

1. **Update Rust**: `rustup update`
2. **Clean build**: `cargo clean && cargo build`
3. **Update dependencies**: `cargo update`
4. **Check Rust version**: `rustc --version` (should be 1.70+)
5. **Read error messages**: Rust provides excellent error messages

## Getting Help

If you encounter build issues:
1. Check error message carefully
2. Search GitHub issues
3. Ask in Discussions
4. Provide error output and system info

## Next Steps

After successful build, see:
- **QUICKSTART.md** for usage guide
- **README.md** for project overview
- **ROADMAP.md** for development plan
