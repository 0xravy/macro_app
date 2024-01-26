use rdev::{simulate, Button, Event, EventType, Key};
use std::sync::{Arc, Mutex};

pub struct ModifierWatcher {
    captures: Vec<Key>,
    prefixes: Vec<Key>,
}

impl ModifierWatcher {
    fn new(prefixes: Vec<Key>) -> Self {
        Self {
            prefixes,
            captures: Vec::new(),
        }
    }

    fn capture(&mut self, pkey: Key) {
        let check_key = self.prefixes.iter().find(|e| **e == pkey);

        if check_key.is_none() || self.captures.iter().find(|e| **e == pkey).is_some() {
            return;
        }
        println!("{:?}", self.captures);
        self.captures.push(pkey);
    }

    fn passed(&self) -> bool {
        if self.prefixes.len() != self.captures.len() {
            println!("{:?}", self.captures);
            return false;
        }

        for (i, prefix) in self.prefixes.iter().enumerate() {
            if self.captures.iter().nth(i).unwrap() != prefix {
                println!("{:?}", self.captures);
                return false;
            }
        }

        println!("{:?}", self.captures);
        return true;
    }

    fn clear(&mut self) {
        self.captures.clear();
    }
}

#[derive(Clone)]
pub struct MouseMap {
    x: f64,
    y: f64,
    mouse_click: Option<Button>,
    mouse_is_press: bool,
    key: Option<Key>,
    key_is_press: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Send2Tuari {
    x: f64,
    y: f64,
    mouse_click: String,
    mouse_is_press: bool,
    key: String,
    key_is_press: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Status {
    comment: String,
}

impl Status {
    fn start_recording() -> Self {
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

#[derive(Clone)]
pub struct InputManager {
    speed: Arc<Mutex<u64>>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            speed: Arc::new(Mutex::new(0)),
        }
    }

    pub fn hook(&self, window: tauri::Window) {
        let mut mouse_map: Vec<MouseMap> = Vec::new();
        let mut is_record: bool = false;
        let mut mouse_is_press: bool = false;
        let mut key_is_press: bool = false;
        let mut last_x = 0_f64;
        let mut last_y = 0_f64;
        let mut last_btn = Button::Unknown(0);
        let mut last_key = Key::Unknown(0);
        let mut modifier = ModifierWatcher::new(vec![Key::Alt]);

        let w = window.clone();

        let self_clone = Arc::new(self.clone());

        let macro_loop = move |event: Event| {
            let string_btn = format!("{:?}", last_btn);
            let string_key = format!("{:?}", last_key);

            let _evets_type_loop = match &event.event_type {
                /* mouse move */
                EventType::MouseMove { x, y } => {
                    last_x = *x;
                    last_y = *y;
                    let _ = w.emit(
                        "mouse",
                        Send2Tuari {
                            x: last_x,
                            y: last_y,
                            mouse_click: string_btn,
                            mouse_is_press,
                            key: string_key,
                            key_is_press,
                        },
                    );
                }

                /* button press */
                EventType::ButtonPress(btn) => {
                    mouse_is_press = true;
                    last_btn = *btn;
                    if is_record {
                        mouse_map.push(MouseMap {
                            x: last_x,
                            y: last_y,
                            mouse_click: Some(last_btn),
                            mouse_is_press,
                            key: Some(last_key),
                            key_is_press,
                        });
                    }

                    let _ = w.emit(
                        "mouse",
                        Send2Tuari {
                            x: last_x,
                            y: last_y,
                            mouse_click: string_btn,
                            mouse_is_press,
                            key: string_key,
                            key_is_press,
                        },
                    );
                }

                /* button release */
                EventType::ButtonRelease(btn) => {
                    mouse_is_press = false;
                    last_btn = *btn;
                    if is_record {
                        mouse_map.push(MouseMap {
                            x: last_x,
                            y: last_y,
                            mouse_click: Some(last_btn),
                            mouse_is_press,
                            key: Some(last_key),
                            key_is_press,
                        });
                    }
                    let _ = w.emit(
                        "mouse",
                        Send2Tuari {
                            x: last_x,
                            y: last_y,
                            mouse_click: string_btn,
                            mouse_is_press,
                            key: string_key,
                            key_is_press,
                        },
                    );
                }

                /* key press */
                EventType::KeyPress(key) => {
                    modifier.capture(*key);
                    key_is_press = true;
                    last_key = *key;
                    if is_record {
                        mouse_map.push(MouseMap {
                            x: last_x,
                            y: last_y,
                            mouse_click: Some(last_btn),
                            mouse_is_press,
                            key: Some(last_key),
                            key_is_press,
                        });
                    }

                    /* start recording */
                    if Key::KeyZ == *key && modifier.passed() {
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

                                std::thread::sleep(std::time::Duration::from_millis(
                                    *self_clone.speed.lock().unwrap(),
                                ));

                                println!("End Sleep");
                            } else if mouse_obj.key.is_some() {
                                if mouse_obj.key_is_press {
                                    send(&EventType::KeyPress(mouse_obj.key.unwrap()));
                                } else {
                                    send(&EventType::KeyRelease(mouse_obj.key.unwrap()));
                                }
                            }
                        }
                    }

                    /* stop recording */
                    if Key::KeyC == *key && modifier.passed() {
                        is_record = false;
                        mouse_map.clear();
                        w.emit("status", Status::stop_recording()).ok();
                    }

                    /* start recording */
                    if Key::KeyR == *key && modifier.passed() {
                        mouse_map.clear();
                        is_record = true;
                        w.emit("status", Status::start_recording()).ok();
                    }

                    let _ = w.emit(
                        "mouse",
                        Send2Tuari {
                            x: last_x,
                            y: last_y,
                            mouse_click: string_btn,
                            mouse_is_press: false,
                            key: string_key,
                            key_is_press,
                        },
                    );
                }

                /* key release */
                EventType::KeyRelease(key) => {
                    key_is_press = false;
                    last_key = *key;
                    if is_record {
                        mouse_map.push(MouseMap {
                            x: last_x,
                            y: last_y,
                            mouse_click: Some(last_btn),
                            mouse_is_press,
                            key: Some(last_key),
                            key_is_press,
                        });
                    }

                    modifier.clear();
                }
                _ => (),
            };
        };

        if let Err(error) = rdev::listen(macro_loop) {
            println!("Error: {:?}", error)
        }
    }

    pub fn set_speed(&self, speed: u64) {
        let mut speed_ref = self.speed.lock().unwrap();
        *speed_ref = speed;
    }
}
