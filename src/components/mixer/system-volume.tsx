import { Volume2 } from 'lucide-react'
import { Button } from '../ui'
import { Slider } from '../ui/slider'

interface SystemVolumeProps {
  disabled: boolean
  volume: number
  onChange: (volume: number) => void
}

export function SystemVolume({ disabled, volume, onChange }: SystemVolumeProps) {
  // master-strip
  return (
    <section className="rounded-[13px] px-3.5 py-3 bg-white/60 dark:bg-white/5 border-white/7 dark:border-white/8 border" aria-label="系统音量">
      <div className="flex items-center justify-between mb-2.5">
        <div className="flex items-center gap-2">
          <div className="w-7 h-7 rounded-[7px] flex items-center justify-center bg-[linear-gradient(145deg,rgb(52,170,220),rgb(0,122,255))]">
            <Volume2 size={13} />
          </div>
          <div>
            <div className="text-[11px] leading-none text-muted-foreground mb-0.5">
              Master volume
            </div>
            <div className="text-[12.5px] font-medium leading-none text-foreground">
              MacBook Pro Speakers
            </div>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-[12px] tabular-nums font-medium text-muted-foreground">
            {Math.trunc(volume)}%
          </span>
          <Button size="icon" className="size-4.75 rounded-full  bg-black/5 dark:bg-white/[0.07] text-muted-foreground hover:text-foreground hover:bg-black/8 dark:hover:bg-white/12 &_svg" variant="secondary" >
            <Volume2 size={12} />
          </Button>
        </div>
      </div>
      <Slider
        aria-label="System output volume"
        disabled={disabled}
        defaultValue={[volume]}
        max={100}
        value={[volume]}
        min={0}
        step={0.001}
        className="mx-auto w-full max-w-xs"
        onValueChange={([vol]) => onChange(vol)}
      />
    </section>
  )
}
