import init from "./bevy-game.js";
import { show_toast } from "./functions.js";
async function run() {
  await init();
}
run();

setTimeout(() => show_toast(), 1100);

function goHome() {
  console.log("logged goHome");
  window.location.href = "/home";
}
const canvas = document.getElementById("game-canvas");
canvas.addEventListener("blur", () => canvas.focus());
canvas.addEventListener("load", () => canvas.focus());
