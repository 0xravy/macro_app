use rdev::{simulate, Button, Event, EventType, Key};

#[derive(Clone)]
pub struct MousePos {
    x: f64,
    y: f64,
    click: Option<Button>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Lala {
    x: f64,
    y: f64,
    click: String,
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
    let mut mouse_map: Vec<MousePos> = Vec::new();
    let mut is_record: bool = false;
    let mut last_x = 0_f64;
    let mut last_y = 0_f64;
    let mut last_click = Button::Unknown(0);

    let macro_loop = move |event: Event| {
        let w = turi_window.clone();

        match &event.event_type {
            EventType::MouseMove { x, y } => {
                last_x = *x;
                last_y = *y;
                let string_click = format!("{:?}", last_click);
                let _ = w.emit(
                    "mouse",
                    Lala {
                        x: *x,
                        y: *y,
                        click: string_click,
                    },
                );
            }
            EventType::ButtonPress(btn) => {
                last_click = *btn;
                if is_record {
                    mouse_map.push(MousePos {
                        x: last_x,
                        y: last_y,
                        click: Some(*btn),
                    });
                }
            }
            EventType::KeyPress(key) => {
                if Key::KeyZ == *key {
                    for m in &mouse_map {
                        send(&EventType::MouseMove { x: m.x, y: m.y });

                        if m.click.is_some() {
                            send(&EventType::ButtonPress(m.click.unwrap()));
                            send(&EventType::ButtonRelease(m.click.unwrap()));
                            std::thread::sleep(std::time::Duration::from_millis(300));
                        }
                    }
                }
                if Key::KeyC == *key {
                    is_record = false;
                    mouse_map.clear();
                    println!("stop & delete the recroding {:?}", is_record);
                }
                if Key::KeyR == *key {
                    is_record = true;
                    println!("start recording {:?}", is_record);
                }
            }
            _ => (),
        };
    };

    if let Err(error) = rdev::listen(macro_loop) {
        println!("Error: {:?}", error)
    }
}
