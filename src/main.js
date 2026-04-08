const { invoke } = window.__TAURI__.core;

let modelInputEl;
let resultMsgEl;
let vramDetailsEl;
let modelNameEl;
let fp16El;
let fp32El;
let int8El;
let int4El;
let kvCacheEl;

async function calculateVram() {
  const modelName = modelInputEl.value.trim();
  if (!modelName) {
    resultMsgEl.textContent = "Please enter a model name";
    resultMsgEl.className = "error";
    vramDetailsEl.classList.add("hidden");
    return;
  }

  try {
    const result = await invoke("calculate_vram", { modelName });

    if (result.found) {
      resultMsgEl.textContent = `Found model: ${result.model_name}`;
      resultMsgEl.className = "success";

      modelNameEl.textContent = result.model_name;
      fp16El.textContent = `${result.fp16_gb.toFixed(1)} GB`;
      fp32El.textContent = `${result.fp32_gb.toFixed(1)} GB`;
      int8El.textContent = `${result.int8_gb.toFixed(1)} GB`;
      int4El.textContent = `${result.int4_gb.toFixed(1)} GB`;
      kvCacheEl.textContent = `${result.with_kv_cache_fp16.toFixed(1)} GB`;

      vramDetailsEl.classList.remove("hidden");
    } else {
      resultMsgEl.textContent = `Model "${result.model_name}" not found in database. Try: llama-3-8b, mistral-7b, mixtral-8x7b, etc.`;
      resultMsgEl.className = "error";
      vramDetailsEl.classList.add("hidden");
    }
  } catch (error) {
    resultMsgEl.textContent = `Error: ${error.message}`;
    resultMsgEl.className = "error";
    vramDetailsEl.classList.add("hidden");
  }
}

window.addEventListener("DOMContentLoaded", () => {
  modelInputEl = document.querySelector("#model-input");
  resultMsgEl = document.querySelector("#result-msg");
  vramDetailsEl = document.querySelector("#vram-details");
  modelNameEl = document.querySelector("#model-name");
  fp16El = document.querySelector("#fp16-value");
  fp32El = document.querySelector("#fp32-value");
  int8El = document.querySelector("#int8-value");
  int4El = document.querySelector("#int4-value");
  kvCacheEl = document.querySelector("#kv-cache-value");

  document.querySelector("#vram-form").addEventListener("submit", (e) => {
    e.preventDefault();
    calculateVram();
  });
});
