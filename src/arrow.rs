use macroquad::prelude::*;

pub struct NeighborData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
}

pub struct Arrow {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    size: f32,
    color: Color,
    angle: f32,  // Rotation angle in radians
}

impl Arrow {
    const MAX_SPEED: f32 = 5.0;
    const MOUSE_AVOIDANCE_DISTANCE: f32 = 100.0;  // Distance at which arrows start avoiding mouse
    const MOUSE_AVOIDANCE_STRENGTH: f32 = 0.5;     // Strength of avoidance force
    
    // Flocking parameters
    pub const SEPARATION_DISTANCE: f32 = 70.0;    // Distance to maintain from neighbors
    pub const ALIGNMENT_DISTANCE: f32 = 120.0;    // Distance to consider for alignment
    pub const COHESION_DISTANCE: f32 = 200.0;     // Distance to consider for cohesion (increased to help regrouping)
    pub const SEPARATION_STRENGTH: f32 = 0.3;     // Strength of separation force
    pub const ALIGNMENT_STRENGTH: f32 = 0.25;     // Strength of alignment force
    pub const COHESION_STRENGTH: f32 = 0.3;      // Strength of cohesion force (increased to help regrouping)

    pub fn position(&self) -> Vec2 {
        self.position
    }
    
    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }
    
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, size: f32, color: Color) -> Self {
        // Calculate initial angle from velocity direction
        let angle = if velocity.length() > 0.001 {
            velocity.y.atan2(velocity.x)
        } else {
            0.0
        };
        Arrow { position, velocity, acceleration, size, color, angle }
    }

    pub fn update(&mut self, screen_width: f32, screen_height: f32, mouse_pos: Vec2, neighbors: &[NeighborData]) {
        // Calculate arrow center position
        let arrow_center = self.position + Vec2::new(self.size / 2.0, self.size / 2.0);
        
        // Calculate distance to mouse
        let to_mouse = arrow_center - mouse_pos;
        let distance_to_mouse = to_mouse.length();
        
        // Apply mouse avoidance force if mouse is within avoidance distance
        if distance_to_mouse < Self::MOUSE_AVOIDANCE_DISTANCE && distance_to_mouse > 0.001 {
            // Calculate repulsion force (stronger when closer)
            let avoidance_force = (1.0 - distance_to_mouse / Self::MOUSE_AVOIDANCE_DISTANCE) 
                * Self::MOUSE_AVOIDANCE_STRENGTH;
            let avoidance_direction = to_mouse.normalize();
            self.velocity += avoidance_direction * avoidance_force;
        }
        
        // Apply flocking behaviors
        let (separation, alignment, cohesion) = self.calculate_flocking(neighbors);
        self.velocity += separation * Self::SEPARATION_STRENGTH;
        self.velocity += alignment * Self::ALIGNMENT_STRENGTH;
        self.velocity += cohesion * Self::COHESION_STRENGTH;
        
        // Apply acceleration to velocity
        self.velocity += self.acceleration;
        
        // Clamp speed (magnitude of velocity) between 0 and MAX_SPEED
        let speed = self.velocity.length();
        if speed > Self::MAX_SPEED {
            self.velocity = self.velocity.normalize() * Self::MAX_SPEED;
        } else if speed < 0.0 {
            self.velocity = Vec2::ZERO;
        }
        
        // Update angle to point in direction of movement
        if self.velocity.length() > 0.001 {
            self.angle = self.velocity.y.atan2(self.velocity.x);
        }
        
        // Update position based on velocity
        self.position += self.velocity;
        
        // Wrap around when hitting edges
        if self.position.x > screen_width {
            self.position.x = -self.size;
        } else if self.position.x < -self.size {
            self.position.x = screen_width;
        }
        
        if self.position.y > screen_height {
            self.position.y = -self.size;
        } else if self.position.y < -self.size {
            self.position.y = screen_height;
        }
    }

    pub fn draw(&self) {
        // Calculate center point of the arrow
        let center = self.position + Vec2::new(self.size / 2.0, self.size / 2.0);
        
        // Define arrow points relative to center (pointing right by default)
        let half_size = self.size / 2.0;
        let tip_length = self.size * 0.5;
        
        // Points relative to center (before rotation)
        let top_local = Vec2::new(half_size, -half_size);
        let bottom_local = Vec2::new(half_size, half_size);
        let tip_local = Vec2::new(half_size + tip_length, 0.0);
        
        // Rotate points around center
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();
        
        let rotate_point = |p: Vec2| -> Vec2 {
            Vec2::new(
                p.x * cos_a - p.y * sin_a,
                p.x * sin_a + p.y * cos_a
            )
        };
        
        // Rotate and translate to world position
        let top = center + rotate_point(top_local);
        let bottom = center + rotate_point(bottom_local);
        let tip = center + rotate_point(tip_local);
        
        draw_triangle(top, bottom, tip, self.color);
    }

    fn calculate_flocking(&self, neighbors: &[NeighborData]) -> (Vec2, Vec2, Vec2) {
        let arrow_center = self.position + Vec2::new(self.size / 2.0, self.size / 2.0);
        
        let mut separation = Vec2::ZERO;
        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;
        let mut alignment_count = 0;
        let mut cohesion_count = 0;
        
        for neighbor in neighbors {
            let neighbor_center = neighbor.position + Vec2::new(neighbor.size / 2.0, neighbor.size / 2.0);
            let diff = arrow_center - neighbor_center;
            let distance = diff.length();
            
            // Skip self
            if distance < 0.001 {
                continue;
            }
            
            // Separation: steer away from neighbors that are too close
            if distance < Self::SEPARATION_DISTANCE {
                let strength = 1.0 - (distance / Self::SEPARATION_DISTANCE);
                separation += diff.normalize() * strength;
            }
            
            // Alignment: steer towards the average heading of neighbors
            if distance < Self::ALIGNMENT_DISTANCE {
                alignment += neighbor.velocity;
                alignment_count += 1;
            }
            
            // Cohesion: steer towards the average position of neighbors
            if distance < Self::COHESION_DISTANCE {
                cohesion += neighbor_center;
                cohesion_count += 1;
            }
        }
        
        // Normalize alignment
        if alignment_count > 0 {
            alignment = (alignment / alignment_count as f32).normalize_or_zero();
        }
        
        // Calculate cohesion (steer towards center of mass)
        // Use distance-based strength so arrows far away are more strongly attracted
        if cohesion_count > 0 {
            let center_of_mass = cohesion / cohesion_count as f32;
            let to_center = center_of_mass - arrow_center;
            let distance_to_center = to_center.length();
            
            // Stronger attraction when further away to help regrouping
            if distance_to_center > 0.001 {
                let distance_factor = (distance_to_center / Self::COHESION_DISTANCE).min(1.0);
                // Increase strength for distant neighbors
                let strength = 0.5 + (distance_factor * 0.5); // Range from 0.5 to 1.0
                cohesion = to_center.normalize_or_zero() * strength;
            } else {
                cohesion = Vec2::ZERO;
            }
        }
        
        (separation, alignment, cohesion)
    }
}

