const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

// ── DOM refs ─────────────────────────────────────────────────────────────────
const presetSelect       = document.getElementById("preset");
const seedInput          = document.getElementById("seed");
const randomizeSeedBtn   = document.getElementById("randomize-seed");
const widthInput         = document.getElementById("width");
const heightInput        = document.getElementById("height");
const fpsInput           = document.getElementById("fps");
const durationInput      = document.getElementById("duration");
const frameIndexInput    = document.getElementById("frame-index");
const frameIndexLabel    = document.getElementById("frame-index-label");
const refreshButton      = document.getElementById("refresh");
const previewImage       = document.getElementById("preview");
const animCanvas         = document.getElementById("anim-canvas");
const playPreviewBtn     = document.getElementById("play-preview");
const playIcon           = document.getElementById("play-icon");
const thumbnailStrip     = document.getElementById("thumbnail-strip");
const previewEmpty       = document.getElementById("preview-empty");
const previewOverlay     = document.getElementById("preview-overlay");
const previewOverlayText = document.getElementById("preview-overlay-text");
const previewInfo        = document.getElementById("preview-info");
const exportButton       = document.getElementById("export");
const copyTomlBtn        = document.getElementById("copy-toml");
const exportFormatSelect = document.getElementById("export-format");
const exportBitrateSelect = document.getElementById("export-bitrate");
const bitrateRow         = document.getElementById("bitrate-row");
const progressContainer  = document.getElementById("progress-container");
const progressBarFill    = document.getElementById("progress-bar-fill");
const renderStatus       = document.getElementById("render-status");

// Effects inputs
const fxInvert           = document.getElementById("fx-invert");
const fxBlurOn           = document.getElementById("fx-blur-on");
const fxBlurParams       = document.getElementById("fx-blur-params");
const fxBlurRadius       = document.getElementById("fx-blur-radius");
const fxPixelOn          = document.getElementById("fx-pixelation-on");
const fxPixelParams      = document.getElementById("fx-pixelation-params");
const fxPixelSize        = document.getElementById("fx-pixelation-size");
const fxDitherOn         = document.getElementById("fx-dither-on");
const fxDitherParams     = document.getElementById("fx-dither-params");
const fxDitherSpread     = document.getElementById("fx-dither-spread");
const fxPaletteOn        = document.getElementById("fx-palette-on");
const fxPaletteParams    = document.getElementById("fx-palette-params");
const fxPaletteName      = document.getElementById("fx-palette-name");
const fxMotionOn         = document.getElementById("fx-motion-blur-on");
const fxMotionParams     = document.getElementById("fx-motion-blur-params");
const fxMotionStrength   = document.getElementById("fx-motion-blur-strength");
const fxBcOn             = document.getElementById("fx-bc-on");
const fxBcParams         = document.getElementById("fx-bc-params");
const fxBcBrightness     = document.getElementById("fx-bc-brightness");
const fxBcContrast       = document.getElementById("fx-bc-contrast");
const fxVhsOn            = document.getElementById("fx-vhs-on");
const fxVhsParams        = document.getElementById("fx-vhs-params");
const fxVhsScanlines     = document.getElementById("fx-vhs-scanlines");
const fxVhsChroma        = document.getElementById("fx-vhs-chroma");
const fxVhsNoise         = document.getElementById("fx-vhs-noise");
const fxVhsWarp          = document.getElementById("fx-vhs-warp");

// Gradient color pickers
const gradientStartColor = document.getElementById("gradient-start-color");
const gradientEndColor   = document.getElementById("gradient-end-color");

// Blend scene
const blendSceneA        = document.getElementById("blend-scene-a");
const blendSceneB        = document.getElementById("blend-scene-b");
const blendSpeed         = document.getElementById("blend-speed");

