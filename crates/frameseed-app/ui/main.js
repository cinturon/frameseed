const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

const presetSelect = document.getElementById("preset");
const seedInput = document.getElementById("seed");
const widthInput = document.getElementById("width");
const heightInput = document.getElementById("height");
const fpsInput = document.getElementById("fps");
const durationInput = document.getElementById("duration");
const frameIndexInput = document.getElementById("frame-index");
const frameIndexLabel = document.getElementById("frame-index-label");
const refreshButton = document.getElementById("refresh");
const previewImage = document.getElementById("preview");
const exportButton = document.getElementById("export");
const exportFormatSelect = document.getElementById("export-format");
const renderProgress = document.getElementById("render-progress");
const renderStatus = document.getElementById("render-status");

const gradientSpeedInput = document.getElementById("gradient-speed");
const gradientPaletteInput = document.getElementById("gradient-palette");
const noiseSpeedInput = document.getElementById("noise-speed");
const noiseScaleInput = document.getElementById("noise-scale");
const particlesCountInput = document.getElementById("particles-count");
const particlesSpeedInput = document.getElementById("particles-speed");
const particlesKindInput = document.getElementById("particles-kind");
const particlesFpsInput = document.getElementById("particles-fps");
const conwayCellSizeInput = document.getElementById("conway-cell-size");
const conwayDensityInput = document.getElementById("conway-density");
const flowCountInput = document.getElementById("flow-count");
const flowSpeedInput = document.getElementById("flow-speed");
const flowScaleInput = document.getElementById("flow-scale");
const flowFpsInput = document.getElementById("flow-fps");
const sdfCircleRadiusInput = document.getElementById("sdf-circle-radius");
const sdfBoxHalfWidthInput = document.getElementById("sdf-box-half-width");
const sdfBoxHalfHeightInput = document.getElementById("sdf-box-half-height");
const sdfSpeedInput = document.getElementById("sdf-speed");
const mandelbrotMaxIterInput = document.getElementById("mandelbrot-max-iter");
const mandelbrotCenterReInput = document.getElementById("mandelbrot-center-re");
const mandelbrotCenterImInput = document.getElementById("mandelbrot-center-im");
const mandelbrotViewWidthInput = document.getElementById("mandelbrot-view-width");
const mandelbrotZoomSpeedInput = document.getElementById("mandelbrot-zoom-speed");
const voronoiSeedCountInput = document.getElementById("voronoi-seed-count");
const voronoiSpeedInput = document.getElementById("voronoi-speed");
const voronoiEdgeWidthInput = document.getElementById("voronoi-edge-width");

const panels = {
  gradient: document.getElementById("scene-gradient"),
  noise_clouds: document.getElementById("scene-noise-clouds"),
  particles: document.getElementById("scene-particles"),
  conway: document.getElementById("scene-conway"),
  flow_field: document.getElementById("scene-flow-field"),
  sdf_shapes: document.getElementById("scene-sdf-shapes"),
  mandelbrot: document.getElementById("scene-mandelbrot"),
  voronoi: document.getElementById("scene-voronoi"),
};

let currentConfig = null;
let refreshTimer = null;
let exportRunning = false;

function setExportRunning(running) {
  exportRunning = running;
  exportButton.disabled = running;
}

function updateRenderProgress(event) {
  const { phase, current, total } = event.payload;
  const percent = total > 0 ? Math.round((current / total) * 100) : 0;
  renderProgress.value = String(percent);
  renderProgress.max = "100";
  renderStatus.textContent = `${phase}: ${current}/${total}`;
}

async function queueExport() {
  const config = buildConfigFromForm();
  if (!config || exportRunning) {
    return;
  }

  try {
    setExportRunning(true);
    renderProgress.value = "0";
    renderStatus.textContent = "Queued…";
    await invoke("queue_render", {
      request: {
        config,
        output_format: exportFormatSelect.value,
      },
    });
  } catch (error) {
    setExportRunning(false);
    renderStatus.textContent = `Error: ${error}`;
    console.error("Error queueing render:", error);
  }
}

async function setupRenderEvents() {
  await listen("render-progress", (event) => {
    updateRenderProgress(event);
  });

  await listen("render-complete", (event) => {
    setExportRunning(false);
    renderProgress.value = "100";
    renderStatus.textContent = `Done: ${event.payload.output_path}`;
  });

  await listen("render-error", (event) => {
    setExportRunning(false);
    renderStatus.textContent = `Error: ${event.payload.message}`;
  });
}

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

