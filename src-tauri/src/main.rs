mod functions;
use functions::mouse_macro::InputManager;
use functions::tauri_commands::*;

fn main() {
    let tauri = tauri::Builder::default();

    tauri.manage(InputManager::new())
        .invoke_handler(tauri::generate_handler![init, change_speed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
