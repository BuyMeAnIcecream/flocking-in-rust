use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

mod arrow;
mod game;

use flocking_shared::messages::{ClientMessage, ServerMessage};
use game::Game;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const UPDATE_RATE: u64 = 16; // ~60 FPS

type GameState = Arc<RwLock<Game>>;

#[tokio::main]
async fn main() {
    let game_state = Arc::new(RwLock::new(Game::new(SCREEN_WIDTH, SCREEN_HEIGHT)));
    
    // Spawn game loop
    let game_state_clone = game_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(UPDATE_RATE));
        loop {
            interval.tick().await;
            let mut game = game_state_clone.write().await;
            game.update();
        }
    });
    
    let app = Router::new()
        .route("/ws", get(handle_socket))
        .nest_service("/", tower_http::services::ServeDir::new("../client"))
        .layer(CorsLayer::permissive())
        .with_state(game_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    println!("WebSocket endpoint: ws://localhost:3000/ws");
    
    axum::serve(listener, app).await.unwrap();
}

async fn handle_socket(ws: WebSocketUpgrade, axum::extract::State(state): axum::extract::State<GameState>) -> Response {
    ws.on_upgrade(|socket| handle_connection(socket, state))
}

async fn handle_connection(socket: WebSocket, state: GameState) {
    use futures_util::{SinkExt, StreamExt};
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial game state
    let (arrows, obstacles, parameters) = {
        let game = state.read().await;
        game.get_state()
    };
    
    let initial_message = ServerMessage::GameState {
        arrows,
        obstacles,
        screen_width: SCREEN_WIDTH,
        screen_height: SCREEN_HEIGHT,
        parameters: parameters.clone(),
    };
    
    if let Ok(json) = serde_json::to_string(&initial_message) {
        let _ = sender.send(Message::Text(json)).await;
    }
    
    // Spawn task to send periodic updates
    let state_clone = state.clone();
    let mut send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(33)); // ~30 FPS
        loop {
            interval.tick().await;
            let (arrows, obstacles, parameters) = {
                let game = state_clone.read().await;
                game.get_state()
            };
            
            let message = ServerMessage::GameState {
                arrows,
                obstacles,
                screen_width: SCREEN_WIDTH,
                screen_height: SCREEN_HEIGHT,
                parameters: parameters.clone(),
            };
            
            if let Ok(json) = serde_json::to_string(&message) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
                match msg {
                    ClientMessage::CreateObstacle { x, y } => {
                        let mut game = state.write().await;
                        game.add_obstacle(x, y);
                    }
                    ClientMessage::UpdateParameters { parameters } => {
                        let mut game = state.write().await;
                        game.update_parameters(parameters);
                    }
                    ClientMessage::ClearObstacles => {
                        let mut game = state.write().await;
                        game.clear_obstacles();
                    }
                }
            }
        }
    });
    
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

