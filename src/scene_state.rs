use vibevj_engine::{RenderObject, Camera};

/// Shared state for rendering the 3D scene
/// This allows rendering the same scene on multiple devices
pub struct SceneState {
    pub camera: Camera,
    pub render_objects: Vec<RenderObject>,
    pub time: f32,
}

impl SceneState {
    pub fn new() -> Self {
        // Create camera
        let camera = Camera::new(
            glam::Vec3::new(3.0, 2.0, 5.0),
            glam::Vec3::ZERO,
            1280.0 / 720.0, // aspect ratio
        );

        Self {
            camera,
            render_objects: Vec::new(),
            time: 0.0,
        }
    }

    /// Update the scene state
    /// Note: Transform updates are done separately using update_transform with queue
    pub fn update(&mut self, time: f32) {
        self.time = time;
    }
}

impl Default for SceneState {
    fn default() -> Self {
        Self::new()
    }
}
