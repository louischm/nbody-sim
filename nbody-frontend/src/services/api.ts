import axios from 'axios'

const API_BASE_URL = '/api'

export interface Body {
  name: string
  mass: number
  position: [number, number, number]
  velocity: [number, number, number]
  color: string
}

export interface SimulationState {
  bodies: Body[]
  time: number
  running: boolean
}

export interface ApiResponse<T> {
  success: boolean
  data: T | null
  message: string | null
}

class SimulationAPI {
  async getState(): Promise<SimulationState> {
    const response = await axios.get<ApiResponse<SimulationState>>(`${API_BASE_URL}/state`)
    return response.data.data!
  }

  async step(): Promise<SimulationState> {
    const response = await axios.get<ApiResponse<SimulationState>>(`${API_BASE_URL}/step`)
    return response.data.data!
  }

  async start(): Promise<void> {
    await axios.post(`${API_BASE_URL}/start`)
  }

  async stop(): Promise<void> {
    await axios.post(`${API_BASE_URL}/stop`)
  }
}

export const api = new SimulationAPI()
