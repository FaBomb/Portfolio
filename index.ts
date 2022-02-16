import "./assets/favicon.png"
import "./assets/reset.scss"
import "./assets/style.scss"
const wasmPromise = import("./dist/app");

import("./js").then(js => {
    //@ts-expect-error
    window["_wasm_js_bridge"] = js
    wasmPromise.then(app => app.start(process.env.BUILD_MODE === "dev" ? app.AppMode.Dev : app.AppMode.Production))
})

export {}