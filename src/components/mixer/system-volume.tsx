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
        <Volume2 size={18} strokeWidth={2.2} />
      </div>
      <div className="strip-content">
        <div className="strip-label">
          <div><strong>系统输出</strong><span>默认输出设备</span></div>
          <output>{volume}%</output>
        </div>
        <input
          aria-label="系统输出音量"
          type="range"
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