// Scene param inputs
const inputs = {
  // gradient
  gradientSpeed:       document.getElementById("gradient-speed"),
  gradientPalette:     document.getElementById("gradient-palette"),
  gradientDirection:   document.getElementById("gradient-direction"),
  // noise clouds
  noiseSpeed:          document.getElementById("noise-speed"),
  noiseScale:          document.getElementById("noise-scale"),
  noiseColored:        document.getElementById("noise-colored"),
  // particles
  particlesCount:      document.getElementById("particles-count"),
  particlesSpeed:      document.getElementById("particles-speed"),
  particlesKind:       document.getElementById("particles-kind"),
  // conway
  conwayCellSize:      document.getElementById("conway-cell-size"),
  conwayDensity:       document.getElementById("conway-density"),
  // flow field
  flowCount:           document.getElementById("flow-count"),
  flowSpeed:           document.getElementById("flow-speed"),
  flowScale:           document.getElementById("flow-scale"),
  // sdf shapes
  sdfCircleRadius:     document.getElementById("sdf-circle-radius"),
  sdfBoxHalfWidth:     document.getElementById("sdf-box-half-width"),
  sdfBoxHalfHeight:    document.getElementById("sdf-box-half-height"),
  sdfSpeed:            document.getElementById("sdf-speed"),
  // mandelbrot
  mandelbrotMaxIter:   document.getElementById("mandelbrot-max-iter"),
  mandelbrotZoomSpeed: document.getElementById("mandelbrot-zoom-speed"),
  mandelbrotCenterRe:  document.getElementById("mandelbrot-center-re"),
  mandelbrotCenterIm:  document.getElementById("mandelbrot-center-im"),
  // voronoi
  voronoiSeedCount:    document.getElementById("voronoi-seed-count"),
  voronoiSpeed:        document.getElementById("voronoi-speed"),
  voronoiEdgeWidth:    document.getElementById("voronoi-edge-width"),
  // plasma
  plasmaSpeed:         document.getElementById("plasma-speed"),
  plasmaScale:         document.getElementById("plasma-scale"),
  // lissajous
  lissajousA:          document.getElementById("lissajous-a"),
  lissajousB:          document.getElementById("lissajous-b"),
  lissajousSpeed:      document.getElementById("lissajous-speed"),
  lissajousTrail:      document.getElementById("lissajous-trail"),
  // sine wave
  sineSpeed:           document.getElementById("sine-speed"),
  // starfield
  starfieldCount:      document.getElementById("starfield-count"),
  starfieldSpeed:      document.getElementById("starfield-speed"),
  // tunnel
  tunnelSpeed:         document.getElementById("tunnel-speed"),
  tunnelRings:         document.getElementById("tunnel-rings"),
};

// Scene panels
const panels = {
  gradient:     document.getElementById("scene-gradient"),
  noise_clouds: document.getElementById("scene-noise-clouds"),
  particles:    document.getElementById("scene-particles"),
  conway:       document.getElementById("scene-conway"),
  flow_field:   document.getElementById("scene-flow-field"),
  sdf_shapes:   document.getElementById("scene-sdf-shapes"),
  mandelbrot:   document.getElementById("scene-mandelbrot"),
  voronoi:      document.getElementById("scene-voronoi"),
  plasma:       document.getElementById("scene-plasma"),
  lissajous:    document.getElementById("scene-lissajous"),
  sine_wave:    document.getElementById("scene-sine-wave"),
  starfield:    document.getElementById("scene-starfield"),
  tunnel:       document.getElementById("scene-tunnel"),
  blend:        document.getElementById("scene-blend"),
};

// ── Value badges ─────────────────────────────────────────────────────────────
const badges = {
  "gradient-speed":        document.getElementById("gradient-speed-val"),
  "noise-speed":           document.getElementById("noise-speed-val"),
  "noise-scale":           document.getElementById("noise-scale-val"),
  "particles-count":       document.getElementById("particles-count-val"),
  "particles-speed":       document.getElementById("particles-speed-val"),
  "conway-cell-size":      document.getElementById("conway-cell-size-val"),
  "conway-density":        document.getElementById("conway-density-val"),
  "flow-count":            document.getElementById("flow-count-val"),
  "flow-speed":            document.getElementById("flow-speed-val"),
  "flow-scale":            document.getElementById("flow-scale-val"),
  "sdf-circle-radius":     document.getElementById("sdf-circle-radius-val"),
  "sdf-box-half-width":    document.getElementById("sdf-box-half-width-val"),
  "sdf-box-half-height":   document.getElementById("sdf-box-half-height-val"),
  "sdf-speed":             document.getElementById("sdf-speed-val"),
  "mandelbrot-max-iter":   document.getElementById("mandelbrot-max-iter-val"),
  "mandelbrot-zoom-speed": document.getElementById("mandelbrot-zoom-speed-val"),
  "voronoi-seed-count":    document.getElementById("voronoi-seed-count-val"),
  "voronoi-speed":         document.getElementById("voronoi-speed-val"),
  "voronoi-edge-width":    document.getElementById("voronoi-edge-width-val"),
  "plasma-speed":          document.getElementById("plasma-speed-val"),
  "plasma-scale":          document.getElementById("plasma-scale-val"),
  "lissajous-a":           document.getElementById("lissajous-a-val"),
  "lissajous-b":           document.getElementById("lissajous-b-val"),
  "lissajous-speed":       document.getElementById("lissajous-speed-val"),
  "lissajous-trail":       document.getElementById("lissajous-trail-val"),
  "sine-speed":            document.getElementById("sine-speed-val"),
  "starfield-count":       document.getElementById("starfield-count-val"),
  "starfield-speed":       document.getElementById("starfield-speed-val"),
  "tunnel-speed":          document.getElementById("tunnel-speed-val"),
  "tunnel-rings":          document.getElementById("tunnel-rings-val"),
  "blend-speed":           document.getElementById("blend-speed-val"),
  // effects
  "fx-blur-radius":        document.getElementById("fx-blur-radius-val"),
  "fx-pixelation-size":    document.getElementById("fx-pixelation-size-val"),
  "fx-dither-spread":      document.getElementById("fx-dither-spread-val"),
  "fx-motion-blur-strength": document.getElementById("fx-motion-blur-strength-val"),
  "fx-bc-brightness":      document.getElementById("fx-bc-brightness-val"),
  "fx-bc-contrast":        document.getElementById("fx-bc-contrast-val"),
  "fx-vhs-scanlines":      document.getElementById("fx-vhs-scanlines-val"),
  "fx-vhs-chroma":         document.getElementById("fx-vhs-chroma-val"),
  "fx-vhs-noise":          document.getElementById("fx-vhs-noise-val"),
  "fx-vhs-warp":           document.getElementById("fx-vhs-warp-val"),
};

