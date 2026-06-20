const status = document.getElementById("status");

window.__TAURI__.core
  .invoke("welcome")
  .then((message) => {
    status.textContent = message;
  })
  .catch((error) => {
    status.textContent = `Error: ${error}`;
  });
