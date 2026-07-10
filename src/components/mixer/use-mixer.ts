'use client'

import { invoke } from '@tauri-apps/api/core'
import { useEffect, useEffectEvent, useState } from 'react'

import type { MixerState } from './types'

export function useMixer() {
  const [mixer, setMixer] = useState<MixerState | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)

  const refresh = useEffectEvent(async () => {
    setLoading(true)
    try {
      setMixer(await invoke<MixerState>('get_mixer_state'))
      setError(null)
    }
    catch (reason) {
      setError(String(reason))
    }
    finally {
      setLoading(false)
    }
  })

  useEffect(() => {
    void refresh()
  }, [])

  async function setSystemVolume(volume: number) {
    if (!mixer) return

    const previousVolume = mixer.systemVolume
    setMixer(current => current ? { ...current, systemVolume: volume } : current)

    try {
      await invoke('set_system_volume', { volume })
      setError(null)
    }
    catch (reason) {
      setMixer(current => current ? { ...current, systemVolume: previousVolume } : current)
      setError(String(reason))
    }
  }

  return { mixer, error, loading, refresh, setSystemVolume }
}