// Wire up badges — each range input syncs its badge on input
document.querySelectorAll("input[type=range]").forEach((el) => {
  const badge = badges[el.id];
  if (badge) {
    el.addEventListener("input", () => { badge.textContent = el.value; });
  }
});

// Wire effect toggle visibility
function wireEffectToggle(checkbox, paramsDiv) {
  checkbox.addEventListener("change", () => {
    paramsDiv.hidden = !checkbox.checked;
    scheduleRefresh();
  });
}
wireEffectToggle(fxBlurOn,    fxBlurParams);
wireEffectToggle(fxPixelOn,   fxPixelParams);
wireEffectToggle(fxDitherOn,  fxDitherParams);
wireEffectToggle(fxPaletteOn, fxPaletteParams);
wireEffectToggle(fxMotionOn,  fxMotionParams);
wireEffectToggle(fxBcOn,      fxBcParams);
wireEffectToggle(fxVhsOn,     fxVhsParams);

// Gradient: palette preset → update color pickers
const PALETTE_COLORS = {
  "": null,
  sunset:   ["#ff6b35", "#1a1a2e"],
  ocean:    ["#0077b6", "#caf0f8"],
  forest:   ["#2d6a4f", "#d8f3dc"],
  fire:     ["#e63946", "#ffb703"],
  purple:   ["#7b2d8b", "#e040fb"],
  ice:      ["#a8dadc", "#1d3557"],
  rose:     ["#ff4d6d", "#ffccd5"],
  midnight: ["#0d0221", "#3a0ca3"],
};

inputs.gradientPalette.addEventListener("change", () => {
  const colors = PALETTE_COLORS[inputs.gradientPalette.value];
  if (colors) {
    gradientStartColor.value = colors[0];
    gradientEndColor.value = colors[1];
  }
  scheduleRefresh();
});

gradientStartColor.addEventListener("input", () => {
  inputs.gradientPalette.value = "";
  scheduleRefresh();
});
gradientEndColor.addEventListener("input", () => {
  inputs.gradientPalette.value = "";
  scheduleRefresh();
});

// ── State ────────────────────────────────────────────────────────────────────
let currentConfig = null;
let refreshTimer  = null;
let exportRunning = false;
let previewing    = false;

// Animation preview state
let animFrames    = [];
let animIndex     = 0;
let animTimer     = null;
let animPlaying   = false;

// ── Collapsible sections ─────────────────────────────────────────────────────
document.querySelectorAll(".section-toggle").forEach((btn) => {
  btn.addEventListener("click", () => {
    const section = btn.closest(".collapsible");
    const open = section.classList.toggle("open");
    btn.setAttribute("aria-expanded", String(open));
  });
});

// ── Overlay helpers ──────────────────────────────────────────────────────────
function showOverlay(text = "Rendering…") {
  previewOverlay.hidden = false;
  previewOverlayText.textContent = text;
}
function hideOverlay() {
  previewOverlay.hidden = true;
}

