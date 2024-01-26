use super::mouse_macro::InputManager;
use tauri::Manager;
use tauri::State;

#[tauri::command]
pub fn init(w: tauri::Window) {
    println!("{:?}", w.title());
    std::thread::spawn(move || {
        let cloned = w.clone();
        let manager = w.state::<InputManager>();
        manager.hook(cloned);
    });
}

#[tauri::command]
pub fn change_speed(w: tauri::Window, speed: u64) {
    println!("{:?}", speed);
    let input_manager: State<InputManager> = w.state();

    input_manager.set_speed(speed);
}
