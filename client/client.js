const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const status = document.getElementById('status');

let ws = null;
let gameState = {
    arrows: [],
    obstacles: [],
    screenWidth: 800,
    screenHeight: 600,
    parameters: {
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
};

// Set canvas size
function resizeCanvas() {
    canvas.width = gameState.screenWidth;
    canvas.height = gameState.screenHeight;
}

// Connect to WebSocket
function connect() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws`;
    
    ws = new WebSocket(wsUrl);
    
    ws.onopen = () => {
        status.textContent = 'Connected';
        status.style.color = '#0f0';
    };
    
    ws.onmessage = (event) => {
        try {
            const message = JSON.parse(event.data);
            if (message.type === 'GameState') {
                gameState.arrows = message.arrows;
                gameState.obstacles = message.obstacles;
                gameState.screenWidth = message.screen_width;
                gameState.screenHeight = message.screen_height;
                if (message.parameters) {
                    gameState.parameters = message.parameters;
                    updateControlsFromParameters();
                }
                resizeCanvas();
            }
        } catch (e) {
            console.error('Error parsing message:', e);
        }
    };
    
    ws.onerror = (error) => {
        status.textContent = 'Connection error';
        status.style.color = '#f00';
        console.error('WebSocket error:', error);
    };
    
    ws.onclose = () => {
        status.textContent = 'Disconnected. Reconnecting...';
        status.style.color = '#ff0';
        setTimeout(connect, 1000);
    };
}

// Handle mouse clicks
canvas.addEventListener('click', (e) => {
    if (!ws || ws.readyState !== WebSocket.OPEN) return;
    
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const message = {
        type: 'CreateObstacle',
        x: x,
        y: y,
    };
    
    ws.send(JSON.stringify(message));
});

// Draw arrow
function drawArrow(arrow) {
    const centerX = arrow.position.x + arrow.size / 2;
    const centerY = arrow.position.y + arrow.size / 2;
    const halfSize = arrow.size / 2;
    const tipLength = arrow.size * 0.5;
    
    ctx.save();
    ctx.translate(centerX, centerY);
    ctx.rotate(arrow.angle);
    
    ctx.fillStyle = `rgba(${Math.floor(arrow.color.r * 255)}, ${Math.floor(arrow.color.g * 255)}, ${Math.floor(arrow.color.b * 255)}, ${arrow.color.a})`;
    ctx.beginPath();
    ctx.moveTo(halfSize + tipLength, 0);
    ctx.lineTo(halfSize, -halfSize);
    ctx.lineTo(halfSize, halfSize);
    ctx.closePath();
    ctx.fill();
    
    ctx.restore();
}

// Draw obstacle
function drawObstacle(obstacle) {
    ctx.fillStyle = 'white';
    ctx.beginPath();
    ctx.arc(obstacle.position.x, obstacle.position.y, obstacle.radius, 0, Math.PI * 2);
    ctx.fill();
    
    ctx.strokeStyle = 'gray';
    ctx.lineWidth = 2;
    ctx.stroke();
}

// Render loop
function render() {
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Draw obstacles
    gameState.obstacles.forEach(drawObstacle);
    
    // Draw arrows
    gameState.arrows.forEach(drawArrow);
    
    requestAnimationFrame(render);
}

// Update parameters and send to server
function updateParameters() {
    if (!ws || ws.readyState !== WebSocket.OPEN) return;
    
    const parameters = {
        max_speed: parseFloat(document.getElementById('max_speed').value),
        obstacle_avoidance_distance: parseFloat(document.getElementById('obstacle_avoidance_distance').value),
        obstacle_avoidance_strength: parseFloat(document.getElementById('obstacle_avoidance_strength').value),
        separation_distance: parseFloat(document.getElementById('separation_distance').value),
        alignment_distance: parseFloat(document.getElementById('alignment_distance').value),
        cohesion_distance: parseFloat(document.getElementById('cohesion_distance').value),
        separation_strength: parseFloat(document.getElementById('separation_strength').value),
        alignment_strength: parseFloat(document.getElementById('alignment_strength').value),
        cohesion_strength: parseFloat(document.getElementById('cohesion_strength').value),
    };
    
    const message = {
        type: 'UpdateParameters',
        parameters: parameters,
    };
    
    ws.send(JSON.stringify(message));
}

// Update controls from parameters (when receiving from server)
function updateControlsFromParameters() {
    const params = gameState.parameters;
    document.getElementById('max_speed').value = params.max_speed;
    document.getElementById('max_speed_val').value = params.max_speed;
    document.getElementById('obstacle_avoidance_distance').value = params.obstacle_avoidance_distance;
    document.getElementById('obstacle_avoidance_distance_val').value = params.obstacle_avoidance_distance;
    document.getElementById('obstacle_avoidance_strength').value = params.obstacle_avoidance_strength;
    document.getElementById('obstacle_avoidance_strength_val').value = params.obstacle_avoidance_strength;
    document.getElementById('separation_distance').value = params.separation_distance;
    document.getElementById('separation_distance_val').value = params.separation_distance;
    document.getElementById('separation_strength').value = params.separation_strength;
    document.getElementById('separation_strength_val').value = params.separation_strength;
    document.getElementById('alignment_distance').value = params.alignment_distance;
    document.getElementById('alignment_distance_val').value = params.alignment_distance;
    document.getElementById('alignment_strength').value = params.alignment_strength;
    document.getElementById('alignment_strength_val').value = params.alignment_strength;
    document.getElementById('cohesion_distance').value = params.cohesion_distance;
    document.getElementById('cohesion_distance_val').value = params.cohesion_distance;
    document.getElementById('cohesion_strength').value = params.cohesion_strength;
    document.getElementById('cohesion_strength_val').value = params.cohesion_strength;
}

// Sync slider and number input
function syncInputs(sliderId, numberId) {
    const slider = document.getElementById(sliderId);
    const number = document.getElementById(numberId);
    
    slider.addEventListener('input', () => {
        number.value = slider.value;
        updateParameters();
    });
    
    number.addEventListener('input', () => {
        slider.value = number.value;
        updateParameters();
    });
}

// Setup all control syncs
syncInputs('max_speed', 'max_speed_val');
syncInputs('obstacle_avoidance_distance', 'obstacle_avoidance_distance_val');
syncInputs('obstacle_avoidance_strength', 'obstacle_avoidance_strength_val');
syncInputs('separation_distance', 'separation_distance_val');
syncInputs('separation_strength', 'separation_strength_val');
syncInputs('alignment_distance', 'alignment_distance_val');
syncInputs('alignment_strength', 'alignment_strength_val');
syncInputs('cohesion_distance', 'cohesion_distance_val');
syncInputs('cohesion_strength', 'cohesion_strength_val');

// Clear obstacles button
document.getElementById('clear_obstacles').addEventListener('click', () => {
    if (!ws || ws.readyState !== WebSocket.OPEN) return;
    
    const message = {
        type: 'ClearObstacles',
    };
    
    ws.send(JSON.stringify(message));
});

// Initialize
resizeCanvas();
connect();
render();

