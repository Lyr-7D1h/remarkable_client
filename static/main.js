const { invoke } = window.__TAURI__.tauri;
const filesDom = document.getElementById("files");

const scan = {};
const fs = {};

// setInterval(() => {
invoke("scan").then((entries) => {
  for (const entry of entries) {
    scan[entry.mac] = entry;
  }
  render();
});
// }, 1000);

const deviceButton = (device) => `
<button type="button" onclick="connect('${device.mac}')" style="padding-right: 15px;">
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V64c0-35.3-28.7-64-64-64H64zM176 432h96c8.8 0 16 7.2 16 16s-7.2 16-16 16H176c-8.8 0-16-7.2-16-16s7.2-16 16-16z"/></svg>
  ${device.ip}
</button>`;

/// Connect to a device
function connect(device) {
  if (device.known === false) {
    invoke("add_device", { mac: device.mac }).catch();
  }
  invoke("fs", { mac }).then((fs) => {
    console.log(fs);
  });
}

function render() {
  // load devices_block
  if (Object.keys(scan).length == 0) return;

  document.getElementById("devices_block").innerHTML = `<div>${Object.values(
    scan
  ).map(deviceButton)}</div>`;
  document.getElementById("loading_block").style.display = "none";
  document.getElementById("devices_block").style.display = "block";
}
