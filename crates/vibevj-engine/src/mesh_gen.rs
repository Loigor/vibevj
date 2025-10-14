use super::mesh::{Mesh, Vertex};
use std::f32::consts::PI;

/// Generate a cube mesh with specified size
pub fn create_cube(size: f32) -> Mesh {
    let s = size / 2.0;
    
    let vertices = vec![
        // Front face (Z+)
        Vertex::new([-s, -s, s], [0.0, 0.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([s, -s, s], [0.0, 0.0, 1.0], [1.0, 0.0], [1.0, 0.0, 0.0]),
        Vertex::new([s, s, s], [0.0, 0.0, 1.0], [1.0, 1.0], [1.0, 0.0, 0.0]),
        Vertex::new([-s, s, s], [0.0, 0.0, 1.0], [0.0, 1.0], [1.0, 0.0, 0.0]),
        
        // Back face (Z-)
        Vertex::new([s, -s, -s], [0.0, 0.0, -1.0], [0.0, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([-s, -s, -s], [0.0, 0.0, -1.0], [1.0, 0.0], [0.0, 1.0, 0.0]),
        Vertex::new([-s, s, -s], [0.0, 0.0, -1.0], [1.0, 1.0], [0.0, 1.0, 0.0]),
        Vertex::new([s, s, -s], [0.0, 0.0, -1.0], [0.0, 1.0], [0.0, 1.0, 0.0]),
        
        // Right face (X+)
        Vertex::new([s, -s, s], [1.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([s, -s, -s], [1.0, 0.0, 0.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
        Vertex::new([s, s, -s], [1.0, 0.0, 0.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        Vertex::new([s, s, s], [1.0, 0.0, 0.0], [0.0, 1.0], [0.0, 0.0, 1.0]),
        
        // Left face (X-)
        Vertex::new([-s, -s, -s], [-1.0, 0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 0.0]),
        Vertex::new([-s, -s, s], [-1.0, 0.0, 0.0], [1.0, 0.0], [1.0, 1.0, 0.0]),
        Vertex::new([-s, s, s], [-1.0, 0.0, 0.0], [1.0, 1.0], [1.0, 1.0, 0.0]),
        Vertex::new([-s, s, -s], [-1.0, 0.0, 0.0], [0.0, 1.0], [1.0, 1.0, 0.0]),
        
        // Top face (Y+)
        Vertex::new([-s, s, s], [0.0, 1.0, 0.0], [0.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([s, s, s], [0.0, 1.0, 0.0], [1.0, 0.0], [1.0, 0.0, 1.0]),
        Vertex::new([s, s, -s], [0.0, 1.0, 0.0], [1.0, 1.0], [1.0, 0.0, 1.0]),
        Vertex::new([-s, s, -s], [0.0, 1.0, 0.0], [0.0, 1.0], [1.0, 0.0, 1.0]),
        
        // Bottom face (Y-)
        Vertex::new([-s, -s, -s], [0.0, -1.0, 0.0], [0.0, 0.0], [0.0, 1.0, 1.0]),
        Vertex::new([s, -s, -s], [0.0, -1.0, 0.0], [1.0, 0.0], [0.0, 1.0, 1.0]),
        Vertex::new([s, -s, s], [0.0, -1.0, 0.0], [1.0, 1.0], [0.0, 1.0, 1.0]),
        Vertex::new([-s, -s, s], [0.0, -1.0, 0.0], [0.0, 1.0], [0.0, 1.0, 1.0]),
    ];
    
    let indices = vec![
        0, 1, 2, 2, 3, 0,       // Front
        4, 5, 6, 6, 7, 4,       // Back
        8, 9, 10, 10, 11, 8,    // Right
        12, 13, 14, 14, 15, 12, // Left
        16, 17, 18, 18, 19, 16, // Top
        20, 21, 22, 22, 23, 20, // Bottom
    ];
    
    Mesh::new(vertices, indices)
}

/// Generate a UV sphere mesh
pub fn create_sphere(radius: f32, segments: u32, rings: u32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Generate vertices
    for ring in 0..=rings {
        let phi = PI * ring as f32 / rings as f32;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        
        for segment in 0..=segments {
            let theta = 2.0 * PI * segment as f32 / segments as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            let x = sin_phi * cos_theta;
            let y = cos_phi;
            let z = sin_phi * sin_theta;
            
            let u = segment as f32 / segments as f32;
            let v = ring as f32 / rings as f32;
            
            // Color based on position
            let color = [
                (x + 1.0) * 0.5,
                (y + 1.0) * 0.5,
                (z + 1.0) * 0.5,
            ];
            
            vertices.push(Vertex::new(
                [x * radius, y * radius, z * radius],
                [x, y, z], // Normal is same as normalized position for sphere
                [u, v],
                color,
            ));
        }
    }
    
    // Generate indices
    for ring in 0..rings {
        for segment in 0..segments {
            let current = ring * (segments + 1) + segment;
            let next = current + segments + 1;
            
            indices.push(current);
            indices.push(next);
            indices.push(current + 1);
            
            indices.push(current + 1);
            indices.push(next);
            indices.push(next + 1);
        }
    }
    
    Mesh::new(vertices, indices)
}

/// Generate a plane mesh
pub fn create_plane(width: f32, height: f32, subdivisions_x: u32, subdivisions_y: u32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    
    // Generate vertices
    for y in 0..=subdivisions_y {
        let v = y as f32 / subdivisions_y as f32;
        let py = -half_height + height * v;
        
        for x in 0..=subdivisions_x {
            let u = x as f32 / subdivisions_x as f32;
            let px = -half_width + width * u;
            
            vertices.push(Vertex::new(
                [px, py, 0.0],
                [0.0, 0.0, 1.0], // Normal pointing up (Z+)
                [u, v],
                [1.0, 1.0, 1.0], // White color
            ));
        }
    }
    
    // Generate indices
    for y in 0..subdivisions_y {
        for x in 0..subdivisions_x {
            let base = y * (subdivisions_x + 1) + x;
            let next_row = base + subdivisions_x + 1;
            
            indices.push(base);
            indices.push(next_row);
            indices.push(base + 1);
            
            indices.push(base + 1);
            indices.push(next_row);
            indices.push(next_row + 1);
        }
    }
    
    Mesh::new(vertices, indices)
}

/// Generate a cylinder mesh
pub fn create_cylinder(radius: f32, height: f32, segments: u32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let half_height = height / 2.0;
    
    // Generate side vertices
    for ring in 0..=1 {
        let y = if ring == 0 { -half_height } else { half_height };
        let v = ring as f32;
        
        for segment in 0..=segments {
            let theta = 2.0 * PI * segment as f32 / segments as f32;
            let x = theta.cos();
            let z = theta.sin();
            
            let u = segment as f32 / segments as f32;
            
            vertices.push(Vertex::new(
                [x * radius, y, z * radius],
                [x, 0.0, z], // Normal points outward
                [u, v],
                [1.0, 1.0, 1.0],
            ));
        }
    }
    
    // Generate side indices
    for segment in 0..segments {
        let base = segment;
        let next = segment + 1;
        let top_base = base + segments + 1;
        let top_next = next + segments + 1;
        
        indices.push(base);
        indices.push(top_base);
        indices.push(next);
        
        indices.push(next);
        indices.push(top_base);
        indices.push(top_next);
    }
    
    // Add caps (simplified)
    let cap_center_bottom = vertices.len() as u32;
    vertices.push(Vertex::new(
        [0.0, -half_height, 0.0],
        [0.0, -1.0, 0.0],
        [0.5, 0.5],
        [1.0, 1.0, 1.0],
    ));
    
    let cap_center_top = vertices.len() as u32;
    vertices.push(Vertex::new(
        [0.0, half_height, 0.0],
        [0.0, 1.0, 0.0],
        [0.5, 0.5],
        [1.0, 1.0, 1.0],
    ));
    
    Mesh::new(vertices, indices)
}
