const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;
const { listen } = window.__TAURI__.event;

let fileListEl;
let selectBtnEl;
let processBtnEl;
let maxWidthEl;
let maxHeightEl;
let qualityEl;
const jobs = [];
let processing = false;



window.addEventListener("DOMContentLoaded", () => {
  fileListEl = document.querySelector("#file-list");
  selectBtnEl = document.querySelector("#select-btn");
  processBtnEl = document.querySelector("#process-btn");
  maxWidthEl = document.querySelector("#max-width");
  maxHeightEl = document.querySelector("#max-height");
  qualityEl = document.querySelector("#quality");

  listen("tauri://file-drop", (event) => {
    addFiles(event.payload);
  });

  processBtnEl.addEventListener("click", processFiles);
  selectBtnEl.addEventListener("click", selectFiles);
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

async function selectFiles() {
  const selected = await open({
    multiple: true,
    filters: [{ name: "Zip Files", extensions: ["zip"] }],
  });
  if (!selected) return;
  if (Array.isArray(selected)) {
    addFiles(selected);
  } else {
    addFiles([selected]);
  }
}


async function processFiles() {
  if (processing || jobs.length === 0) return;
  processing = true;
  const parseNumber = (el) => {
    const v = parseInt(el.value, 10);
    return Number.isNaN(v) ? null : v;
  };
  const opts = {
    maxWidth: parseNumber(maxWidthEl),
    maxHeight: parseNumber(maxHeightEl),
    quality: parseNumber(qualityEl),
  };
  await Promise.all(
    jobs.map(async (job) => {
      job.statusEl.textContent = "processing";
      job.statusEl.className = "status processing";
      try {
        await invoke("process_zip_cmd", { path: job.path, options: opts });
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

