'use client'

import { RefreshCw, Search } from 'lucide-react'
import { useDeferredValue, useState } from 'react'

import { ApplicationRow } from './application-row'
import { SystemVolume } from './system-volume'
import { useMixer } from './use-mixer'
import { invoke } from '@tauri-apps/api/core'

export function MixerPanel() {
  const { mixer, error, loading, refresh, setSystemVolume } = useMixer()
  const [query, setQuery] = useState('')
  const deferredQuery = useDeferredValue(query.trim().toLocaleLowerCase())
  const apps = mixer?.apps.filter(app => app.name.toLocaleLowerCase().includes(deferredQuery)) ?? []

  async function setAppVolumeWithPid(pid: number, volume: number) {
    await invoke('set_app_volume_with_pid', { pid, volume })
  }

  return (
    <main className="overflow-hidden size-full flex flex-col">
      <div className="px-4 pt-4 pb-3">
        <SystemVolume
          disabled={!mixer}
          volume={mixer?.systemVolume ?? 0}
          onChange={volume => void setSystemVolume(volume)}
        />
      </div>

      <div className="mx-4 h-px dark:bg-white/6 bg-black/6"></div>

      <div className="px-4 pt-3 pb-2 flex items-center gap-2">
        <span className="text-[12px] font-semibold text-foreground leading-none">Applications</span>
        <span className="text-[11px] text-muted-foreground tabular-nums leading-none">{apps.length}</span>
      </div>

      <div className="section-heading hidden">
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

      <div className="flex-1 overflow-y-auto box-border px-2 pb-3" aria-label="application volume">
        {error && <p className="status error">{error}</p>}
        {!error && loading && !mixer && <p className="status">Loading applications...</p>}
        {!error && !loading && apps.length === 0 && <p className="status">No open applications found.</p>}
        {apps.map(
          app => <ApplicationRow app={app} key={app.id} onChange={(volume) => setAppVolumeWithPid(app.pid, volume)} />,
        )}
      </div>

      <footer className="px-4 py-2.5 flex items-center justify-center border-t border-t-white/6 dark:border-t-black/6">
        <span className="text-[10.5px] text-muted-foreground/50">Single App Control requires macOS 14.2 or later.</span>
      </footer>
    </main>
  )
}
