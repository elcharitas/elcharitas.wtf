import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const entryPath = resolve("build/elcharitas_new.js");
const fixedEntry = `import wasm from "./elcharitas_new_bg.wasm";
export * from "./elcharitas_new_bg.js";
import { __wbg_set_wasm } from "./elcharitas_new_bg.js";

const instance = new WebAssembly.Instance(wasm, {});
__wbg_set_wasm(instance.exports);
instance.exports.__wbindgen_start();
`;

const currentEntry = readFileSync(entryPath, "utf8");

if (currentEntry !== fixedEntry) {
  writeFileSync(entryPath, fixedEntry);
}