// ── Export state ─────────────────────────────────────────────────────────────
function setExportBusy(busy) {
  exportRunning = busy;
  exportButton.disabled = busy;
}

function showProgress(text, cls = "") {
  progressContainer.hidden = false;
  progressBarFill.style.width = "0%";
  renderStatus.textContent = text;
  renderStatus.className = cls;
}

function updateRenderProgress(event) {
  const { phase, current, total } = event.payload;
  const percent = total > 0 ? Math.round((current / total) * 100) : 0;
  progressBarFill.style.width = `${percent}%`;
  const label = phase === "rendering" ? "Rendering" : "Encoding";
  renderStatus.textContent = `${label} frame ${current} of ${total} (${percent}%)`;
  renderStatus.className = "";
}

async function queueExport() {
  if (exportRunning) return;

  let config;
  try {
    config = buildConfigFromForm();
  } catch (err) {
    console.error("buildConfigFromForm error:", err);
    showProgress(`Config error: ${err.message}`, "error");
    return;
  }

  if (!config) {
    showProgress("Load a preset first.", "error");
    return;
  }

  setExportBusy(true);
  showProgress("Opening save dialog…");

  try {
    const fmt = exportFormatSelect.value;
    await invoke("export_render", {
      request: {
        config,
        output_format: fmt,
        bitrate: fmt !== "gif" ? exportBitrateSelect.value : null,
      },
    });
  } catch (error) {
    setExportBusy(false);
    renderStatus.textContent = `Error: ${error}`;
    renderStatus.className = "error";
    console.error("Export error:", error);
  }
}

// ── Copy TOML ─────────────────────────────────────────────────────────────────
async function copyConfigToml() {
  const config = buildConfigFromForm();
  if (!config) return;
  try {
    const toml = await invoke("export_config_to_toml", { config });
    await navigator.clipboard.writeText(toml);
    const orig = copyTomlBtn.textContent;
    copyTomlBtn.textContent = "✓ Copied";
    setTimeout(() => { copyTomlBtn.textContent = orig; }, 1500);
  } catch (err) {
    console.error("Failed to copy TOML:", err);
  }
}

async function setupRenderEvents() {
  // Show bitrate control only for MP4 and WebM.
  function syncBitrateVisibility() {
    bitrateRow.hidden = exportFormatSelect.value === "gif";
  }
  exportFormatSelect.addEventListener("change", syncBitrateVisibility);
  syncBitrateVisibility();

  await listen("render-progress", (event) => updateRenderProgress(event));
  await listen("render-complete", (event) => {
    setExportBusy(false);
    progressBarFill.style.width = "100%";
    const path = event.payload.output_path || "";
    const filename = path.split("/").pop() || path;
    renderStatus.textContent = `Saved: ${filename}`;
    renderStatus.className = "success";
  });
  await listen("render-error", (event) => {
    setExportBusy(false);
    renderStatus.textContent = `Error: ${event.payload.message}`;
    renderStatus.className = "error";
  });
  await listen("export-cancelled", () => {
    setExportBusy(false);
    progressBarFill.style.width = "0%";
    renderStatus.textContent = "Export cancelled.";
    renderStatus.className = "";
  });
}

// ── Animation preview ─────────────────────────────────────────────────────────
function stopAnimation() {
  if (animTimer !== null) {
    clearInterval(animTimer);
    animTimer = null;
  }
  animPlaying = false;
  playIcon.textContent = "▶";
  animCanvas.hidden = true;
  previewImage.hidden = false;
}

async function startAnimation() {
  const config = buildConfigFromForm();
  if (!config) return;

  showOverlay("Rendering preview frames…");
  playPreviewBtn.disabled = true;

  try {
    animFrames = await invoke("render_animation_preview", {
      request: { config, frame_index: null },
    });
  } catch (err) {
    console.error("Animation preview error:", err);
    hideOverlay();
    playPreviewBtn.disabled = false;
    return;
  }

  hideOverlay();
  playPreviewBtn.disabled = false;

  if (!animFrames.length) return;

  // Set canvas dimensions to match the preview image
  const img = new Image();
  img.onload = () => {
    animCanvas.width = img.naturalWidth;
    animCanvas.height = img.naturalHeight;

    // Also build thumbnail strip
    buildThumbnailStrip(animFrames);
  };
  img.src = `data:image/png;base64,${animFrames[0]}`;

  previewImage.hidden = true;
  animCanvas.hidden = false;

  animIndex = 0;
  animPlaying = true;
  playIcon.textContent = "⏹";

  const fps = Number(fpsInput.value) || 24;
  const interval = 1000 / fps;
  const ctx = animCanvas.getContext("2d");

  function drawFrame() {
    const image = new Image();
    image.onload = () => { ctx.drawImage(image, 0, 0, animCanvas.width, animCanvas.height); };
    image.src = `data:image/png;base64,${animFrames[animIndex]}`;
    animIndex = (animIndex + 1) % animFrames.length;
  }

  drawFrame();
  animTimer = setInterval(drawFrame, interval);
}

