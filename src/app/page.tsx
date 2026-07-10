'use client'

import { invoke } from '@tauri-apps/api/core'
import { RefreshCw, Search, Volume2, VolumeX } from 'lucide-react'
import { useEffect, useEffectEvent, useState } from 'react'

interface RunningApp {
  id: string
  name: string
  pid: number
  volume: number
  muted: boolean
  controllable: boolean
}

interface MixerState {
  systemVolume: number
  systemMuted: boolean
  apps: RunningApp[]
}

export default function Home() {
  const [mixer, setMixer] = useState<MixerState | null>(null)
  const [query, setQuery] = useState('')
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

  async function changeSystemVolume(volume: number) {
    if (!mixer) return

    setMixer({ ...mixer, systemVolume: volume })
    try {
      await invoke('set_system_volume', { volume })
      setError(null)
    }
    catch (reason) {
      setError(String(reason))
      void refresh()
    }
  }

  const apps = mixer?.apps.filter(app => app.name.toLowerCase().includes(query.toLowerCase())) ?? []

  return (
    <main className="mixer-shell">
      <header className="titlebar" data-tauri-drag-region>
        <div>
          <p className="eyebrow">PER-APP VOLUME</p>
          <h1>声音混音器</h1>
        </div>
        <button className="icon-button" type="button" title="刷新已打开的应用" onClick={() => void refresh()} disabled={loading}>
          <RefreshCw size={16} className={loading ? 'spin' : ''} />
        </button>
      </header>

      <section className="master-strip" aria-label="系统音量">
        <div className="app-icon master-icon"><Volume2 size={21} /></div>
        <div className="strip-content">
          <div className="strip-label">
            <div><strong>系统输出</strong><span>默认输出设备</span></div>
            <output>{mixer?.systemVolume ?? 0}%</output>
          </div>
          <input
            aria-label="系统输出音量"
            type="range"
            min="0"
            max="100"
            value={mixer?.systemVolume ?? 0}
            disabled={!mixer}
            style={{ '--volume': `${mixer?.systemVolume ?? 0}%` } as React.CSSProperties}
            onChange={event => void changeSystemVolume(Number(event.target.value))}
          />
        </div>
      </section>

      <div className="section-heading">
        <div><h2>已打开的应用</h2><span>{apps.length} 个应用</span></div>
        <label className="search-field">
          <Search size={14} />
          <input value={query} onChange={event => setQuery(event.target.value)} placeholder="搜索" />
        </label>
      </div>

      <section className="app-list" aria-label="应用音量">
        {error && <p className="status error">{error}</p>}
        {!error && loading && !mixer && <p className="status">正在读取音频状态...</p>}
        {!loading && apps.length === 0 && <p className="status">没有找到已打开的用户应用</p>}
        {apps.map(app => (
          <article className="app-strip" key={app.id}>
            <div className="app-icon">
              {app.name.slice(0, 1).toUpperCase()}
            </div>
            <div className="strip-content">
              <div className="strip-label">
                <div><strong>{app.name}</strong><span>PID {app.pid}</span></div>
                <output>{app.controllable ? `${app.volume}%` : '待接入'}</output>
              </div>
              <div className="app-control-row">
                <button className="mute-button" type="button" title="应用静音" disabled={!app.controllable}>
                  {app.muted ? <VolumeX size={15} /> : <Volume2 size={15} />}
                </button>
                <input
                  aria-label={`${app.name} 音量`}
                  type="range"
                  min="0"
                  max="100"
                  value={app.volume}
                  disabled={!app.controllable}
                  style={{ '--volume': `${app.volume}%` } as React.CSSProperties}
                  readOnly
                />
              </div>
            </div>
          </article>
        ))}
      </section>

      <footer>单应用音量需要 macOS 14.2+ 的 Core Audio Process Tap</footer>
    </main>
  )
}
