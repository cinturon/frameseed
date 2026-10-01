# FrameSeed Capstone — *Drift*

A 45-second procedural short built from a single `flow_field` scene with motion blur, sunset palette quantization, and light VHS/CRT post-processing. Every frame is deterministic from the config and seed below.

## Reproduce exactly

**Requirements:** Rust toolchain, FFmpeg on `PATH`.

```bash
git clone https://github.com/cinturon/frameseed.git
cd frameseed
cargo build --release
```

Render the MP4 and contact sheet:

```bash
cargo run --release -p frameseed-cli -- render \
  --preset capstone \
  --contact-sheet \
  --contact-sheet-step 24 \
  --contact-sheet-cols 6
```

### Outputs

| File | Location |
|------|----------|
| MP4 | `output/video.mp4` |
| Frame sequence | `output/sequence/frame_*.png` |
| Contact sheet | `output/contact_sheet.png` |

GIF variant:

```bash
cargo run --release -p frameseed-cli -- render --preset capstone --output-format gif
```

Preview a single frame without a full render:

```bash
cargo run --release -p frameseed-cli -- preview \
  --preset capstone \
  --frame-index 540 \
  --output capstone_preview.png
```

## Canonical parameters

| Field | Value |
|-------|-------|
| Preset file | `presets/capstone.toml` |
| Seed | `506200` |
| Resolution | 1280 × 720 |
| FPS | 24 |
| Duration | 45 s (1080 frames) |
| Scene | `flow_field` |
| Effects | motion blur, palette (`sunset`), VHS/CRT |

Changing any field (including seed) produces different pixels. Restoring the table values reproduces this film byte-for-byte on the same FrameSeed version.

## What to look for

- Thousands of flow-field particles weaving through noise-driven vector fields
- Motion blur smearing trails between frames
- Sunset palette bands compressing color into retro gradients
- Subtle scanlines, chromatic offset, and warp evoking worn analog video

## Desktop app

Open the **Capstone — Drift** gallery entry (or load preset `capstone`), confirm seed `506200`, and export via the UI save dialog. The same config object drives CLI and app output.
