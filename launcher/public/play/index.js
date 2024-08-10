import init from "./bevy-game.js";
async function run() {
  await init();
}

window.__TAURI__.tauri.invoke("get_user").then((user) => {
  if (!user.username) {
    document.getElementById("app").setHTMLUnsafe("Not logged in");
    document.getElementById("app").style.color = "white";
  } else {
    window.user = user;
    run();
    const canvas = document.getElementById("game-canvas");
    canvas.addEventListener("blur", () => canvas.focus());
    canvas.addEventListener("load", () => canvas.focus());
  }
});
