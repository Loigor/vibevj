# VibeVJ Quick Start Guide

## Installation

### 1. Install Prerequisites

**Rust**: Install from [rustup.rs](https://rustup.rs/)
```bash
# Windows (PowerShell)
# Download and run rustup-init.exe from rustup.rs

# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**GPU Drivers**: Ensure you have up-to-date graphics drivers
- NVIDIA: Latest Game Ready or Studio drivers
- AMD: Latest Adrenalin drivers  
- Intel: Latest graphics drivers

### 2. Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/vibevj.git
cd vibevj

# Build the project (first build will take a few minutes)
cargo build --release

# Run the application
cargo run --release
```

## First Launch

When you first launch VibeVJ, you'll see a three-panel interface:

### Left Panel (25%)
- **Render Preview**: Shows the output of your visualization
- **Controls**: Play/Pause/Stop buttons
- **Stats**: FPS and frame counter
- **Scene Settings**: Background color, resolution, quality

### Center Panel (60%)
- **Preview Tab**: Full-screen visualization preview
- **Scene Editor Tab**: Node-based visual programming
- **Sequencer Tab**: scene sequencer (coming soon)

### Right Panel (15%)
- **Resources**: Browser for prefabs, shaders, textures
- **Search**: Find resources quickly
- **Quick Actions**: Import new resources

## Creating Your First Visualization

### Method 1: Using Scripts

1. Create a new file `examples/my_first_scene.rhai`:

```rhai
// Create a rotating cube that reacts to bass
let cube = create_cube();

fn update(time) {
    // Get audio data
    let bass = get_bass();
    
    // Scale based on bass
    let scale = 1.0 + bass * 2.0;
    set_scale(cube, scale, scale, scale);
    
    // Rotate over time
    set_rotation(cube, 0.0, time * 2.0, 0.0);
}
```

2. Load and run the script in VibeVJ

### Method 2: Using Node Graph (Coming Soon)

1. Click **Scene Editor** tab in center panel
2. Drag nodes from the right panel
3. Connect nodes to create data flow
4. See results in real-time

## Audio Setup

VibeVJ automatically tries to capture audio from your default input device.

### Windows
- Right-click speaker icon → Sounds → Recording
- Enable "Stereo Mix" to capture system audio
- Or use a physical microphone for live audio

### macOS
- Use a virtual audio device like BlackHole
- Or use a physical microphone

### Linux
- Use PulseAudio or PipeWire
- `pactl list sources` to see available sources

## Keyboard Shortcuts (Planned)

- `Space`: Play/Pause
- `Ctrl+N`: New scene
- `Ctrl+O`: Open scene
- `Ctrl+S`: Save scene
- `Ctrl+Shift+S`: Save scene as
- `F11`: Toggle fullscreen
- `Tab`: Cycle between panels

## Performance Tips

### Getting Best Performance

1. **Use Release Build**: Always run `cargo run --release` for performance
2. **GPU Acceleration**: Ensure WGPU is using your dedicated GPU
3. **Resolution**: Lower resolution = higher FPS
4. **Shader Complexity**: Simpler shaders run faster

### Troubleshooting

**Application won't start:**
- Check GPU drivers are up to date
- Ensure you have Vulkan/DirectX 12/Metal support
- Run with `RUST_LOG=debug cargo run` to see detailed logs

**No audio input:**
- Check your audio device is working
- Try selecting a different input device
- Enable "Stereo Mix" on Windows

**Low FPS:**
- Lower the resolution
- Simplify your shaders
- Close other GPU-intensive applications
- Check you're using the release build

## Example Scenes

### 1. Audio-Reactive Sphere

```rhai
let sphere = create_sphere();

fn update(time) {
    let energy = get_energy();
    let bass = get_bass();
    let treble = get_treble();
    
    // Scale with energy
    let scale = 1.0 + energy;
    set_scale(sphere, scale, scale, scale);
    
    // Color based on frequency bands
    // (Color control coming soon)
}
```

### 2. Procedural Animation

```rhai
let cube = create_cube();

fn update(time) {
    let x = sin(time) * 3.0;
    let y = cos(time * 2.0) * 2.0;
    let z = sin(time * 0.5);
    
    set_position(cube, x, y, z);
    set_rotation(cube, time, time * 0.5, time * 1.5);
}
```

### 3. Multi-Object Scene

```rhai
let cube1 = create_cube();
let cube2 = create_cube();
let sphere = create_sphere();

fn update(time) {
    let bass = get_bass();
    
    // Cube 1 - follows bass
    set_position(cube1, -2.0, 0.0, 0.0);
    set_scale(cube1, 1.0 + bass, 1.0 + bass, 1.0 + bass);
    
    // Cube 2 - rotates
    set_position(cube2, 2.0, 0.0, 0.0);
    set_rotation(cube2, time, time * 2.0, 0.0);
    
    // Sphere - orbits
    let x = cos(time) * 3.0;
    let z = sin(time) * 3.0;
    set_position(sphere, x, 0.0, z);
}
```

## Next Steps

1. **Explore the Code**: Check out the different modules in `crates/`
2. **Read the Roadmap**: See what features are coming in [ROADMAP.md](ROADMAP.md)
3. **Experiment**: Try modifying the example scripts
4. **Contribute**: Found a bug or want to add a feature? We welcome contributions!

## Learning Resources

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Graphics Programming
- [Learn WGPU](https://sotrh.github.io/learn-wgpu/)
- [The Book of Shaders](https://thebookofshaders.com/)
- [Shader Toy](https://www.shadertoy.com/)

### Audio Programming
- [Digital Audio Signal Processing](https://www.dspguide.com/)
- [Music DSP](https://www.musicdsp.org/)

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Discussions**: Ask questions and share your creations
- **Discord**: Join our community (link coming soon)

## What's Next?

Check out the [ROADMAP.md](ROADMAP.md) to see what features are being developed and how you can help!

Happy visualizing! 🎨✨
