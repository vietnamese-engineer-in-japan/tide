import * as wasm from "tide";

const fileInputElement = document.getElementById("fileInput");
fileInputElement.addEventListener("change", async _ => {
    const result = await wasm.read_file(fileInputElement.files[0]);
    console.log(result);
});
