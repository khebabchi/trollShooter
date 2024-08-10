import achievements from "../achievements.js";
function show_toast(id = 1) {
  const toastContent = document.createElement("div");
  toastContent.className = "toastify-custom";
  toastContent.innerHTML = `
            <img src="../achievements/${
              achievements[id].img
            }" id="toast-img" alt="Icon" />
            <div><span>Achievement unlocked !</span>
            <strong style="color:${
              achievements[id].rarity == 3
                ? "#cfb35d"
                : achievements[id].rarity == 2
                ? "#9840c4d0"
                : "gray"
            }">${achievements[id].title}</strong></div>
        `;
  toastContent.id = "toast-div";
  Toastify({
    node: toastContent,
    gravity: "bottom",
    position: "left",
    style: {
      background: "rgba(0, 0, 0, 0.7)",
      color: "black",
      "border-radius": "10px",
    },
  }).showToast();
}
export async function postAchievement(id) {
  const url = `https://trollshooterbackend-production.up.railway.app/users/${window.user.username}/achievements/${id}`;
  try {
    const response = await window.__TAURI__.http.fetch(url, {
      method: "POST",
    });
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    } else {
      new Audio("achievement.mp3").play();
      show_toast(id - 1);
    }
  } catch (error) {
    console.error(error.message);
  }
}
export function endGame() {
 setTimeout(()=>window.location.href = "/home",5000)  
}

export async function updateScore(score) {
  const url = `https://trollshooterbackend-production.up.railway.app/users/${window.user.username}/score/${score}`;
  console.log({
    topScore: Math.max(window.user.topScore, score),
    ...window.user,
  });
  console.log(window.user);
  console.log(window.user.topScore);
  console.log(score);
  window.__TAURI__.tauri.invoke("set_user", {
    user: { topScore: Math.max(window.user.topScore, score), ...window.user },
  });
  window.__TAURI__.http.fetch(url, {
    method: "POST",
  });
}
