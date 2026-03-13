#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    Tick,
    QuitRequested,
    WindowCloseRequested,
    IncrementRequested,
    DecrementRequested,
    MouseClick { x: i32, y: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventResult {
    Continue,
    ExitRequested,
}

#[derive(Debug, Default)]
pub struct EventLoop {
    running: bool,
}

impl EventLoop {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    #[cfg(not(feature = "sdl-backend"))]
    pub fn poll_headless(&mut self) -> Vec<AppEvent> {
        self.running = false;
        vec![AppEvent::Tick, AppEvent::WindowCloseRequested]
    }

    #[cfg(feature = "sdl-backend")]
    pub fn poll_sdl(&mut self, event_pump: &mut sdl3::EventPump) -> Vec<AppEvent> {
        use sdl3::event::{Event, WindowEvent};
        use sdl3::keyboard::Keycode;
        use sdl3::mouse::MouseButton;

        let mut mapped = vec![AppEvent::Tick];
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => mapped.push(AppEvent::QuitRequested),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Equals),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::KpPlus),
                    ..
                } => mapped.push(AppEvent::IncrementRequested),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::KpMinus),
                    ..
                } => mapped.push(AppEvent::DecrementRequested),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => mapped.push(AppEvent::QuitRequested),
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => mapped.push(AppEvent::MouseClick {
                    x: x as i32,
                    y: y as i32,
                }),
                Event::Window {
                    win_event: WindowEvent::CloseRequested,
                    ..
                } => mapped.push(AppEvent::WindowCloseRequested),
                _ => {}
            }
        }

        mapped
    }
}
