<div align="center">
  <img src="./src-tauri/icons/icon.png" width="88" alt="Per-App Volume icon">
  <h1>Per-App Volume</h1>
  <p>A work-in-progress macOS menu bar volume controller for managing system and per-app volume from a single panel.</p>
</div>

<!-- cspell:ignore pgrep -->

Per-App Volume uses Next.js and React for the interface, with macOS CoreAudio handling system volume control and per-app audio processing.

> [!IMPORTANT]
> This project is still in the early stages of development. System volume control, the menu bar panel, and the app list are functional. Per-app audio processing can currently be tested through a command-line example, while the complete control flow for the menu bar interface is still being implemented.

## Project Goals

macOS provides only a single global volume slider, but music, meetings, games, and browsers often need different volume levels. Even when apps provide their own volume settings, those controls are scattered across different interfaces.

Per-App Volume aims to bring these controls together in the menu bar. Open the panel to see running apps and adjust their volume from one place.

## Current Status

| Feature | Status |
| --- | --- |
| Persistent menu bar presence and native popover panel | Available |
| Read and adjust the default output device volume | Available |
| Automatically list regular running apps | Available |
| Process audio from an individual app by PID | Experimental command-line example available |
| Continuously control per-app volume from the menu bar panel | In development |

> [!NOTE]
> Per-app audio processing relies on CoreAudio Process Tap, introduced in macOS 14.2. Some apps and audio devices may still have compatibility issues.

## How It Works

The Rust backend reads and writes the system volume directly on the current default output device. Because macOS does not provide a public API for changing the volume of an individual app, Per-App Volume uses the target app's PID to locate its CoreAudio process and create a dedicated audio processing pipeline:

`App process → Process Tap → Audio gain adjustment → Default output device`

Process Tap captures the target app's audio stream, an IOProc applies a volume multiplier to each audio sample, and an aggregate device routes the processed audio back to the current default output device.

## Development

- macOS 14.2 or later
- [Xcode Command Line Tools](https://developer.apple.com/xcode/resources/)
- Node.js 23 or later
- [pnpm](https://pnpm.io/installation)
- [Rust](https://www.rust-lang.org/tools/install)

### Run the App

```bash
git clone https://github.com/clovu/per-app-vol.git
cd per-app-vol
pnpm install
pnpm tauri dev
```

## License

[MIT](./LICENSE) License © [Clover You](https://github.com/clovu)
