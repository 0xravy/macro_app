const { invoke } = window.__TAURI__.tauri;
const { event } = window.__TAURI__;

const x = document.getElementById("x");
const y = document.getElementById("y");
const mouse_click = document.getElementById("mouse_click");
const mouse_is_press = document.getElementById("mouse_is_press");
const key = document.getElementById("key");
const key_is_press = document.getElementById("key_is_press");
const comment = document.getElementById("comment");
const speed_millis = document.getElementById("speed_millis");

/********

x: f64,
y: f64,
mouse_click: String,
mouse_is_press: bool,
key: String,
key_is_press: bool,

********/

async function main() {

    await invoke("send_data");

    await event.listen("mouse", (e) => {
        x.textContent = e.payload.x;
        y.textContent = e.payload.y;
        mouse_click.textContent = e.payload.mouse_click;
        mouse_is_press.textContent = e.payload.mouse_is_press;
        key.textContent = e.payload.key;
        key_is_press.textContent = e.payload.key_is_press;
    });

    await event.listen("status", (e) => {
        comment.textContent = e.payload.comment;
    });


    speed_millis.oninput = (_) => {
        console.log("lala");
        window.__TAURI__.invoke("get_data", { speed: Number(speed_millis.value) });
    };
}

main();
