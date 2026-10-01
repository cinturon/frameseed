# FrameSeed — Portfolio Case Study

## Elevator pitch

FrameSeed is a procedural video engine I built in Rust to learn systems programming through a creative lens. Every frame is generated from algorithms and configuration — no camera, no stock footage. The same TOML config and seed always reproduce the same pixels, which makes the tool useful for generative art, demos, and portfolio pieces.

## Problem

I wanted a flagship Rust project that forced real engineering decisions: flat buffers vs nested structures, trait-based plugins, deterministic RNG, FFmpeg integration, and eventually a desktop UI — not a tutorial todo app.

## Architecture

FrameSeed splits responsibilities into four crates:

1. **Core** — `Frame`, `RenderContext`, `Scene` and `Effect` traits, TOML config
2. **Encoder** — PNG sequences → MP4/GIF via FFmpeg, contact sheets
3. **CLI** — batch rendering, discovery commands, preview frames
4. **App** — Tauri shell with live preview, parameter controls, render queue, export dialog

Rendering never knows about encoding; the CLI never embeds scene algorithms. That separation kept the codebase teachable as features landed phase by phase.

See [architecture.md](./architecture.md) for the full pipeline diagram.

## Technical highlights

| Area | What I learned / built |
|------|------------------------|
| Pixel model | Flat `Vec<Rgba>` with explicit bounds checking |
| Determinism | `ChaCha8Rng` seeded from config; snapshot tests on sample pixels |
| Scenes | Gradient, noise, Conway, particles, flow fields, Mandelbrot, SDFs, Voronoi |
| Effects | Invert, pixelation, palette quantization, dither, motion blur, VHS/CRT |
| Performance | Rayon parallel loops where scenes allow it without breaking reproducibility |
| Export | Image sequence + FFmpeg rather than binding libffmpeg early |
| Desktop | Tauri commands for preview (base64 PNG), background export with progress events |

## User-facing workflow

**CLI** — discover presets, preview a frame, render MP4/GIF:

```bash
frameseed list-presets
frameseed preview --preset nebula --seed 77 --output preview.png
frameseed render --preset capstone --contact-sheet
```

**Desktop app** — pick a gallery preset, tune parameters, export through a native save dialog.

## Demo artifacts

- Gallery presets in `presets/` (gradient, nebula, fractal zoom, VHS grid, etc.)
- Capstone short film: `presets/capstone.toml` + [capstone.md](./capstone.md)
- Contact sheets for quick visual review of long renders

## Lessons learned

1. **Visible milestones beat big designs.** A horizontal gradient PNG export came before particles or Tauri.
2. **Config errors are UX.** Friendly messages (`list-presets`, FFmpeg install hints) save hours when sharing the tool.
3. **Trait objects fit creative tools.** New scenes plug in without touching the render loop.
4. **Determinism is a feature.** Seeds turn generative output into reproducible portfolio pieces.
5. **Encode is not render.** Keeping FFmpeg in its own crate avoided polluting core with process spawning.

## What I'd do next

- Preset browser thumbnails in the desktop UI
- Optional audio-reactive scenes (stretch track)
- CI render of the capstone preset at reduced resolution for regression checks

## Links

- Repository: [github.com/cinturon/frameseed](https://github.com/cinturon/frameseed)
- README quick start: [../README.md](../README.md)
