use glam::{Vec2, Vec3, Vec4, Mat4};
use serde::{Deserialize, Serialize};

/// Common color type with RGBA channels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_vec4(v: Vec4) -> Self {
        Self { r: v.x, g: v.y, b: v.z, a: v.w }
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

/// Transform in 3D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            glam::Quat::from_euler(glam::EulerRot::XYZ, self.rotation.x, self.rotation.y, self.rotation.z),
            self.position,
        )
    }
}

/// Rectangle for 2D operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x 
            && point.x <= self.x + self.width 
            && point.y >= self.y 
            && point.y <= self.y + self.height
    }
}

/// Time information for animations and synchronization
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TimeInfo {
    /// Total elapsed time in seconds
    pub elapsed: f64,
    /// Delta time since last frame in seconds
    pub delta: f32,
    /// Current frame number
    pub frame: u64,
}
