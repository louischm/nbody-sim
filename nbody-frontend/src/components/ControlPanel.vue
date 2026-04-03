<template>
  <div class="control-panel">
    <h2>N-Body Simulation Control</h2>

    <div class="controls">
      <button
        @click="toggleSimulation"
        :class="{ active: isRunning }"
        class="btn-primary"
      >
        {{ isRunning ? 'Pause' : 'Start' }}
      </button>

      <button
        @click="stepOnce"
        :disabled="isRunning"
        class="btn-secondary"
      >
        Step Once
      </button>
    </div>

    <div class="stats">
      <div class="stat-item">
        <span class="label">Status:</span>
        <span :class="isRunning ? 'status-running' : 'status-paused'">
          {{ isRunning ? 'Running' : 'Paused' }}
        </span>
      </div>
      <div class="stat-item">
        <span class="label">Update Rate:</span>
        <span>{{ updateInterval }}ms</span>
      </div>
    </div>

    <div class="slider-control">
      <label for="speed">Simulation Speed:</label>
      <input
        id="speed"
        type="range"
        v-model="updateInterval"
        min="16"
        max="500"
        step="10"
        :disabled="isRunning"
      >
      <span>{{ updateInterval }}ms ({{ fps }} FPS)</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Emits {
  (e: 'start'): void
  (e: 'stop'): void
  (e: 'step'): void
}

const emit = defineEmits<Emits>()

const isRunning = ref(false)
const updateInterval = ref(50) // ms between updates

const fps = computed(() => Math.round(1000 / updateInterval.value))

function toggleSimulation() {
  if (isRunning.value) {
    isRunning.value = false
    emit('stop')
  } else {
    isRunning.value = true
    emit('start')
  }
}

function stepOnce() {
  emit('step')
}

defineExpose({
  isRunning,
  updateInterval
})
</script>

<style scoped>
.control-panel {
  padding: 20px;
  background: #1a1a1a;
  border-radius: 8px;
  color: white;
  font-family: 'Arial', sans-serif;
  margin-bottom: 20px;
}

.control-panel h2 {
  margin-top: 0;
  margin-bottom: 20px;
  font-size: 20px;
  color: #4CAF50;
}

.controls {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 5px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s;
}

.btn-primary {
  background: #4CAF50;
  color: white;
}

.btn-primary:hover {
  background: #45a049;
}

.btn-primary.active {
  background: #f44336;
}

.btn-primary.active:hover {
  background: #da190b;
}

.btn-secondary {
  background: #2196F3;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background: #0b7dda;
}

.btn-secondary:disabled {
  background: #555;
  cursor: not-allowed;
  opacity: 0.5;
}

.stats {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 10px;
  background: #2a2a2a;
  border-radius: 5px;
}

.stat-item {
  display: flex;
  gap: 10px;
}

.label {
  font-weight: bold;
  color: #aaa;
}

.status-running {
  color: #4CAF50;
  font-weight: bold;
}

.status-paused {
  color: #ff9800;
  font-weight: bold;
}

.slider-control {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.slider-control label {
  font-weight: bold;
  color: #aaa;
}

input[type="range"] {
  width: 100%;
  height: 6px;
  background: #555;
  outline: none;
  border-radius: 3px;
}

input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 18px;
  height: 18px;
  background: #4CAF50;
  cursor: pointer;
  border-radius: 50%;
}

input[type="range"]:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.slider-control span {
  color: #4CAF50;
  font-weight: bold;
}
</style>
