import "./app.css";
import App from "./App.svelte";
import init, { BoardState, Piece } from "@mono/wasm-core";

init().then(() => {
  const state = BoardState.from_fen("");
  console.log(state);
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
