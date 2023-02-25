const { invoke } = window.__TAURI__.tauri;
const filesDom = document.getElementById("files");

invoke("get_path").then((path) => {
  console.log(path);
});

console.log(filesDom);
filesDom.innerHTML = "<button>Asdf</button>";
