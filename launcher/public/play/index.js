import init from "./bevy-game.js";
async function run() {
  await init();
}

document.body.style.overflow = "hidden";
document.addEventListener("contextmenu", (event) => event.preventDefault());

const f = (e) => {
  for (let i = 1; i <= 12; i++) {
    if (e.key == `F${i}`) {
      e.preventDefault();
    }
  }
  if (
    (e.ctrlKey || e.shiftKey) &&
    !(e.ctrlKey && ["C", "c", "A", "a", "V", "v"].includes(e.key))
  ) {
    e.preventDefault();
  }
};

document.onkeydown = f;
document.onclick = f;

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
