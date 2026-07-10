import { Volume2, VolumeX } from 'lucide-react'

import type { RunningApp } from './types'

interface ApplicationRowProps {
  app: RunningApp
}

export function ApplicationRow({ app }: ApplicationRowProps) {
  return (
    <article className="app-strip">
      <div className="app-icon" aria-hidden="true">{app.name.slice(0, 1).toUpperCase()}</div>
      <div className="strip-content">
        <div className="strip-label">
          <div><strong>{app.name}</strong><span>正在运行</span></div>
          <output>{app.controllable ? `${app.volume}%` : '即将支持'}</output>
        </div>
        <div className="app-control-row">
          <button className="mute-button" type="button" title="应用静音" disabled={!app.controllable}>
            {app.muted ? <VolumeX size={14} /> : <Volume2 size={14} />}
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
  )
}
