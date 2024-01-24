use rdev::{simulate, Button, Event, EventType, Key};

#[derive(Clone)]
pub struct MouseMap {
    x: f64,
    y: f64,
    mouse_click: Option<Button>,
    mouse_is_press: bool,
    key: Key,
    key_is_press: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Send2Tuari {
    x: f64,
    y: f64,
    mouse_click: String,
    mouse_is_press: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Status {
    comment: String,
}

impl Status {
    fn recording() -> Self {
        Self {
            comment: "start new recording".to_string(),
        }
    }
    fn stop_recording() -> Self {
        Self {
            comment: "stop & delete the recroding".to_string(),
        }
    }
    fn run_recording() -> Self {
        Self {
            comment: "is run record now".to_string(),
        }
    }
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {:?}", event_type);
        }
    }
}

pub fn mouse(turi_window: tauri::Window) {
    let mut mouse_map: Vec<MouseMap> = Vec::new();
    let mut is_record: bool = false;
    let mut is_press: bool = false;
    let mut last_x = 0_f64;
    let mut last_y = 0_f64;
    let mut last_click = Button::Unknown(0);

    let macro_loop = move |event: Event| {
        let w = turi_window.clone();
        let string_click = format!("{:?}", last_click);

        match &event.event_type {
            EventType::MouseMove { x, y } => {
                last_x = *x;
                last_y = *y;
                let _ = w.emit(
                    "mouse",
                    Send2Tuari {
                        x: last_x,
                        y: last_y,
                        mouse_click: string_click,
                        mouse_is_press: is_press,
                    },
                );
            }
            EventType::ButtonPress(btn) => {
                is_press = true;
                last_click = *btn;
                if is_record {
                    mouse_map.push(MouseMap {
                        x: last_x,
                        y: last_y,
                        mouse_click: Some(*btn),
                        mouse_is_press: is_press,
                    });
                }
                let _ = w.emit(
                    "mouse",
                    Send2Tuari {
                        x: last_x,
                        y: last_y,
                        mouse_click: string_click,
                        mouse_is_press: is_press,
                    },
                );
            }
            EventType::ButtonRelease(btn) => {
                is_press = false;
                last_click = *btn;
                if is_record {
                    mouse_map.push(MouseMap {
                        x: last_x,
                        y: last_y,
                        mouse_click: Some(*btn),
                        mouse_is_press: is_press,
                    });
                }
                let _ = w.emit(
                    "mouse",
                    Send2Tuari {
                        x: last_x,
                        y: last_y,
                        mouse_click: string_click,
                        mouse_is_press: is_press,
                    },
                );
            }
            EventType::KeyPress(key) => {
                if Key::KeyZ == *key {
                    w.emit("status", Status::run_recording()).ok();
                    for mouse_obj in &mouse_map {
                        send(&EventType::MouseMove {
                            x: mouse_obj.x,
                            y: mouse_obj.y,
                        });

                        if mouse_obj.mouse_click.is_some() {
                            if mouse_obj.mouse_is_press {
                                send(&EventType::ButtonPress(mouse_obj.mouse_click.unwrap()));
                            } else {
                                send(&EventType::ButtonRelease(mouse_obj.mouse_click.unwrap()));
                            }
                            std::thread::sleep(std::time::Duration::from_millis(300));
                        }
                    }
                }
                if Key::KeyC == *key {
                    is_record = false;
                    mouse_map.clear();
                    w.emit("status", Status::stop_recording()).ok();
                }
                if Key::KeyR == *key {
                    mouse_map.clear();
                    is_record = true;
                    w.emit("status", Status::recording()).ok();
                }
                let _ = w.emit(
                    "mouse",
                    Send2Tuari {
                        x: last_x,
                        y: last_y,
                        mouse_click: string_click,
                        mouse_is_press: false,
                    },
                );
            }
            _ => (),
        };
    };

    if let Err(error) = rdev::listen(macro_loop) {
        println!("Error: {:?}", error)
    }
}
