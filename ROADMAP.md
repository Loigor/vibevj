# VibeVJ Roadmap

## Project Overview
VibeVJ is a Visual Jockey (VJ) application for creating and running real-time visualizations. It combines a powerful 3D rendering engine with audio analysis and a node-based visual programming interface.

## Development Phases

### Phase 1: Core Foundation ✓ (Current)
**Status**: Scaffolding Complete

#### Completed
- [x] Project structure and workspace setup
- [x] Core module architecture (engine, GUI, audio, scene, scripting, common)
- [x] WGPU-based rendering engine foundation
- [x] egui GUI integration with 3-panel layout
- [x] Audio input and FFT analysis module
- [x] Scene graph and node system
- [x] Rhai scripting integration
- [x] Basic component system

#### Next Steps
- [ ] Implement basic 3D rendering (cube, sphere primitives)
- [ ] Test audio input and frequency band extraction
- [ ] Create first working shader example
- [ ] Validate GUI layout and panel interactions

---

### Phase 2: Basic Rendering & Visualization
**Target**: Functional 3D rendering with audio reactivity

#### Rendering Engine
- [ ] Complete mesh rendering system
- [ ] Implement basic lighting (directional, point, spot)
- [ ] Shader compilation and hot-reloading
- [ ] Texture loading and management
- [ ] Camera controls (orbit, pan, zoom)
- [ ] Render targets and post-processing

#### Shader System
- [ ] WGSL shader editor in GUI
- [ ] ShaderToy-style fragment shader support
- [ ] Uniform buffer management
- [ ] Time and resolution uniforms
- [ ] Audio uniforms (bass, mid, treble bands)

#### Audio Integration
- [ ] Real-time audio input visualization
- [ ] Beat detection algorithm
- [ ] Configurable frequency band ranges
- [ ] Audio buffer visualization widget
- [ ] Audio file playback support

---

### Phase 3: Visual Programming Interface
**Target**: Node-based scene creation

#### Node Graph Editor
- [ ] Node graph widget in center panel
- [ ] Drag-and-drop node creation
- [ ] Visual connection system
- [ ] Node parameter editing
- [ ] Node type library
  - [ ] Input nodes (Time, Audio, Mouse)
  - [ ] Math nodes (Add, Multiply, Sin, Cos, etc.)
  - [ ] Transform nodes (Position, Rotation, Scale)
  - [ ] Shader nodes
  - [ ] Output nodes (Geometry, Color)

#### Node Evaluation
- [ ] Graph topology sorting
- [ ] Data flow execution
- [ ] Node caching system
- [ ] Error handling and validation
- [ ] Live preview updates

---

### Phase 4: Scene Management & Timeline
**Target**: Multi-scene and timeline-based animations

#### Scene System
- [ ] Scene save/load (JSON format)
- [ ] Scene templates and presets
- [ ] Multiple scene management
- [ ] Scene transitions and crossfading
- [ ] Scene hotkeys and quick switching

#### Timeline Editor
- [ ] Timeline track visualization
- [ ] Keyframe editing
- [ ] Animation curves
- [ ] Track types:
  - [ ] Transform tracks
  - [ ] Parameter tracks
  - [ ] Shader parameter tracks
  - [ ] Audio trigger tracks
- [ ] Playback controls
- [ ] Timeline scrubbing
- [ ] Loop regions

---

### Phase 5: 2D Visualization Support
**Target**: 2D graphics, images, videos, GIFs

#### 2D Rendering
- [ ] 2D sprite renderer
- [ ] Image loading and display
- [ ] GIF animation support
- [ ] Video playback integration
- [ ] 2D drawing primitives (lines, circles, rectangles)
- [ ] Canvas-style 2D API

#### Media Management
- [ ] Asset browser in right panel
- [ ] Image thumbnail previews
- [ ] Video preview and scrubbing
- [ ] Media import/export
- [ ] Resource packaging

---

### Phase 6: Advanced Scripting
**Target**: Full programmatic control

#### Scripting Features
- [ ] Complete Rhai API documentation
- [ ] Script editor with syntax highlighting
- [ ] Script debugging and error reporting
- [ ] Script hot-reloading
- [ ] Example script library
- [ ] Script sharing and packaging

#### API Extensions
- [ ] Scene manipulation API
- [ ] Animation API
- [ ] Audio-reactive helpers
- [ ] Procedural generation utilities
- [ ] Network/OSC integration

---

### Phase 7: Performance & Polish
**Target**: Production-ready application

#### Performance
- [ ] GPU compute shader support
- [ ] Instanced rendering
- [ ] LOD system
- [ ] Occlusion culling
- [ ] Frame pacing and VSync options
- [ ] Performance profiler

#### User Experience
- [ ] Keyboard shortcuts system
- [ ] Customizable layouts
- [ ] Theme support (dark/light)
- [ ] Preferences and settings
- [ ] Tutorial system
- [ ] Documentation and help system

---

### Phase 8: Community & Export
**Target**: Sharing and production capabilities

#### Export Features
- [ ] Video recording (MP4, WebM)
- [ ] Image sequence export
- [ ] Scene export for web (WASM)
- [ ] Standalone executable export
- [ ] Preset/scene sharing format

#### Community
- [ ] Plugin system architecture
- [ ] Custom node types API
- [ ] Shader library
- [ ] Community scene repository
- [ ] Integration with VJ hardware (MIDI, OSC)

---

## Technical Milestones

### Milestone 1: First Working Demo (2-3 weeks)
- Basic 3D rendering with one shader
- Audio input working with visualization
- GUI functional with all three panels
- One example scene

### Milestone 2: Node Editor Alpha (1-2 months)
- Node graph editor working
- 10+ node types implemented
- Can create simple scenes with nodes
- Scene save/load working

### Milestone 3: Timeline Beta (2-3 months)
- Timeline editor functional
- Keyframe animation working
- Multiple scenes with transitions
- 2D image/video support

### Milestone 4: Public Release v1.0 (4-6 months)
- Full feature set from Phases 1-6
- Documentation complete
- Example scenes and tutorials
- Stable and performant

---

## Architecture Notes

### Module Responsibilities

**vibevj-common**: Shared types, error handling, math utilities
**vibevj-engine**: WGPU rendering, pipelines, shaders, cameras, meshes
**vibevj-gui**: egui interface, panels, widgets, node editor
**vibevj-audio**: Audio input, FFT analysis, frequency bands, beat detection
**vibevj-scene**: Scene graph, nodes, components, serialization
**vibevj-scripting**: Rhai integration, API bindings, script execution

### Design Principles
1. **Modularity**: Each crate has clear boundaries and minimal dependencies
2. **Performance**: GPU-first approach, minimize CPU work
3. **Extensibility**: Plugin system for custom nodes and components
4. **User-Friendly**: Intuitive GUI for beginners, powerful scripting for advanced users
5. **Real-Time**: 60+ FPS target for smooth visualizations

---

## Future Ideas (Post-1.0)

- **VR Support**: Render to VR headsets
- **Network Sync**: Multiple instances synchronized over network
- **AI Integration**: AI-generated shaders and animations
- **Live Coding**: Live shader coding interface
- **Mobile Support**: Android/iOS version
- **Cloud Rendering**: Remote GPU rendering
- **Collaboration**: Multi-user scene editing

---

## Contributing

This roadmap is a living document. As development progresses, priorities may shift based on:
- Community feedback
- Technical challenges
- New opportunities and technologies
- User needs and use cases

## Notes

- Dates are estimates and subject to change
- Features may be moved between phases
- Additional features may be added based on feedback
- Performance targets may be adjusted based on hardware capabilities
