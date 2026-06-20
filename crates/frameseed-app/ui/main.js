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
const previewEmpty       = document.getElementById("preview-empty");
const previewOverlay     = document.getElementById("preview-overlay");
const previewOverlayText = document.getElementById("preview-overlay-text");
const previewInfo        = document.getElementById("preview-info");
const exportButton       = document.getElementById("export");
const exportFormatSelect = document.getElementById("export-format");
const progressContainer  = document.getElementById("progress-container");
const progressBarFill    = document.getElementById("progress-bar-fill");
const renderStatus       = document.getElementById("render-status");

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
};

// Wire up badges — each range input syncs its badge on input
document.querySelectorAll("input[type=range]").forEach((el) => {
  const badge = badges[el.id];
  if (badge) {
    el.addEventListener("input", () => { badge.textContent = el.value; });
  }
});

// ── State ────────────────────────────────────────────────────────────────────
let currentConfig = null;
let refreshTimer  = null;
let exportRunning = false;
let previewing    = false;

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
function setExportRunning(running) {
  exportRunning = running;
  exportButton.disabled = running;
  progressContainer.hidden = !running;
  if (!running) {
    progressBarFill.style.width = "0%";
  }
}

function updateRenderProgress(event) {
  const { phase, current, total } = event.payload;
  const percent = total > 0 ? Math.round((current / total) * 100) : 0;
  progressBarFill.style.width = `${percent}%`;
  renderStatus.textContent = `${phase}: ${current}/${total}`;
  renderStatus.className = "";
}

async function queueExport() {
  const config = buildConfigFromForm();
  if (!config || exportRunning) return;

  progressContainer.hidden = false;
  try {
    setExportRunning(true);
    progressBarFill.style.width = "0%";
    renderStatus.textContent = "Choose save location…";
    renderStatus.className = "";
    await invoke("export_render", {
      request: { config, output_format: exportFormatSelect.value },
    });
  } catch (error) {
    setExportRunning(false);
    renderStatus.textContent = `Error: ${error}`;
    renderStatus.className = "error";
    console.error("Error exporting:", error);
  }
}

async function setupRenderEvents() {
  await listen("render-progress", (event) => updateRenderProgress(event));
  await listen("render-complete", (event) => {
    setExportRunning(false);
    progressBarFill.style.width = "100%";
    const path = event.payload.output_path || "";
    const filename = path.split("/").pop() || path;
    renderStatus.textContent = `Saved: ${filename}`;
    renderStatus.className = "success";
  });
  await listen("render-error", (event) => {
    setExportRunning(false);
    renderStatus.textContent = `Error: ${event.payload.message}`;
    renderStatus.className = "error";
  });
  await listen("export-cancelled", () => {
    setExportRunning(false);
    renderStatus.textContent = "Export cancelled";
    renderStatus.className = "";
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
  s.gradient.palette   = inputs.gradientPalette.value || null;
  s.gradient.direction = inputs.gradientDirection.value;

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

  return currentConfig;
}

// ── Preview ───────────────────────────────────────────────────────────────────
async function refreshPreview() {
  const config = buildConfigFromForm();
  if (!config || previewing) return;

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
    previewEmpty.classList.add("hidden");
    previewInfo.textContent =
      `${config.width}×${config.height}  frame ${frameIndex + 1}/${totalFrames(config)}  seed ${config.seed}`;
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

exportButton.addEventListener("click", (e) => {
  e.preventDefault();
  queueExport();
});

randomizeSeedBtn.addEventListener("click", () => {
  seedInput.value = String(Math.floor(Math.random() * 1_000_000_000) + 1);
  scheduleRefresh();
});

// All live inputs (range + select + number) trigger a debounced refresh
const liveInputs = [
  seedInput, widthInput, heightInput, fpsInput, durationInput,
  ...Object.values(inputs),
];
liveInputs.forEach((el) => {
  if (!el) return;
  el.addEventListener("input", scheduleRefresh);
  el.addEventListener("change", scheduleRefresh);
});

loadInitialState()
  .then(() => setupRenderEvents())
  .catch(console.error);