function toggleAnimation() {
  if (animPlaying) {
    stopAnimation();
  } else {
    startAnimation();
  }
}

// ── Thumbnail strip ───────────────────────────────────────────────────────────
function buildThumbnailStrip(frames) {
  if (!frames.length) { thumbnailStrip.hidden = true; return; }

  thumbnailStrip.innerHTML = "";
  thumbnailStrip.hidden = false;

  const step = Math.max(1, Math.floor(frames.length / 8));
  const selected = [];
  for (let i = 0; i < frames.length; i += step) {
    selected.push(frames[i]);
    if (selected.length >= 8) break;
  }

  selected.forEach((b64, i) => {
    const img = document.createElement("img");
    img.src = `data:image/png;base64,${b64}`;
    img.title = `Frame ${i * step + 1}`;
    img.className = "thumb";
    img.addEventListener("click", () => {
      stopAnimation();
      previewImage.src = `data:image/png;base64,${b64}`;
      frameIndexInput.value = String(Math.min(i * step, Number(frameIndexInput.max)));
      frameIndexLabel.textContent = frameIndexInput.value;
    });
    thumbnailStrip.appendChild(img);
  });
}

// ── Presets ──────────────────────────────────────────────────────────────────
async function loadPresets() {
  const entries = await invoke("list_gallery");
  presetSelect.replaceChildren();
  entries.forEach((entry) => {
    const option = document.createElement("option");
    option.value = entry.slug;
    option.textContent = entry.title;
    presetSelect.appendChild(option);
  });
}

// ── Scene panels ─────────────────────────────────────────────────────────────
function showScenePanel(name) {
  Object.values(panels).forEach((p) => { p.hidden = true; });
  if (panels[name]) panels[name].hidden = false;
}

// ── Frame slider ─────────────────────────────────────────────────────────────
function totalFrames(config) {
  return Math.max(1, Math.ceil(config.duration * config.fps));
}

function updateFrameSliderMax(config) {
  const maxFrame = totalFrames(config) - 1;
  frameIndexInput.max = String(maxFrame);
  const current = Math.min(Number(frameIndexInput.value) || 0, maxFrame);
  frameIndexInput.value = String(current);
  frameIndexLabel.textContent = String(current);
}

// ── Sync form ↔ config ───────────────────────────────────────────────────────
function rgbaToHex(rgba) {
  if (!rgba) return "#000000";
  const toHex = (n) => n.toString(16).padStart(2, "0");
  return `#${toHex(rgba.r)}${toHex(rgba.g)}${toHex(rgba.b)}`;
}

