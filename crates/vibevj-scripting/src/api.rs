use rhai::Engine;

/// Register VibeVJ API with the Rhai engine
pub fn register_api(engine: &mut Engine) {
    // Register types
    register_math_functions(engine);
    register_scene_functions(engine);
    register_audio_functions(engine);
    register_utility_functions(engine);
}

/// Register mathematical functions
fn register_math_functions(engine: &mut Engine) {
    // Vector operations
    engine.register_fn("vec3", |x: f32, y: f32, z: f32| -> (f32, f32, f32) {
        (x, y, z)
    });

    // Lerp function
    engine.register_fn("lerp", |a: f32, b: f32, t: f32| -> f32 {
        a + (b - a) * t
    });

    // Smoothstep
    engine.register_fn("smoothstep", |edge0: f32, edge1: f32, x: f32| -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    });
}

/// Register scene manipulation functions
fn register_scene_functions(engine: &mut Engine) {
    // Scene node creation (placeholder implementations)
    engine.register_fn("create_cube", || -> String {
        log::info!("Script: Creating cube");
        "cube".to_string()
    });

    engine.register_fn("create_sphere", || -> String {
        log::info!("Script: Creating sphere");
        "sphere".to_string()
    });

    engine.register_fn("set_position", |_node: String, x: f32, y: f32, z: f32| {
        log::info!("Script: Setting position to ({}, {}, {})", x, y, z);
    });

    engine.register_fn("set_rotation", |_node: String, x: f32, y: f32, z: f32| {
        log::info!("Script: Setting rotation to ({}, {}, {})", x, y, z);
    });

    engine.register_fn("set_scale", |_node: String, x: f32, y: f32, z: f32| {
        log::info!("Script: Setting scale to ({}, {}, {})", x, y, z);
    });
}

/// Register audio-reactive functions
fn register_audio_functions(engine: &mut Engine) {
    engine.register_fn("get_bass", || -> f32 {
        // This would connect to the actual audio analyzer
        0.5
    });

    engine.register_fn("get_mid", || -> f32 {
        0.5
    });

    engine.register_fn("get_treble", || -> f32 {
        0.5
    });

    engine.register_fn("get_energy", || -> f32 {
        0.5
    });
}

/// Register utility functions
fn register_utility_functions(engine: &mut Engine) {
    engine.register_fn("log", |msg: &str| {
        log::info!("Script: {}", msg);
    });

    engine.register_fn("random", || -> f32 {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};
        
        let s = RandomState::new();
        let mut hasher = s.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        (hasher.finish() % 10000) as f32 / 10000.0
    });
}

/// Example script templates
pub mod examples {
    pub const ROTATING_CUBE: &str = r#"
// Create a rotating cube
let cube = create_cube();
let angle = 0.0;

fn update(time) {
    angle = time * 2.0;
    set_rotation(cube, 0.0, angle, 0.0);
}
"#;

    pub const AUDIO_REACTIVE: &str = r#"
// Audio-reactive sphere
let sphere = create_sphere();

fn update(time) {
    let bass = get_bass();
    let scale = 1.0 + bass * 2.0;
    set_scale(sphere, scale, scale, scale);
}
"#;

    pub const PROCEDURAL_ANIMATION: &str = r#"
// Procedural animation
let cube = create_cube();

fn update(time) {
    let x = sin(time) * 3.0;
    let y = cos(time * 2.0) * 2.0;
    let z = sin(time * 0.5) * 1.0;
    set_position(cube, x, y, z);
}
"#;
}
