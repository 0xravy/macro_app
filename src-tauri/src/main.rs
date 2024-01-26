mod functions;
use functions::tauri_commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_data, get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