function syncFormFromConfig(config) {
  seedInput.value    = String(config.seed);
  widthInput.value   = String(config.width);
  heightInput.value  = String(config.height);
  fpsInput.value     = String(config.fps);
  durationInput.value = String(config.duration);

  const s = config.scene;

  // gradient
  inputs.gradientSpeed.value     = String(s.gradient.speed);
  inputs.gradientPalette.value   = s.gradient.palette || "";
  inputs.gradientDirection.value = (s.gradient.direction || "horizontal").toLowerCase();

  // Set color pickers from palette or explicit colors
  const paletteColors = PALETTE_COLORS[s.gradient.palette || ""];
  if (s.gradient.start_color) {
    gradientStartColor.value = s.gradient.start_color;
  } else if (paletteColors) {
    gradientStartColor.value = paletteColors[0];
  }
  if (s.gradient.end_color) {
    gradientEndColor.value = s.gradient.end_color;
  } else if (paletteColors) {
    gradientEndColor.value = paletteColors[1];
  }

  // noise clouds
  inputs.noiseSpeed.value   = String(s.noise_clouds.speed);
  inputs.noiseScale.value   = String(s.noise_clouds.scale);
  inputs.noiseColored.checked = !!s.noise_clouds.colored;

  // particles
  inputs.particlesCount.value = String(s.particles.count);
  inputs.particlesSpeed.value = String(s.particles.speed);
  inputs.particlesKind.value  = s.particles.kind || "snow";

  // conway
  inputs.conwayCellSize.value = String(s.conway.cell_size);
  inputs.conwayDensity.value  = String(s.conway.density);

  // flow field
  inputs.flowCount.value = String(s.flow_field.count);
  inputs.flowSpeed.value = String(s.flow_field.speed);
  inputs.flowScale.value = String(s.flow_field.scale);

  // sdf shapes
  inputs.sdfCircleRadius.value   = String(s.sdf_shapes.circle_radius);
  inputs.sdfBoxHalfWidth.value   = String(s.sdf_shapes.box_half_width);
  inputs.sdfBoxHalfHeight.value  = String(s.sdf_shapes.box_half_height);
  inputs.sdfSpeed.value          = String(s.sdf_shapes.speed);

  // mandelbrot
  inputs.mandelbrotMaxIter.value   = String(s.mandelbrot.max_iter);
  inputs.mandelbrotZoomSpeed.value = String(s.mandelbrot.zoom_speed);
  inputs.mandelbrotCenterRe.value  = String(s.mandelbrot.center_re);
  inputs.mandelbrotCenterIm.value  = String(s.mandelbrot.center_im);

  // voronoi
  inputs.voronoiSeedCount.value = String(s.voronoi.seed_count);
  inputs.voronoiSpeed.value     = String(s.voronoi.speed);
  inputs.voronoiEdgeWidth.value = String(s.voronoi.edge_width);

  // plasma
  if (s.plasma) {
    inputs.plasmaSpeed.value = String(s.plasma.speed);
    inputs.plasmaScale.value = String(s.plasma.scale);
  }

  // lissajous
  if (s.lissajous) {
    inputs.lissajousA.value     = String(s.lissajous.a);
    inputs.lissajousB.value     = String(s.lissajous.b);
    inputs.lissajousSpeed.value = String(s.lissajous.speed);
    inputs.lissajousTrail.value = String(s.lissajous.trail_frames);
  }

  // sine wave
  if (s.sine_wave) {
    inputs.sineSpeed.value = String(s.sine_wave.speed);
  }

  // starfield
  if (s.starfield) {
    inputs.starfieldCount.value = String(s.starfield.count);
    inputs.starfieldSpeed.value = String(s.starfield.speed);
  }

  // tunnel
  if (s.tunnel) {
    inputs.tunnelSpeed.value = String(s.tunnel.speed);
    inputs.tunnelRings.value = String(s.tunnel.rings);
  }

  // blend
  if (s.blend) {
    blendSceneA.value = s.blend.scene_a || "gradient";
    blendSceneB.value = s.blend.scene_b || "plasma";
    blendSpeed.value  = String(s.blend.speed || 1.0);
  }

  // effects
  const e = config.effects;
  fxInvert.checked = !!e.invert;

  fxBlurOn.checked = !!e.blur;
  fxBlurParams.hidden = !e.blur;
  if (e.blur) fxBlurRadius.value = String(e.blur.radius);

  fxPixelOn.checked = !!e.pixelation;
  fxPixelParams.hidden = !e.pixelation;
  if (e.pixelation) fxPixelSize.value = String(e.pixelation.block_size);

  fxDitherOn.checked = !!e.dither;
  fxDitherParams.hidden = !e.dither;
  if (e.dither) fxDitherSpread.value = String(e.dither.spread);

  fxPaletteOn.checked = !!e.palette;
  fxPaletteParams.hidden = !e.palette;
  if (e.palette) fxPaletteName.value = e.palette.name || "cga16";

  fxMotionOn.checked = !!e.motion_blur;
  fxMotionParams.hidden = !e.motion_blur;
  if (e.motion_blur) fxMotionStrength.value = String(e.motion_blur.strength);

  fxBcOn.checked = !!e.brightness_contrast;
  fxBcParams.hidden = !e.brightness_contrast;
  if (e.brightness_contrast) {
    fxBcBrightness.value = String(e.brightness_contrast.brightness);
    fxBcContrast.value   = String(e.brightness_contrast.contrast);
  }

  fxVhsOn.checked = !!e.vhs_crt;
  fxVhsParams.hidden = !e.vhs_crt;
  if (e.vhs_crt) {
    fxVhsScanlines.value = String(e.vhs_crt.scanlines_strength);
    fxVhsChroma.value    = String(e.vhs_crt.chromatic_offset);
    fxVhsNoise.value     = String(e.vhs_crt.noise_amount);
    fxVhsWarp.value      = String(e.vhs_crt.warp_amount);
  }

  // Sync all badges
  Object.entries(badges).forEach(([id, badge]) => {
    const el = document.getElementById(id);
    if (el && badge) badge.textContent = el.value;
  });
}

