export interface RunningApp {
  id: string
  name: string
  pid: number
  volume: number
  muted: boolean
  controllable: boolean
}

export interface MixerState {
  systemVolume: number
  systemMuted: boolean
  apps: RunningApp[]
}
