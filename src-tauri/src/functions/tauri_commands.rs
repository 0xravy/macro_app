use super::mouse_macro::mouse;

#[tauri::command]
pub fn send_data(w: tauri::Window) {
    println!("{:?}", w);
    std::thread::spawn(move || mouse(w));
}

#[tauri::command]
pub fn get_data(speed: u32) {
    println!("{:?}", speed);
}