function buildConfigFromForm() {
  if (!currentConfig) return null;

  currentConfig.seed     = Number(seedInput.value);
  currentConfig.width    = Number(widthInput.value);
  currentConfig.height   = Number(heightInput.value);
  currentConfig.fps      = Number(fpsInput.value);
  currentConfig.duration = Number(durationInput.value);

  const s = currentConfig.scene;

  s.gradient.speed     = Number(inputs.gradientSpeed.value);
  s.gradient.palette   = inputs.gradientPalette.value || "";
  s.gradient.direction = inputs.gradientDirection.value;
  s.gradient.start_color = inputs.gradientPalette.value ? null : gradientStartColor.value;
  s.gradient.end_color   = inputs.gradientPalette.value ? null : gradientEndColor.value;

  s.noise_clouds.speed   = Number(inputs.noiseSpeed.value);
  s.noise_clouds.scale   = Number(inputs.noiseScale.value);
  s.noise_clouds.colored = inputs.noiseColored.checked;

  s.particles.count = Number(inputs.particlesCount.value);
  s.particles.speed = Number(inputs.particlesSpeed.value);
  s.particles.kind  = inputs.particlesKind.value;

  s.conway.cell_size = Number(inputs.conwayCellSize.value);
  s.conway.density   = Number(inputs.conwayDensity.value);

  s.flow_field.count = Number(inputs.flowCount.value);
  s.flow_field.speed = Number(inputs.flowSpeed.value);
  s.flow_field.scale = Number(inputs.flowScale.value);

  s.sdf_shapes.circle_radius  = Number(inputs.sdfCircleRadius.value);
  s.sdf_shapes.box_half_width  = Number(inputs.sdfBoxHalfWidth.value);
  s.sdf_shapes.box_half_height = Number(inputs.sdfBoxHalfHeight.value);
  s.sdf_shapes.speed           = Number(inputs.sdfSpeed.value);

  s.mandelbrot.max_iter   = Number(inputs.mandelbrotMaxIter.value);
  s.mandelbrot.zoom_speed = Number(inputs.mandelbrotZoomSpeed.value);
  s.mandelbrot.center_re  = Number(inputs.mandelbrotCenterRe.value);
  s.mandelbrot.center_im  = Number(inputs.mandelbrotCenterIm.value);

  s.voronoi.seed_count = Number(inputs.voronoiSeedCount.value);
  s.voronoi.speed      = Number(inputs.voronoiSpeed.value);
  s.voronoi.edge_width = Number(inputs.voronoiEdgeWidth.value);

  if (s.plasma) {
    s.plasma.speed = Number(inputs.plasmaSpeed.value);
    s.plasma.scale = Number(inputs.plasmaScale.value);
  }
  if (s.lissajous) {
    s.lissajous.a            = Number(inputs.lissajousA.value);
    s.lissajous.b            = Number(inputs.lissajousB.value);
    s.lissajous.speed        = Number(inputs.lissajousSpeed.value);
    s.lissajous.trail_frames = Number(inputs.lissajousTrail.value);
  }
  if (s.sine_wave) {
    s.sine_wave.speed = Number(inputs.sineSpeed.value);
  }
  if (s.starfield) {
    s.starfield.count = Number(inputs.starfieldCount.value);
    s.starfield.speed = Number(inputs.starfieldSpeed.value);
  }
  if (s.tunnel) {
    s.tunnel.speed = Number(inputs.tunnelSpeed.value);
    s.tunnel.rings = Number(inputs.tunnelRings.value);
  }
  if (s.blend) {
    s.blend.scene_a = blendSceneA.value;
    s.blend.scene_b = blendSceneB.value;
    s.blend.speed   = Number(blendSpeed.value);
  }

  // Effects
  currentConfig.effects.invert = fxInvert.checked;

  currentConfig.effects.blur = fxBlurOn.checked
    ? { radius: Number(fxBlurRadius.value) } : null;

  currentConfig.effects.pixelation = fxPixelOn.checked
    ? { block_size: Number(fxPixelSize.value) } : null;

  currentConfig.effects.dither = fxDitherOn.checked
    ? { spread: Number(fxDitherSpread.value) } : null;

  currentConfig.effects.palette = fxPaletteOn.checked
    ? { name: fxPaletteName.value } : null;

  currentConfig.effects.motion_blur = fxMotionOn.checked
    ? { strength: Number(fxMotionStrength.value) } : null;

  currentConfig.effects.brightness_contrast = fxBcOn.checked
    ? { brightness: Number(fxBcBrightness.value), contrast: Number(fxBcContrast.value) } : null;

  currentConfig.effects.vhs_crt = fxVhsOn.checked
    ? {
        scanlines_strength: Number(fxVhsScanlines.value),
        chromatic_offset:   Number(fxVhsChroma.value),
        noise_amount:       Number(fxVhsNoise.value),
        warp_amount:        Number(fxVhsWarp.value),
      } : null;

  return currentConfig;
}

