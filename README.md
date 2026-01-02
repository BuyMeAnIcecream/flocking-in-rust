# Flocking in Rust

A client-server implementation of flocking behavior using Rust and WebSockets. The server handles all physics calculations, while the browser client renders the simulation and provides real-time parameter controls.

## Architecture

- **Server** (Rust): Runs game loop at 60 FPS, calculates arrow physics, flocking, and obstacle avoidance
- **Client** (Browser): Renders arrows and obstacles, sends mouse click events to create obstacles, provides parameter controls
- **Communication**: WebSocket for real-time bidirectional communication

## Project Structure

```
flocking-in-rust/
├── server/          # Rust WebSocket server
│   ├── src/
│   │   ├── main.rs  # WebSocket server + game loop
│   │   ├── game.rs  # Game state management
│   │   └── arrow.rs # Arrow physics and flocking logic
│   └── Cargo.toml
├── client/          # Web client
│   ├── index.html   # UI with parameter controls
│   └── client.js    # WebSocket client + rendering
└── shared/          # Shared message types
    └── src/
        └── messages.rs
```

## Running

1. Start the server:
```bash
cd server
cargo run
```

The server will start on `http://localhost:3000`

2. Open your browser and navigate to:
```
http://localhost:3000
```

## Features

### Flocking Behavior
Arrows exhibit three classic flocking behaviors:
- **Separation**: Maintains distance from nearby neighbors
- **Alignment**: Matches direction with nearby neighbors
- **Cohesion**: Moves toward the center of nearby neighbors

### Interactive Controls
- **Click to Create Obstacles**: Click anywhere on the canvas to create white sphere obstacles
- **Clear Obstacles Button**: Remove all obstacles with one click
- **Real-time Parameter Adjustment**: Control panel with sliders for all flocking parameters:
  - Max Speed
  - Obstacle Avoidance (Distance & Strength)
  - Separation (Distance & Strength)
  - Alignment (Distance & Strength)
  - Cohesion (Distance & Strength)

### Physics System
- Velocity and acceleration-based movement
- Speed capped between 0 and 100
- Arrows automatically point in their direction of movement
- Edge wrapping: Arrows wrap around screen edges seamlessly

### Real-time Updates
- Server broadcasts game state at ~30 FPS to all connected clients
- Parameter changes are applied immediately
- Multiple clients can connect and see the same simulation

## Customization

All parameters can be adjusted in real-time using the control panel in the browser:

- **Max Speed** (0-20): Maximum arrow velocity
- **Obstacle Avoidance Distance** (0-200): How far arrows start avoiding obstacles
- **Obstacle Avoidance Strength** (0-2): Force of obstacle avoidance
- **Separation Distance** (0-200): Distance arrows try to maintain from neighbors
- **Separation Strength** (0-2): Force of separation behavior
- **Alignment Distance** (0-300): Range for alignment behavior
- **Alignment Strength** (0-2): Force of alignment behavior
- **Cohesion Distance** (0-400): Range for cohesion behavior
- **Cohesion Strength** (0-2): Force of cohesion behavior

Default values are optimized for smooth flocking behavior, but you can experiment with different combinations to see various effects!

## Dependencies

### Server
- [axum](https://github.com/tokio-rs/axum) - Web framework with WebSocket support
- [tokio](https://tokio.rs/) - Async runtime
- [serde](https://serde.rs/) - Serialization framework
- [tower-http](https://github.com/tower-rs/tower-http) - HTTP utilities including static file serving

## Development

The project is organized into three main components:

1. **Server** (`server/`): Rust backend that runs the simulation
2. **Client** (`client/`): HTML/JavaScript frontend for visualization and control
3. **Shared** (`shared/`): Common message types used for client-server communication

## License

This project is open source and available for learning and experimentation.
