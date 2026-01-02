use flocking_shared::messages::{Vec2, Color, FlockingParameters};

pub struct NeighborData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
}

pub struct Obstacle {
    pub position: Vec2,
    pub radius: f32,
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

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.position.x, self.position.y)
    }
    
    pub fn velocity(&self) -> Vec2 {
        Vec2::new(self.velocity.x, self.velocity.y)
    }
    
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn color(&self) -> Color {
        Color::new(self.color.r, self.color.g, self.color.b, self.color.a)
    }

    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, size: f32, color: Color) -> Self {
        let angle = if velocity.length() > 0.001 {
            velocity.y.atan2(velocity.x)
        } else {
            0.0
        };
        Arrow { position, velocity, acceleration, size, color, angle }
    }

    pub fn update(&mut self, screen_width: f32, screen_height: f32, obstacles: &[Obstacle], neighbors: &[NeighborData], params: &FlockingParameters) {
        let arrow_center = Vec2::new(
            self.position.x + self.size / 2.0,
            self.position.y + self.size / 2.0
        );
        
        // Obstacle avoidance
        for obstacle in obstacles {
            let to_obstacle = Vec2::new(
                arrow_center.x - obstacle.position.x,
                arrow_center.y - obstacle.position.y
            );
            let distance_to_obstacle = to_obstacle.length();
            let avoidance_distance = params.obstacle_avoidance_distance + obstacle.radius;
            
            if distance_to_obstacle < avoidance_distance && distance_to_obstacle > 0.001 {
                let avoidance_force = (1.0 - (distance_to_obstacle - obstacle.radius) / params.obstacle_avoidance_distance) 
                    * params.obstacle_avoidance_strength;
                let avoidance_direction = to_obstacle.normalize();
                self.velocity.x += avoidance_direction.x * avoidance_force;
                self.velocity.y += avoidance_direction.y * avoidance_force;
            }
        }
        
        // Flocking
        let (separation, alignment, cohesion) = self.calculate_flocking(neighbors, params);
        self.velocity.x += separation.x * params.separation_strength;
        self.velocity.y += separation.y * params.separation_strength;
        self.velocity.x += alignment.x * params.alignment_strength;
        self.velocity.y += alignment.y * params.alignment_strength;
        self.velocity.x += cohesion.x * params.cohesion_strength;
        self.velocity.y += cohesion.y * params.cohesion_strength;
        
        // Apply acceleration
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        
        // Clamp speed
        let speed = self.velocity.length();
        if speed > params.max_speed {
            let normalized = self.velocity.normalize();
            self.velocity.x = normalized.x * params.max_speed;
            self.velocity.y = normalized.y * params.max_speed;
        } else if speed < 0.0 {
            self.velocity.x = 0.0;
            self.velocity.y = 0.0;
        }
        
        // Update angle
        if self.velocity.length() > 0.001 {
            self.angle = self.velocity.y.atan2(self.velocity.x);
        }
        
        // Update position
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        
        // Wrap around edges
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

    fn calculate_flocking(&self, neighbors: &[NeighborData], params: &FlockingParameters) -> (Vec2, Vec2, Vec2) {
        let arrow_center = Vec2::new(
            self.position.x + self.size / 2.0,
            self.position.y + self.size / 2.0
        );
        
        let mut separation = Vec2::new(0.0, 0.0);
        let mut alignment = Vec2::new(0.0, 0.0);
        let mut cohesion = Vec2::new(0.0, 0.0);
        let mut alignment_count = 0;
        let mut cohesion_count = 0;
        
        for neighbor in neighbors {
            let neighbor_center = Vec2::new(
                neighbor.position.x + neighbor.size / 2.0,
                neighbor.position.y + neighbor.size / 2.0
            );
            let diff = Vec2::new(
                arrow_center.x - neighbor_center.x,
                arrow_center.y - neighbor_center.y
            );
            let distance = diff.length();
            
            if distance < 0.001 {
                continue;
            }
            
            if distance < params.separation_distance {
                let strength = 1.0 - (distance / params.separation_distance);
                let normalized = diff.normalize();
                separation.x += normalized.x * strength;
                separation.y += normalized.y * strength;
            }
            
            if distance < params.alignment_distance {
                alignment.x += neighbor.velocity.x;
                alignment.y += neighbor.velocity.y;
                alignment_count += 1;
            }
            
            if distance < params.cohesion_distance {
                cohesion.x += neighbor_center.x;
                cohesion.y += neighbor_center.y;
                cohesion_count += 1;
            }
        }
        
        if alignment_count > 0 {
            alignment.x /= alignment_count as f32;
            alignment.y /= alignment_count as f32;
            let normalized = alignment.normalize();
            alignment = normalized;
        }
        
        if cohesion_count > 0 {
            let center_of_mass = Vec2::new(
                cohesion.x / cohesion_count as f32,
                cohesion.y / cohesion_count as f32
            );
            let to_center = Vec2::new(
                center_of_mass.x - arrow_center.x,
                center_of_mass.y - arrow_center.y
            );
            let distance_to_center = to_center.length();
            
            if distance_to_center > 0.001 {
                let distance_factor = (distance_to_center / params.cohesion_distance).min(1.0);
                let strength = 0.5 + (distance_factor * 0.5);
                let normalized = to_center.normalize();
                cohesion = Vec2::new(normalized.x * strength, normalized.y * strength);
            } else {
                cohesion = Vec2::new(0.0, 0.0);
            }
        }
        
        (separation, alignment, cohesion)
    }
}

