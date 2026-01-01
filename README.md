# Flocking in Rust

A Rust implementation of flocking behavior using the macroquad game framework. Watch colorful arrows form flocks, avoid your mouse cursor, and move together in mesmerizing patterns!

## Features

- **Flocking Behavior**: Arrows exhibit three classic flocking behaviors:
  - **Separation**: Maintains distance from nearby neighbors
  - **Alignment**: Matches direction with nearby neighbors
  - **Cohesion**: Moves toward the center of nearby neighbors

- **Mouse Avoidance**: Arrows dynamically avoid the mouse cursor when it gets too close

- **Physics System**: 
  - Velocity and acceleration-based movement
  - Speed capped between 0 and 100
  - Arrows automatically point in their direction of movement

- **Edge Wrapping**: Arrows wrap around screen edges seamlessly

## Running

Make sure you have Rust installed, then run:

```bash
cargo run
```

The first run will download and compile dependencies (including macroquad), which may take a minute.

## Project Evolution

This project started as a simple demo of moving squares and evolved into a full flocking simulation:

1. **Started with squares** moving left to right
2. **Converted to arrows** with rotation
3. **Added physics** with velocity and acceleration vectors
4. **Implemented mouse avoidance** for interactive behavior
5. **Added flocking** with separation, alignment, and cohesion behaviors

## Code Structure

- `src/main.rs` - Main game loop and arrow initialization
- `src/arrow.rs` - Arrow struct with flocking logic and rendering

## Customization

You can adjust flocking behavior by modifying constants in `src/arrow.rs`:

- `SEPARATION_DISTANCE` - How far apart arrows try to stay (default: 70.0)
- `ALIGNMENT_DISTANCE` - Range for alignment behavior (default: 120.0)
- `COHESION_DISTANCE` - Range for cohesion behavior (default: 200.0)
- `SEPARATION_STRENGTH` - Strength of separation force (default: 0.3)
- `ALIGNMENT_STRENGTH` - Strength of alignment force (default: 0.25)
- `COHESION_STRENGTH` - Strength of cohesion force (default: 0.3)
- `MAX_SPEED` - Maximum arrow speed (default: 5.0)
- `MOUSE_AVOIDANCE_DISTANCE` - Mouse avoidance range (default: 100.0)
- `MOUSE_AVOIDANCE_STRENGTH` - Mouse avoidance force (default: 0.5)

## Dependencies

- [macroquad](https://github.com/not-fl3/macroquad) - A simple and easy game framework for Rust

## License

This project is open source and available for learning and experimentation.
