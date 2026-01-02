use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.001 {
            Vec2::new(self.x / len, self.y / len)
        } else {
            Vec2::new(0.0, 0.0)
        }
    }

    pub fn normalize_or_zero(&self) -> Self {
        self.normalize()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrowState {
    pub position: Vec2,
    pub angle: f32,
    pub size: f32,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObstacleState {
    pub position: Vec2,
    pub radius: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlockingParameters {
    pub max_speed: f32,
    pub obstacle_avoidance_distance: f32,
    pub obstacle_avoidance_strength: f32,
    pub separation_distance: f32,
    pub alignment_distance: f32,
    pub cohesion_distance: f32,
    pub separation_strength: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
}

impl Default for FlockingParameters {
    fn default() -> Self {
        FlockingParameters {
            max_speed: 5.0,
            obstacle_avoidance_distance: 100.0,
            obstacle_avoidance_strength: 0.5,
            separation_distance: 70.0,
            alignment_distance: 120.0,
            cohesion_distance: 200.0,
            separation_strength: 0.3,
            alignment_strength: 0.25,
            cohesion_strength: 0.3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    CreateObstacle { x: f32, y: f32 },
    UpdateParameters { parameters: FlockingParameters },
    ClearObstacles,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    GameState {
        arrows: Vec<ArrowState>,
        obstacles: Vec<ObstacleState>,
        screen_width: f32,
        screen_height: f32,
        parameters: FlockingParameters,
    },
}

