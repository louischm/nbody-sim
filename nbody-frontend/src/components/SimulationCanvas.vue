<template>
  <div class="canvas-container">
    <canvas
      ref="canvas"
      :width="width"
      :height="height"
      class="simulation-canvas"
      @wheel="handleWheel"
    ></canvas>
    <div class="info-overlay">
      <div>Time: {{ formattedTime }}</div>
      <div>Bodies: {{ bodies.length }}</div>
      <div>FPS: {{ fps }}</div>
      <div>Zoom: {{ zoomLevel.toFixed(2) }}x</div>
    </div>
    <div class="zoom-controls">
      <button @click="zoomIn" class="zoom-btn">+</button>
      <button @click="resetZoom" class="zoom-btn">Reset</button>
      <button @click="zoomOut" class="zoom-btn">-</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import type { Body } from '../services/api'

interface Props {
  bodies: Body[]
  time: number
}

const props = defineProps<Props>()

const canvas = ref<HTMLCanvasElement | null>(null)
const width = ref(1200)
const height = ref(800)
const fps = ref(0)
const zoomLevel = ref(1.0)

let lastFrameTime = 0
let frameCount = 0
let fpsUpdateTime = 0

const MIN_ZOOM = 0.1
const MAX_ZOOM = 10.0
const ZOOM_STEP = 0.1

const formattedTime = computed(() => {
  const days = props.time / (24 * 3600)
  return `${days.toFixed(2)} days`
})

const colorMap: Record<string, string> = {
  yellow: '#FFFF00',
  gray: '#808080',
  beige: '#F5F5DC',
  blue: '#0000FF',
  red: '#FF0000',
  orange: '#FFA500',
  maroon: '#800000',
  skyblue: '#87CEEB',
  darkblue: '#00008B',
  white: '#FFFFFF',
}

watch(() => props.bodies, () => {
  drawBodies()
  updateFPS()
}, { deep: true })

function drawBodies() {
  if (!canvas.value) return

  const ctx = canvas.value.getContext('2d')
  if (!ctx) return

  // Clear canvas
  ctx.fillStyle = '#000000'
  ctx.fillRect(0, 0, width.value, height.value)

  const scale = 1e-9 * zoomLevel.value
  const centerX = width.value * 0.5
  const centerY = height.value * 0.5

  props.bodies.forEach((body) => {
    const x = centerX + body.position[0] * scale
    const y = centerY + body.position[1] * scale

    // Calculate radius based on mass (also scale with zoom)
    const baseRadius = Math.max(2, Math.min(8, Math.log10(Math.abs(body.mass))))
    const radius = baseRadius * Math.max(0.5, Math.min(2, zoomLevel.value))

    // Draw body
    ctx.fillStyle = colorMap[body.color] || '#FFFFFF'
    ctx.beginPath()
    ctx.arc(x, y, radius, 0, Math.PI * 2)
    ctx.fill()

    // Draw name (only if zoomed in enough)
    if (zoomLevel.value >= 0.5) {
      ctx.fillStyle = '#FFFFFF'
      ctx.font = '12px Arial'
      ctx.fillText(body.name, x + 8, y - 8)
    }
  })
}

function updateFPS() {
  const now = performance.now()
  frameCount++

  if (now - fpsUpdateTime >= 1000) {
    fps.value = Math.round((frameCount * 1000) / (now - fpsUpdateTime))
    frameCount = 0
    fpsUpdateTime = now
  }
}

function handleWheel(event: WheelEvent) {
  event.preventDefault()

  const delta = -event.deltaY / 1000
  const newZoom = zoomLevel.value + delta

  zoomLevel.value = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, newZoom))
  drawBodies()
}

function zoomIn() {
  const newZoom = zoomLevel.value + ZOOM_STEP
  zoomLevel.value = Math.min(MAX_ZOOM, newZoom)
  drawBodies()
}

function zoomOut() {
  const newZoom = zoomLevel.value - ZOOM_STEP
  zoomLevel.value = Math.max(MIN_ZOOM, newZoom)
  drawBodies()
}

function resetZoom() {
  zoomLevel.value = 1.0
  drawBodies()
}

onMounted(() => {
  drawBodies()
  fpsUpdateTime = performance.now()
})
</script>

<script lang="ts">
import { computed } from 'vue'
</script>

<style scoped>
.canvas-container {
  position: relative;
  display: inline-block;
}

.simulation-canvas {
  border: 2px solid #333;
  background: #000;
  display: block;
}

.info-overlay {
  position: absolute;
  top: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 10px;
  border-radius: 5px;
  font-family: monospace;
  font-size: 14px;
}

.info-overlay > div {
  margin: 5px 0;
}

.zoom-controls {
  position: absolute;
  bottom: 10px;
  left: 10px;
  display: flex;
  gap: 5px;
  background: rgba(0, 0, 0, 0.7);
  padding: 5px;
  border-radius: 5px;
}

.zoom-btn {
  background: #2196F3;
  color: white;
  border: none;
  border-radius: 3px;
  padding: 8px 12px;
  font-size: 14px;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.2s;
}

.zoom-btn:hover {
  background: #0b7dda;
}

.zoom-btn:active {
  transform: scale(0.95);
}

.simulation-canvas {
  cursor: grab;
}

.simulation-canvas:active {
  cursor: grabbing;
}
</style>
