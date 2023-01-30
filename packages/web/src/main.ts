import "./app.css";
import App from "./App.svelte";
import init, { init_debug } from "@mono/wasm-core";

init().then(() => {
  init_debug();
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
