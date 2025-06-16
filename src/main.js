const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let greetInputEl;
let greetMsgEl;
let fileListEl;
let dropAreaEl;
let processBtnEl;
const jobs = [];
let processing = false;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  dropAreaEl = document.querySelector("#drop-area");
  fileListEl = document.querySelector("#file-list");
  processBtnEl = document.querySelector("#process-btn");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  processBtnEl.addEventListener("click", processFiles);

  setupFileDrop();
});

function addFiles(paths) {
  paths.forEach((p) => {
    if (!p.toLowerCase().endsWith(".zip")) return;
    if (jobs.find((j) => j.path === p)) return;
    const li = document.createElement("li");
    li.innerHTML = `<span class="name">${p}</span> - <span class="status pending">pending</span>`;
    fileListEl.appendChild(li);
    jobs.push({ path: p, statusEl: li.querySelector(".status") });
  });
}

async function setupFileDrop() {
  await listen("tauri://file-drop-hover", () => {
    dropAreaEl.classList.add("hover");
  });
  await listen("tauri://file-drop-cancelled", () => {
    dropAreaEl.classList.remove("hover");
  });
  await listen("tauri://file-drop", (event) => {
    dropAreaEl.classList.remove("hover");
    addFiles(event.payload);
  });
}

async function processFiles() {
  if (processing || jobs.length === 0) return;
  processing = true;
  await Promise.all(
    jobs.map(async (job) => {
      job.statusEl.textContent = "processing";
      job.statusEl.className = "status processing";
      try {
        await invoke("process_zip_cmd", { path: job.path });
        job.statusEl.textContent = "done";
        job.statusEl.className = "status done";
      } catch (e) {
        console.error(e);
        job.statusEl.textContent = "failed";
        job.statusEl.className = "status failed";
      }
    })
  );
  processing = false;
}

