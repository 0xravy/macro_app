mod functions;

use functions::mouse_macro::mouse;

#[tauri::command]
fn rust_data(w: tauri::Window) {
    println!("{:?}", w);
    std::thread::spawn(move || mouse(w));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rust_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
