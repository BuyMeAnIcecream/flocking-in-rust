use crate::arrow::{Arrow, NeighborData, Obstacle};
use flocking_shared::messages::{Vec2, Color, ArrowState, ObstacleState, FlockingParameters};

pub struct Game {
    pub arrows: Vec<Arrow>,
    pub obstacles: Vec<Obstacle>,
    pub screen_width: f32,
    pub screen_height: f32,
    pub parameters: FlockingParameters,
}

impl Game {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut arrows = Vec::new();
        
        // Create initial arrows
        arrows.push(Arrow::new(
            Vec2::new(0.0, 50.0),
            Vec2::new(20.0, 0.0),
            Vec2::new(0.1, 0.0),
            40.0,
            Color::new(1.0, 0.0, 0.0, 1.0), // RED
        ));
        arrows.push(Arrow::new(
            Vec2::new(0.0, 120.0),
            Vec2::new(30.0, 5.0),
            Vec2::new(0.0, 0.05),
            50.0,
            Color::new(0.0, 0.0, 1.0, 1.0), // BLUE
        ));
        arrows.push(Arrow::new(
            Vec2::new(0.0, 200.0),
            Vec2::new(15.0, -3.0),
            Vec2::new(0.15, 0.0),
            35.0,
            Color::new(0.0, 1.0, 0.0, 1.0), // GREEN
        ));
        arrows.push(Arrow::new(
            Vec2::new(0.0, 280.0),
            Vec2::new(25.0, 2.0),
            Vec2::new(-0.05, 0.1),
            45.0,
            Color::new(1.0, 1.0, 0.0, 1.0), // YELLOW
        ));
        arrows.push(Arrow::new(
            Vec2::new(0.0, 360.0),
            Vec2::new(18.0, -5.0),
            Vec2::new(0.2, -0.1),
            55.0,
            Color::new(1.0, 0.0, 1.0, 1.0), // MAGENTA
        ));
        
        Game {
            arrows,
            obstacles: Vec::new(),
            screen_width,
            screen_height,
            parameters: FlockingParameters::default(),
        }
    }

    pub fn add_obstacle(&mut self, x: f32, y: f32) {
        self.obstacles.push(Obstacle {
            position: Vec2::new(x, y),
            radius: 40.0,
        });
    }

    pub fn clear_obstacles(&mut self) {
        self.obstacles.clear();
    }

    pub fn update_parameters(&mut self, parameters: FlockingParameters) {
        self.parameters = parameters;
    }

    pub fn update(&mut self) {
        // Collect neighbor data
        let neighbor_data: Vec<NeighborData> = self.arrows.iter().map(|a| NeighborData {
            position: a.position(),
            velocity: a.velocity(),
            size: a.size(),
        }).collect();
        
        // Update all arrows
        for arrow in &mut self.arrows {
            arrow.update(
                self.screen_width,
                self.screen_height,
                &self.obstacles,
                &neighbor_data,
                &self.parameters,
            );
        }
    }

    pub fn get_state(&self) -> (Vec<ArrowState>, Vec<ObstacleState>, FlockingParameters) {
        let arrows: Vec<ArrowState> = self.arrows.iter().map(|a| ArrowState {
            position: a.position(),
            angle: a.angle(),
            size: a.size(),
            color: a.color(),
        }).collect();
        
        let obstacles: Vec<ObstacleState> = self.obstacles.iter().map(|o| ObstacleState {
            position: Vec2::new(o.position.x, o.position.y),
            radius: o.radius,
        }).collect();
        
        (arrows, obstacles, self.parameters.clone())
    }
}

