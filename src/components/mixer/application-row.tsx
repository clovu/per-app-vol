import { Volume2, VolumeX } from 'lucide-react'

import type { RunningApp } from './types'
import React from 'react'
import { Button } from '../ui'
import { buttonVariants } from '../ui/button'
import { cn } from '@/lib/utils'
import { Slider } from '../ui/slider'

const btnGhostClass = buttonVariants({ variant: 'ghost', size: 'lg' })

interface ApplicationRowProps {
  app: RunningApp
  onChange?: (volume: number) => void
}

export function ApplicationRow({ app, onChange = () => { } }: ApplicationRowProps) {
  const record = React.useRef<number>(null)

  return (
    <article className={cn(
      btnGhostClass,
      'group flex items-center gap-2.5 px-3 py-2.5 rounded-xl w-full size-auto dark:hover:bg-white/8 hover:bg-black/4',
    )}>
      <div className="size-6.5 rounded-[8px] flex items-center justify-center shrink-0 text-white text-[11px] font-semibold shadow-sm bg-[rgb(90_200_250)]" aria-hidden="true">
        {app.name.slice(0, 1).toUpperCase()}
      </div>
      <div className="flex-1 min-w-0 flex flex-col gap-1.25">
        <div className="flex items-center justify-between gap-2">
          <span className="text-[12.5px] font-medium leading-none truncate text-foreground">{app.name}</span>
          <output className="text-[11px] tabular-nums shrink-0 leading-none text-muted-foreground">{app.controllable ? `${app.volume}%` : 'Coming soon'}</output>
        </div>

        <div className="flex items-center gap-2">
          <Button
            size={'icon'}
            className="mute-button size-auto bg-none text-muted-foreground hover:text-foreground"
            variant="link"
            title={`mute ${app.name}`}
            disabled={!app.controllable}
          >
            {app.muted ? <VolumeX size={13} /> : <Volume2 size={13} />}
          </Button>
          <Slider
            aria-label={`${app.name} volume`}
            min={0}
            max={100}
            step={0.001}
            // value={[app.volume]}
            defaultValue={[app.volume]}
            disabled={!app.controllable}
            // style={{ '--volume': `${app.volume}%` } as React.CSSProperties}
            onValueChange={([vol]) => {
              const volume = Number(vol)
              if (volume === record.current)
                return
              onChange(record.current = volume)
            }}
          />
        </div>
      </div>
    </article>
  )
}