// ── Preview ───────────────────────────────────────────────────────────────────
async function refreshPreview() {
  const config = buildConfigFromForm();
  if (!config || previewing) return;

  stopAnimation();
  previewing = true;
  showOverlay("Rendering preview…");

  try {
    updateFrameSliderMax(config);
    const frameIndex = Number(frameIndexInput.value);
    const base64 = await invoke("preview_frame", {
      request: { config, frame_index: frameIndex },
    });
    previewImage.src = `data:image/png;base64,${base64}`;
    previewImage.classList.add("loaded");
    previewImage.hidden = false;
    previewEmpty.classList.add("hidden");
    previewInfo.textContent =
      `${config.width}×${config.height}  frame ${frameIndex + 1}/${totalFrames(config)}  seed ${config.seed}`;
    playPreviewBtn.hidden = false;

    // Clear old thumbnail strip when preview refreshes
    thumbnailStrip.hidden = true;
    thumbnailStrip.innerHTML = "";
    animFrames = [];
  } catch (error) {
    console.error("Error refreshing preview:", error);
  } finally {
    previewing = false;
    hideOverlay();
  }
}

function scheduleRefresh() {
  clearTimeout(refreshTimer);
  refreshTimer = setTimeout(refreshPreview, 200);
}

// ── Preset load ───────────────────────────────────────────────────────────────
async function onPresetChange(slug) {
  showOverlay("Loading preset…");
  try {
    currentConfig = await invoke("preset_config", { preset: slug });
    syncFormFromConfig(currentConfig);
    showScenePanel(currentConfig.scene.name);
    updateFrameSliderMax(currentConfig);
    await refreshPreview();
  } catch (e) {
    hideOverlay();
    console.error("Error loading preset:", e);
  }
}

// ── Init ──────────────────────────────────────────────────────────────────────
async function loadInitialState() {
  await loadPresets();
  if (presetSelect.value) {
    await onPresetChange(presetSelect.value);
  }
}

// ── Event wiring ──────────────────────────────────────────────────────────────
presetSelect.addEventListener("change", () =>
  onPresetChange(presetSelect.value).catch(console.error)
);

frameIndexInput.addEventListener("input", () => {
  frameIndexLabel.textContent = frameIndexInput.value;
  scheduleRefresh();
});

refreshButton.addEventListener("click", (e) => {
  e.preventDefault();
  refreshPreview();
});

playPreviewBtn.addEventListener("click", (e) => {
  e.preventDefault();
  toggleAnimation();
});

exportButton.addEventListener("click", (e) => {
  e.preventDefault();
  queueExport();
});

copyTomlBtn.addEventListener("click", (e) => {
  e.preventDefault();
  copyConfigToml();
});

randomizeSeedBtn.addEventListener("click", () => {
  seedInput.value = String(Math.floor(Math.random() * 1_000_000_000) + 1);
  scheduleRefresh();
});

// Resolution presets
document.querySelectorAll(".res-preset").forEach((btn) => {
  btn.addEventListener("click", () => {
    widthInput.value  = btn.dataset.w;
    heightInput.value = btn.dataset.h;
    scheduleRefresh();
  });
});

// All live inputs (range + select + number) trigger a debounced refresh
const liveInputs = [
  seedInput, widthInput, heightInput, fpsInput, durationInput,
  ...Object.values(inputs),
  fxInvert, fxBlurRadius, fxPixelSize, fxDitherSpread, fxPaletteName,
  fxMotionStrength, fxBcBrightness, fxBcContrast,
  fxVhsScanlines, fxVhsChroma, fxVhsNoise, fxVhsWarp,
  blendSceneA, blendSceneB, blendSpeed,
];
liveInputs.forEach((el) => {
  if (!el) return;
  el.addEventListener("input", scheduleRefresh);
  el.addEventListener("change", scheduleRefresh);
});

// Register render events independently so they work even if preset load fails.
setupRenderEvents().catch(console.error);
loadInitialState().catch(console.error);
