use macroquad::prelude::*;

mod arrow;

use arrow::Arrow;

#[macroquad::main("Moving Arrows")]
async fn main() {
    let mut arrows = Vec::new();
    
    // Create multiple arrows with different properties
    // Each arrow has: position, velocity, acceleration, size, color
    arrows.push(Arrow::new(
        Vec2::new(0.0, 50.0),
        Vec2::new(20.0, 0.0),  // Initial velocity (speed 20, moving right)
        Vec2::new(0.1, 0.0),   // Small acceleration
        40.0,
        RED
    ));
    arrows.push(Arrow::new(
        Vec2::new(0.0, 120.0),
        Vec2::new(30.0, 5.0),
        Vec2::new(0.0, 0.05),
        50.0,
        BLUE
    ));
    arrows.push(Arrow::new(
        Vec2::new(0.0, 200.0),
        Vec2::new(15.0, -3.0),
        Vec2::new(0.15, 0.0),
        35.0,
        GREEN
    ));
    arrows.push(Arrow::new(
        Vec2::new(0.0, 280.0),
        Vec2::new(25.0, 2.0),
        Vec2::new(-0.05, 0.1),
        45.0,
        YELLOW
    ));
    arrows.push(Arrow::new(
        Vec2::new(0.0, 360.0),
        Vec2::new(18.0, -5.0),
        Vec2::new(0.2, -0.1),
        55.0,
        MAGENTA
    ));
    
    loop {
        clear_background(BLACK);
        
        let screen_width = screen_width();
        let screen_height = screen_height();
        let mouse_pos = mouse_position();
        let mouse_pos_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
        
        // Collect neighbor data for flocking (before mutating)
        use arrow::NeighborData;
        let neighbor_data: Vec<NeighborData> = arrows.iter().map(|a| NeighborData {
            position: a.position(),
            velocity: a.velocity(),
            size: a.size(),
        }).collect();
        
        // Update all arrows with flocking behavior
        for arrow in &mut arrows {
            arrow.update(screen_width, screen_height, mouse_pos_vec, &neighbor_data);
        }
        
        // Draw all arrows
        for arrow in &arrows {
            arrow.draw();
        }
        
        next_frame().await;
    }
}

