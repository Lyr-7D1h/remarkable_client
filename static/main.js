const { invoke } = window.__TAURI__.tauri;
const filesDom = document.getElementById("files");

invoke("devices").then((path) => {
  console.log(path);
});
