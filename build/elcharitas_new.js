import wasm from "./elcharitas_new_bg.wasm";
import * as __wbg from "./elcharitas_new_bg.js";
import { __wbg_set_wasm } from "./elcharitas_new_bg.js";
export * from "./elcharitas_new_bg.js";

const instance = new WebAssembly.Instance(wasm, { "./elcharitas_new_bg.js": __wbg });
__wbg_set_wasm(instance.exports);
instance.exports.__wbindgen_start();

export default {
  async fetch(request, env, ctx) {
    const fn = __wbg.fetch ?? __wbg.handle;
    if (typeof fn === "function") {
      return fn(request, env, ctx);
    }
    return new Response("Worker not initialised", { status: 503 });
  },
};
