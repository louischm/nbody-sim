<template>
  <div id="app">
    <header>
      <h1>N-Body Gravitational Simulation</h1>
      <p>Real-time visualization of planetary motion</p>
    </header>

    <main>
      <ControlPanel
        ref="controlPanel"
        @start="startSimulation"
        @stop="stopSimulation"
        @step="stepSimulation"
      />

      <SimulationCanvas
        :bodies="bodies"
        :time="time"
      />

      <div v-if="error" class="error">
        {{ error }}
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import SimulationCanvas from './components/SimulationCanvas.vue'
import ControlPanel from './components/ControlPanel.vue'
import { api, type Body } from './services/api'

const bodies = ref<Body[]>([])
const time = ref(0)
const error = ref<string | null>(null)
const controlPanel = ref<InstanceType<typeof ControlPanel> | null>(null)

let animationInterval: number | null = null

async function loadInitialState() {
  try {
    const state = await api.getState()
    bodies.value = state.bodies
    time.value = state.time
    error.value = null
  } catch (e) {
    error.value = 'Failed to connect to simulation server. Make sure the Rust backend is running on port 3000.'
    console.error('Error loading state:', e)
  }
}

async function stepSimulation() {
  try {
    const state = await api.step()
    bodies.value = state.bodies
    time.value = state.time
    error.value = null
  } catch (e) {
    error.value = 'Failed to step simulation'
    console.error('Error stepping simulation:', e)
  }
}

function startSimulation() {
  if (animationInterval) return

  const interval = controlPanel.value?.updateInterval || 50
  animationInterval = window.setInterval(async () => {
    await stepSimulation()
  }, interval)
}

function stopSimulation() {
  if (animationInterval) {
    clearInterval(animationInterval)
    animationInterval = null
  }
}

onMounted(() => {
  loadInitialState()
})

onUnmounted(() => {
  stopSimulation()
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: #0a0a0a;
  color: white;
}

#app {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
}

header {
  text-align: center;
  margin-bottom: 30px;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 10px;
}

header h1 {
  font-size: 32px;
  margin-bottom: 10px;
}

header p {
  font-size: 16px;
  opacity: 0.9;
}

main {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.error {
  padding: 15px;
  background: #f44336;
  color: white;
  border-radius: 5px;
  margin: 20px 0;
  max-width: 600px;
  text-align: center;
}
</style>
