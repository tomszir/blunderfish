import "./app.css";
import App from "./App.svelte";
import init, { greet } from "@mono/wasm-core";

init().then(() => {
  console.log("wasm loaded!");
  greet();
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