function showScenePanel(name) {
  Object.values(panels).forEach((panel) => {
    panel.hidden = true;
  });
  if (panels[name]) {
    panels[name].hidden = false;
  }
}

function totalFrames(config) {
  return Math.max(1, Math.ceil(config.duration * config.fps));
}

function updateFrameSliderMax(config) {
  const maxFrame = totalFrames(config) - 1;
  frameIndexInput.max = String(maxFrame);
  const frameIndex = Math.min(Number(frameIndexInput.value) || 0, maxFrame);
  frameIndexInput.value = String(frameIndex);
  frameIndexLabel.textContent = String(frameIndex);
}

function syncFormFromConfig(config) {
  seedInput.value = String(config.seed);
  widthInput.value = String(config.width);
  heightInput.value = String(config.height);
  fpsInput.value = String(config.fps);
  durationInput.value = String(config.duration);

  gradientSpeedInput.value = String(config.scene.gradient.speed);
  gradientPaletteInput.value = config.scene.gradient.palette || "default";

  noiseSpeedInput.value = String(config.scene.noise_clouds.speed);
  noiseScaleInput.value = String(config.scene.noise_clouds.scale);

  particlesCountInput.value = String(config.scene.particles.count);
  particlesSpeedInput.value = String(config.scene.particles.speed);
  particlesKindInput.value = config.scene.particles.kind;
  particlesFpsInput.value = String(config.scene.particles.fps);

  conwayCellSizeInput.value = String(config.scene.conway.cell_size);
  conwayDensityInput.value = String(config.scene.conway.density);

  flowCountInput.value = String(config.scene.flow_field.count);
  flowSpeedInput.value = String(config.scene.flow_field.speed);
  flowScaleInput.value = String(config.scene.flow_field.scale);
  flowFpsInput.value = String(config.scene.flow_field.fps);

  sdfCircleRadiusInput.value = String(config.scene.sdf_shapes.circle_radius);
  sdfBoxHalfWidthInput.value = String(config.scene.sdf_shapes.box_half_width);
  sdfBoxHalfHeightInput.value = String(config.scene.sdf_shapes.box_half_height);
  sdfSpeedInput.value = String(config.scene.sdf_shapes.speed);

  mandelbrotMaxIterInput.value = String(config.scene.mandelbrot.max_iter);
  mandelbrotCenterReInput.value = String(config.scene.mandelbrot.center_re);
  mandelbrotCenterImInput.value = String(config.scene.mandelbrot.center_im);
  mandelbrotViewWidthInput.value = String(config.scene.mandelbrot.initial_view_width);
  mandelbrotZoomSpeedInput.value = String(config.scene.mandelbrot.zoom_speed);

  voronoiSeedCountInput.value = String(config.scene.voronoi.seed_count);
  voronoiSpeedInput.value = String(config.scene.voronoi.speed);
  voronoiEdgeWidthInput.value = String(config.scene.voronoi.edge_width);
}

