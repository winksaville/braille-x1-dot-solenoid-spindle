//use nalgebra as na;
use stl_io::{Vertex, Normal, Triangle, write_stl};
use std::fs::File;
use std::io::BufWriter;

fn create_cylinder(radius: f32, height: f32, segments: usize) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let step = std::f32::consts::TAU / segments as f32;
    
    for i in 0..segments {
        let theta1 = i as f32 * step;
        let theta2 = (i + 1) as f32 * step;
        let x1 = radius * theta1.cos();
        let y1 = radius * theta1.sin();
        let x2 = radius * theta2.cos();
        let y2 = radius * theta2.sin();

        // Bottom triangle
        triangles.push(Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [
                Vertex::new([0.0, 0.0, 0.0]),
                Vertex::new([x2, y2, 0.0]),
                Vertex::new([x1, y1, 0.0]),
            ],
        });

        // Top triangle
        triangles.push(Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [
                Vertex::new([0.0, 0.0, height]),
                Vertex::new([x1, y1, height]),
                Vertex::new([x2, y2, height]),
            ],
        });
    }
    
    triangles
}

fn main() {
    let spindle_diameter = 2.0;
    let spindle_length = 20.0;
    let flange_diameter = 6.0;
    let flange_thickness = 1.5;
    let segments = 32;

    let mut triangles = Vec::new();
    
    // Create spindle and flanges
    triangles.extend(create_cylinder(spindle_diameter / 2.0, spindle_length, segments));
    triangles.extend(create_cylinder(flange_diameter / 2.0, flange_thickness, segments));
    triangles.extend(create_cylinder(flange_diameter / 2.0, flange_thickness, segments));
    
    // Export as STL
    let mut file = BufWriter::new(File::create("solenoid_spindle.stl").expect("Failed to create file"));
    write_stl(&mut file, triangles.iter()).expect("Failed to write STL file");
}
