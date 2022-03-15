import "./assets/favicon.ico"
import "./assets/logo-full.png"
import "./assets/logo.svg"
import "./assets/logo-opacity.svg"
import "./assets/profile.jpg"
import "./assets/reset.scss"
import "./assets/style.scss"
const wasmPromise = import("./dist/app");

import("./js").then(js => {
    //@ts-expect-error
    window["_wasm_js_bridge"] = js
    wasmPromise.then(app => app.start(process.env.BUILD_MODE === "dev" ? app.AppMode.Dev : app.AppMode.Production))
})

export {}