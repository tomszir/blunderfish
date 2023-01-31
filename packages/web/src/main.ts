import "./app.css";
import App from "./App.svelte";
import init, { BoardState, Piece } from "@mono/wasm-core";

init().then(() => {
  const state = BoardState.from_fen(
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  );
  console.log(state);
});

const app = new App({
  target: document.getElementById("app"),
});

export default app;