function buildConfigFromForm() {
  if (!currentConfig) {
    return null;
  }

  currentConfig.seed = Number(seedInput.value);
  currentConfig.width = Number(widthInput.value);
  currentConfig.height = Number(heightInput.value);
  currentConfig.fps = Number(fpsInput.value);
  currentConfig.duration = Number(durationInput.value);

  currentConfig.scene.gradient.speed = Number(gradientSpeedInput.value);
  currentConfig.scene.gradient.palette = gradientPaletteInput.value;

  currentConfig.scene.noise_clouds.speed = Number(noiseSpeedInput.value);
  currentConfig.scene.noise_clouds.scale = Number(noiseScaleInput.value);

  currentConfig.scene.particles.count = Number(particlesCountInput.value);
  currentConfig.scene.particles.speed = Number(particlesSpeedInput.value);
  currentConfig.scene.particles.kind = particlesKindInput.value;
  currentConfig.scene.particles.fps = Number(particlesFpsInput.value);

  currentConfig.scene.conway.cell_size = Number(conwayCellSizeInput.value);
  currentConfig.scene.conway.density = Number(conwayDensityInput.value);

  currentConfig.scene.flow_field.count = Number(flowCountInput.value);
  currentConfig.scene.flow_field.speed = Number(flowSpeedInput.value);
  currentConfig.scene.flow_field.scale = Number(flowScaleInput.value);
  currentConfig.scene.flow_field.fps = Number(flowFpsInput.value);

  currentConfig.scene.sdf_shapes.circle_radius = Number(sdfCircleRadiusInput.value);
  currentConfig.scene.sdf_shapes.box_half_width = Number(sdfBoxHalfWidthInput.value);
  currentConfig.scene.sdf_shapes.box_half_height = Number(sdfBoxHalfHeightInput.value);
  currentConfig.scene.sdf_shapes.speed = Number(sdfSpeedInput.value);

  currentConfig.scene.mandelbrot.max_iter = Number(mandelbrotMaxIterInput.value);
  currentConfig.scene.mandelbrot.center_re = Number(mandelbrotCenterReInput.value);
  currentConfig.scene.mandelbrot.center_im = Number(mandelbrotCenterImInput.value);
  currentConfig.scene.mandelbrot.initial_view_width = Number(mandelbrotViewWidthInput.value);
  currentConfig.scene.mandelbrot.zoom_speed = Number(mandelbrotZoomSpeedInput.value);

  currentConfig.scene.voronoi.seed_count = Number(voronoiSeedCountInput.value);
  currentConfig.scene.voronoi.speed = Number(voronoiSpeedInput.value);
  currentConfig.scene.voronoi.edge_width = Number(voronoiEdgeWidthInput.value);

  return currentConfig;
}

async function onPresetChange(slug) {
  currentConfig = await invoke("preset_config", { preset: slug });
  syncFormFromConfig(currentConfig);
  showScenePanel(currentConfig.scene.name);
  updateFrameSliderMax(currentConfig);
  await refreshPreview();
}

async function refreshPreview() {
  const config = buildConfigFromForm();
  if (!config) {
    return;
  }

  try {
    updateFrameSliderMax(config);
    const base64 = await invoke("preview_frame", {
      request: {
        config,
        frame_index: Number(frameIndexInput.value),
      },
    });
    previewImage.src = `data:image/png;base64,${base64}`;
  } catch (error) {
    console.error("Error refreshing preview:", error);
  }
}

function scheduleRefresh() {
  clearTimeout(refreshTimer);
  refreshTimer = setTimeout(() => {
    refreshPreview();
  }, 150);
}

async function loadInitialState() {
  await loadPresets();
  if (presetSelect.value) {
    await onPresetChange(presetSelect.value);
  }
}

presetSelect.addEventListener("change", () => {
  onPresetChange(presetSelect.value).catch((error) => {
    console.error("Error loading preset:", error);
  });
});

frameIndexInput.addEventListener("input", () => {
  frameIndexLabel.textContent = frameIndexInput.value;
  scheduleRefresh();
});

refreshButton.addEventListener("click", (event) => {
  event.preventDefault();
  refreshPreview();
});

exportButton.addEventListener("click", (event) => {
  event.preventDefault();
  queueExport();
});

const liveInputs = [
  seedInput,
  widthInput,
  heightInput,
  fpsInput,
  durationInput,
  gradientSpeedInput,
  gradientPaletteInput,
  noiseSpeedInput,
  noiseScaleInput,
  particlesCountInput,
  particlesSpeedInput,
  particlesKindInput,
  particlesFpsInput,
  conwayCellSizeInput,
  conwayDensityInput,
  flowCountInput,
  flowSpeedInput,
  flowScaleInput,
  flowFpsInput,
  sdfCircleRadiusInput,
  sdfBoxHalfWidthInput,
  sdfBoxHalfHeightInput,
  sdfSpeedInput,
  mandelbrotMaxIterInput,
  mandelbrotCenterReInput,
  mandelbrotCenterImInput,
  mandelbrotViewWidthInput,
  mandelbrotZoomSpeedInput,
  voronoiSeedCountInput,
  voronoiSpeedInput,
  voronoiEdgeWidthInput,
];

liveInputs.forEach((input) => {
  input.addEventListener("input", scheduleRefresh);
  input.addEventListener("change", scheduleRefresh);
});

loadInitialState()
  .then(() => setupRenderEvents())
  .catch((error) => {
    console.error("Error loading app:", error);
  });
