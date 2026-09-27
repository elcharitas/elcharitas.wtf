import { readFileSync, renameSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const entryPath = resolve("build/elcharitas_new.js");
const wasmPath = resolve("build/elcharitas_new_bg.wasm");
const versionedWasmPath = resolve("build/elcharitas_new_bg_garden.wasm");

renameSync(wasmPath, versionedWasmPath);

const fixedEntry = `import wasm from "./elcharitas_new_bg_garden.wasm";
import * as __wbg from "./elcharitas_new_bg.js";
import { __wbg_set_wasm } from "./elcharitas_new_bg.js";
export * from "./elcharitas_new_bg.js";

const instance = new WebAssembly.Instance(wasm, { "./elcharitas_new_bg.js": __wbg });
__wbg_set_wasm(instance.exports);
instance.exports.__wbindgen_start();

export default {
  async fetch(request, env, ctx) {
    if (typeof __wbg.fetch === "function") {
      return __wbg.fetch(request, env, ctx);
    }
    return new Response("Worker not initialised", { status: 503 });
  },
};
`;

const currentEntry = readFileSync(entryPath, "utf8");

if (currentEntry !== fixedEntry) {
  writeFileSync(entryPath, fixedEntry);
}
