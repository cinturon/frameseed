# FrameSeed architecture

FrameSeed separates **render** (pixel generation), **encode** (file export), and **interface** (CLI or desktop UI). The core library never decides whether output becomes a PNG, MP4, or preview image — callers choose after frames exist.

## Crate map

| Crate | Role |
|-------|------|
| `frameseed-core` | Pixel model, scenes, effects, config parsing, deterministic render loop |
| `frameseed-encoder` | PNG sequence → MP4/GIF via FFmpeg, contact sheet assembly |
| `frameseed-cli` | Batch rendering and discovery commands |
| `frameseed-app` | Tauri shell: preview, parameters, render queue, export dialog |

## Data flow

```text
RenderConfig (TOML)
       │
       ▼
 scene_from_config ──► Scene::render ──► Frame (Vec<Rgba>)
       │                                      │
       ▼                                      ▼
 effects_from_config ──► Effect::apply ──► finished frame
       │
       ▼
 render_sequence / render_preview_frame ──► PNG file(s)
       │
       ▼
 export_video (encoder) ──► MP4 or GIF
```

## Core model

- **`Rgba`** — four `u8` channels, no nested color types.
- **`Frame`** — flat `Vec<Rgba>` indexed by `y * width + x`.
- **`RenderContext`** — `frame_index`, `normalized_time`, `time_seconds`, and `seed` passed into every scene and effect.

Scenes and effects receive the same context so animation and RNG stay deterministic for a given config + seed.

## Plugin traits

```rust
pub trait Scene {
    fn name(&self) -> &str;
    fn render(&self, frame: &mut Frame, context: &RenderContext);
}

pub trait Effect {
    fn name(&self) -> &str;
    fn apply(&mut self, frame: &mut Frame, context: &RenderContext);
}
```

`scene_from_config` and `effects_from_config` map TOML tables to concrete types. Adding a scene means implementing the trait, wiring the registry, and extending `SceneConfig` — not changing the render loop.

## Determinism

RNG uses `ChaCha8Rng` seeded from config. Same width, height, fps, duration, seed, scene parameters, and effects → identical pixels on every run. Parallel pixel loops (where used) preserve ordering so snapshots stay stable.

## Config and presets

- **`examples/`** — teaching configs referenced in tests and docs.
- **`presets/`** — gallery-ready combinations located via `presets_dir()` (anchored from the core crate manifest, not the process CWD).

`RenderConfig::validate()` rejects zero dimensions, zero fps/duration, and missing scene names before rendering starts.

## Desktop app integration

Tauri commands load presets, render a single preview frame to base64 PNG, and queue full exports on a background thread. Progress and errors emit as Tauri events (`render-progress`, `render-complete`, `render-error`) so the web UI stays responsive.

## Extension points

1. New scene — algorithm + registry entry + optional preset.
2. New effect — `Effect` impl + `EffectsConfig` field + registry wiring.
3. New export format — encoder crate (keep FFmpeg invocation out of core).
4. New UI — any front end that builds `RenderConfig` and calls the same render/export functions as the CLI.
