const { invoke, event } = window.__TAURI__.tauri;

const x = document.getElementById("x");
const y = document.getElementById("y");
const mouse_click = document.getElementById("mouse_click");


const data = await invoke("rust_data");

async function update(e) {
    x.textContent = e.payload.x;
    y.textContent = e.payload.y;
    mouse_click.textContent = e.payload.click;
}

window.__TAURI__.event.listen("mouse", update);
update();
