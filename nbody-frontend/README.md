# N-Body Simulation - Vue.js Frontend

A modern Vue.js + TypeScript frontend for visualizing the n-body gravitational simulation in real-time.

## Prerequisites

- Node.js (v20.19.0 or >=22.12.0)
- Rust backend server running on port 3000

## Setup

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start the Rust backend server** (from parent directory):
   ```bash
   cd ..
   cargo run -- --mode server --port 3000
   ```

3. **Start the Vue development server:**
   ```bash
   npm run dev
   ```

4. **Open your browser:**
   Navigate to `http://localhost:5173`

## Features

- **Real-time Canvas Rendering**: 2D visualization of planetary bodies
- **Interactive Controls**: Start/stop simulation and step through frames
- **Adjustable Speed**: Control simulation update rate (16ms to 500ms)
- **Live Statistics**: FPS counter, time elapsed, and body count
- **Responsive UI**: Modern dark-themed interface

## Components

- **SimulationCanvas.vue**: Canvas-based renderer for celestial bodies
- **ControlPanel.vue**: Control interface for simulation playback
- **App.vue**: Main application component coordinating everything

## API Integration

The frontend communicates with the Rust backend via REST API:

- `GET /api/state` - Get current simulation state
- `GET /api/step` - Execute one simulation step
- `POST /api/start` - Start continuous simulation
- `POST /api/stop` - Stop simulation

## Development

```bash
# Run dev server with hot-reload
npm run dev

# Type-check
npm run type-check

# Build for production
npm run build

# Preview production build
npm run preview
```

## Configuration

The Vite config includes a proxy to the Rust backend to avoid CORS issues:

```typescript
server: {
  port: 5173,
  proxy: {
    '/api': {
      target: 'http://localhost:3000',
      changeOrigin: true,
    }
  }
}
```

## Customization

- **Canvas size**: Edit `width` and `height` in `SimulationCanvas.vue`
- **Scale factor**: Adjust `scale` variable to zoom in/out
- **Update rate**: Modify default `updateInterval` in `ControlPanel.vue`
- **Colors**: Customize the `colorMap` in `SimulationCanvas.vue`
