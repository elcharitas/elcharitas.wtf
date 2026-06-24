import wasm from "./elcharitas_new_bg.wasm";
import * as __wbg from "./elcharitas_new_bg.js";
import { __wbg_set_wasm, handle } from "./elcharitas_new_bg.js";
export * from "./elcharitas_new_bg.js";

const instance = new WebAssembly.Instance(wasm, { "./elcharitas_new_bg.js": __wbg });
__wbg_set_wasm(instance.exports);
instance.exports.__wbindgen_start();

export default {
  async fetch(request, env, ctx) {
    if (typeof handle === "function") {
      return handle(request, env, ctx);
    }
    return new Response("Worker not initialised", { status: 503 });
  },
};
