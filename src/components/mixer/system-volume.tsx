import { Volume2 } from 'lucide-react'

interface SystemVolumeProps {
  disabled: boolean
  volume: number
  onChange: (volume: number) => void
}

export function SystemVolume({ disabled, volume, onChange }: SystemVolumeProps) {
  return (
    <section className="master-strip" aria-label="系统音量">
      <div className="app-icon master-icon" aria-hidden="true">
        <Volume2 size={20} strokeWidth={2.1} />
      </div>
      <div className="strip-content">
        <div className="strip-label">
          <div><span>主音量</span><strong>Mac 扬声器</strong></div>
          <output>{Math.trunc(volume)}%</output>
        </div>
        <input
          aria-label="系统输出音量"
          type="range"
          step="any"
          min="0"
          max="100"
          value={volume}
          disabled={disabled}
          style={{ '--volume': `${volume}%` } as React.CSSProperties}
          onChange={event => onChange(Number(event.target.value))}
        />
      </div>
    </section>
  )
}
