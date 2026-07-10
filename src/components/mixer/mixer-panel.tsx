'use client'

import { RefreshCw, Search } from 'lucide-react'
import { useDeferredValue, useState } from 'react'

import { ApplicationRow } from './application-row'
import { SystemVolume } from './system-volume'
import { useMixer } from './use-mixer'

export function MixerPanel() {
  const { mixer, error, loading, refresh, setSystemVolume } = useMixer()
  const [query, setQuery] = useState('')
  const deferredQuery = useDeferredValue(query.trim().toLocaleLowerCase())
  const apps = mixer?.apps.filter(app => app.name.toLocaleLowerCase().includes(deferredQuery)) ?? []

  return (
    <main className="mixer-shell">
      <header className="titlebar" data-tauri-drag-region>
        <div><h1>音量混合器</h1><p>控制 Mac 与打开的应用程序</p></div>
      </header>

      <div className="content-scroll">
        <SystemVolume
          disabled={!mixer}
          volume={mixer?.systemVolume ?? 0}
          onChange={volume => void setSystemVolume(volume)}
        />

        <div className="section-heading">
          <div><h2>应用程序</h2><span>{apps.length}</span></div>
          <div className="glass-toolbar">
            <label className="search-field">
              <Search size={13} aria-hidden="true" />
              <input value={query} onChange={event => setQuery(event.target.value)} placeholder="搜索" aria-label="搜索应用" />
            </label>
            <span className="toolbar-divider" aria-hidden="true" />
            <button
              className="icon-button"
              type="button"
              title="刷新应用列表"
              aria-label="刷新应用列表"
              onClick={() => void refresh()}
              disabled={loading}
            >
              <RefreshCw size={14} className={loading ? 'spin' : ''} />
            </button>
          </div>
        </div>

        <section className="app-list" aria-label="应用音量">
          {error && <p className="status error">{error}</p>}
          {!error && loading && !mixer && <p className="status">正在载入应用程序...</p>}
          {!error && !loading && apps.length === 0 && <p className="status">没有找到打开的应用程序</p>}
          {apps.map(app => <ApplicationRow app={app} key={app.id} />)}
        </section>
      </div>

      <footer>单应用控制需要 macOS 14.2 或更高版本</footer>
    </main>
  )
}
