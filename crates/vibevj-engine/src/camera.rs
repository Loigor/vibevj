use glam::{Mat4, Vec3};
use vibevj_common::Transform;

/// Camera for 3D rendering
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Create a new perspective camera
    pub fn new(position: Vec3, target: Vec3, aspect: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    /// Get the view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Get the combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Update aspect ratio
    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::ZERO,
            16.0 / 9.0,
        )
    }
}
