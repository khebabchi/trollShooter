import achievements from "../achievements.js";
export function show_toast(id = 1) {
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
