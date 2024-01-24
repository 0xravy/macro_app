const { invoke } = window.__TAURI__.tauri;

const x = document.getElementById("x");
const y = document.getElementById("y");
const mouse_click = document.getElementById("mouse_click");
const mouse_is_press = document.getElementById("mouse_is_press");
const comment = document.getElementById("comment");

invoke("rust_data");

/**
 * x: f64,
 * y: f64,
 * mouse_click: String,
 * mouse_is_press: bool,
 */


window.__TAURI__.event.listen("mouse", e => {
    x.textContent = e.payload.x;
    y.textContent = e.payload.y;
    mouse_click.textContent = e.payload.mouse_click;
    mouse_is_press.textContent = e.payload.mouse_is_press;
});

window.__TAURI__.event.listen("status", e => {
    comment.textContent = e.payload.comment;
});

