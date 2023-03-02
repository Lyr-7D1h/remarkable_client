const { invoke } = window.__TAURI__.tauri;
const filesDom = document.getElementById("files");

const devices = {};
// setInterval(() => {
// invoke("devices").then((d) => {
//   console.log(devices);
//   for (const device of d) {
//     devices[device.mac] = device;
//   }
//   render();
// });
// }, 1000);

function render() {
  if (devices.length == 0) return;
  console.log(devices);

  document.getElementById("loading_block").style.display = "none";

  document.getElementById("devices_block").innerHTML = `<div>${Object.values(
    devices
  ).map((device) => `<button>${device.ip}</button>`)}</div>`;
}
