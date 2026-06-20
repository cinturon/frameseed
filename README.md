# FrameSeed

FrameSeed is a procedural video engine written in Rust. Every pixel is synthesized from algorithms, configuration, and a seed — no imported footage. Render still previews, MP4 clips, GIF loops, and contact sheets from TOML presets or custom configs.

## Prerequisites

- [Rust](https://rustup.rs/) (1.77 or newer)
- [FFmpeg](https://ffmpeg.org/) on your `PATH` (required for MP4/GIF export)

```bash
# macOS
brew install ffmpeg

# Debian/Ubuntu
sudo apt install ffmpeg
```

## Quick start

```bash
git clone https://github.com/cinturon/frameseed.git
cd frameseed
cargo build --release
```

Discover built-in scenes and presets:

```bash
cargo run -p frameseed-cli -- list-scenes
cargo run -p frameseed-cli -- list-presets
```

Preview a single frame:

```bash
cargo run -p frameseed-cli -- preview --scene particles --seed 42 --output preview.png
cargo run -p frameseed-cli -- preview --preset gradient --seed 42 --output preview.png
```

Render a video from a preset or config file:

```bash
cargo run -p frameseed-cli -- render --preset gradient
cargo run -p frameseed-cli -- render --config examples/sine_wave.toml
cargo run -p frameseed-cli -- render --preset nebula --output-format gif
cargo run -p frameseed-cli -- render --preset flow_field --contact-sheet
```

Output lands in `output/` (`video.mp4` or `animation.gif`, plus `sequence/` PNG frames). Add `--contact-sheet` to also write `output/contact_sheet.png`.

## Desktop app

Run the Tauri UI during development:

```bash
cd crates/frameseed-app
cargo tauri dev
```

The app provides live preview, parameter controls, a render queue with progress, and native MP4/GIF export dialogs.

See [Packaging the desktop app](#packaging-the-desktop-app) below for distributable builds.

## Configuration

Render settings live in TOML. Width, height, fps, duration, and seed control timing and determinism. The `[scene]` table selects a procedural generator; optional `[effects.*]` tables post-process the frame.

Minimal example (`examples/gradient.toml`):

```toml
width = 640
height = 360
fps = 24.0
duration = 5.0
seed = 42

[scene]
name = "gradient"

[scene.gradient]
palette = "sunset"
speed = 1.0
```

Available scene names (also from `frameseed list-scenes`):

`gradient`, `noise_clouds`, `conway`, `particles`, `flow_field`, `sdf_shapes`, `mandelbrot`, `voronoi`

Gallery presets in `presets/` combine scenes with effects — open any file there as a starting point.

Save your own preset:

```bash
cargo run -p frameseed-cli -- presets save --name my_clip --config examples/gradient.toml
```

## Project layout

```text
frameseed/
  crates/
    frameseed-core/     # Frame, scenes, effects, config, render loop
    frameseed-encoder/  # FFmpeg export, contact sheets
    frameseed-cli/      # Command-line interface
    frameseed-app/      # Tauri desktop shell
  presets/              # Curated gallery presets
  examples/             # Example TOML configs
  output/               # CLI render output (gitignored)
```

Architecture details: [docs/architecture.md](docs/architecture.md).

## Packaging the desktop app

Build a standalone app bundle you can share without requiring Rust on the recipient machine:

```bash
cd crates/frameseed-app
cargo tauri build
```

Artifacts are written to `crates/frameseed-app/target/release/bundle/`:

| Platform | Output |
|----------|--------|
| macOS | `bundle/macos/Frameseed.app` and `.dmg` |
| Linux | `bundle/deb/`, `bundle/appimage/` |
| Windows | `bundle/msi/`, `bundle/nsis/` |

The bundled app includes the UI and render engine. **FFmpeg must still be installed** on the target machine for MP4/GIF export from the app.

For local development without packaging:

```bash
cd crates/frameseed-app
cargo tauri dev
```

## Tests

```bash
cargo test
```

## License

See repository license file if present.
