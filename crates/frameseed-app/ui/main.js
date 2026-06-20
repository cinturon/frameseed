const presetSelect = document.getElementById("preset");
const seedInput = document.getElementById("seed");
const refreshButton = document.getElementById("refresh");
const previewImage = document.getElementById("preview");
const invoke = window.__TAURI__.core.invoke;

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

async function refreshPreview() {
  if (!presetSelect.value) {
    return;
  }

  try {
    const base64 = await invoke("preview_frame", {
      request: {
        preset: presetSelect.value,
        seed: Number(seedInput.value),
      },
    });
    previewImage.src = `data:image/png;base64,${base64}`;
  } catch (error) {
    console.error("Error refreshing preview:", error);
  }
}

refreshButton.addEventListener("click", (event) => {
  event.preventDefault();
  refreshPreview();
});

seedInput.addEventListener("change", refreshPreview);
presetSelect.addEventListener("change", refreshPreview);

loadPresets()
  .then(refreshPreview)
  .catch((error) => console.error("Error loading presets:", error));
