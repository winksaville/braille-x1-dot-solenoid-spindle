use truck_modeling::*;
use truck_topology::*;
use truck_meshalgo::*;
use truck_polymesh::*;
use std::fs::File;
use std::io::Write;

fn main() {
    // Define parameters
    let spindle_diameter = 2.0; // mm, internal diameter for the pin
    let spindle_length = 20.0; // mm, length of the solenoid
    let flange_diameter = 6.0; // mm, outer diameter of the flanges
    let flange_thickness = 1.5; // mm, thickness of the end flanges
    let winding_area_diameter = 4.68; // mm (2.34 * 2), max winding area width

    // Create the main spindle body
    let spindle = Solid::cylinder(winding_area_diameter / 2.0, spindle_length);
    
    // Hollow out the center for the pin
    let hole = Solid::cylinder(spindle_diameter / 2.0, spindle_length);
    let spindle = spindle.difference(&hole);

    // Add flanges to both ends
    let flange1 = Solid::cylinder(flange_diameter / 2.0, flange_thickness);
    let flange2 = flange1.translate((0.0, 0.0, spindle_length - flange_thickness));
    
    // Combine the spindle and flanges
    let spindle = spindle.union(&flange1).union(&flange2);
    
    // Export as a STEP file
    let step_data = spindle.to_step();
    let mut file = File::create("solenoid_spindle.step").expect("Failed to create file");
    file.write_all(step_data.as_bytes()).expect("Failed to write file");
}
