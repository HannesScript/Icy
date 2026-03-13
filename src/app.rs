use crate::component::ComponentNode;
use crate::events::{AppEvent, EventLoop, EventResult};
use crate::renderer::{RenderFrame, Renderer};
#[cfg(feature = "sdl-backend")]
use std::path::Path;
#[cfg(feature = "sdl-backend")]
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IcyWindow {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl IcyWindow {
    pub fn new(width: u32, height: u32, title: impl Into<String>) -> Self {
        Self {
            width,
            height,
            title: title.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Created,
    Mounted,
    Running,
    Closed,
}

#[derive(Debug)]
pub struct IcyApp {
    state: AppState,
    window: Option<IcyWindow>,
    root_component: Option<ComponentNode>,
    invalidated: bool,
}

pub fn create_icy_app() -> IcyApp {
    IcyApp::new()
}

impl IcyApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Created,
            window: None,
            root_component: None,
            invalidated: true,
        }
    }

    pub fn mount_main_component(&mut self, root: ComponentNode) {
        self.root_component = Some(root);
        self.invalidated = true;
    }

    pub fn mount_to_window(&mut self, window: IcyWindow) {
        self.window = Some(window);
        self.state = AppState::Mounted;
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn window(&self) -> Option<&IcyWindow> {
        self.window.as_ref()
    }

    pub fn root_component(&self) -> Option<&ComponentNode> {
        self.root_component.as_ref()
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }

    pub fn show_window(&mut self) {
        self.show_window_with_event_handler(|_, _| {});
    }

    pub fn show_window_with_event_handler<F>(&mut self, mut on_event: F)
    where
        F: FnMut(&AppEvent, &mut IcyApp),
    {
        let Some(window) = self.window.clone() else {
            return;
        };

        self.state = AppState::Running;

        let mut renderer = Renderer::new();

        #[cfg(feature = "sdl-backend")]
        {
            let Ok(sdl_context) = sdl3::init() else {
                self.state = AppState::Closed;
                return;
            };
            let Ok(video_subsystem) = sdl_context.video() else {
                self.state = AppState::Closed;
                return;
            };
            let Ok(mut window_handle) = video_subsystem
                .window(&window.title, window.width, window.height)
                .position_centered()
                .build()
            else {
                self.state = AppState::Closed;
                return;
            };

            let _ = window_handle.show();
            let _ = window_handle.raise();

            let mut canvas = window_handle.into_canvas();

            let ttf_context = sdl3::ttf::init().ok();
            let ttf_font = ttf_context.as_ref().and_then(|ctx| load_default_font(ctx));

            let Ok(mut event_pump) = sdl_context.event_pump() else {
                self.state = AppState::Closed;
                return;
            };

            let mut loop_driver = EventLoop::new();
            while loop_driver.is_running() {
                let frame = RenderFrame {
                    window_title: window.title.clone(),
                    root: self.root_component.clone(),
                };
                renderer.render_with_canvas(frame, &mut canvas, ttf_font.as_ref());
                self.invalidated = false;

                for event in loop_driver.poll_sdl(&mut event_pump) {
                    if matches!(self.handle_event(event.clone()), EventResult::ExitRequested) {
                        loop_driver.stop();
                        break;
                    }

                    on_event(&event, self);
                }

                std::thread::sleep(Duration::from_millis(16));
            }
        }

        #[cfg(not(feature = "sdl-backend"))]
        {
            if self.invalidated {
                let frame = RenderFrame {
                    window_title: window.title.clone(),
                    root: self.root_component.clone(),
                };
                renderer.render(frame);
                self.invalidated = false;
            }

            let mut loop_driver = EventLoop::new();
            for event in loop_driver.poll_headless() {
                if matches!(self.handle_event(event.clone()), EventResult::ExitRequested) {
                    break;
                }

                on_event(&event, self);
            }
        }

        self.state = AppState::Closed;
    }

    pub fn handle_event(&mut self, event: AppEvent) -> EventResult {
        match event {
            AppEvent::QuitRequested | AppEvent::WindowCloseRequested => {
                self.state = AppState::Closed;
                EventResult::ExitRequested
            }
            AppEvent::Tick
            | AppEvent::IncrementRequested
            | AppEvent::DecrementRequested
            | AppEvent::MouseClick { .. } => EventResult::Continue,
        }
    }
}

impl Default for IcyApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "sdl-backend")]
fn load_default_font(ttf: &sdl3::ttf::Sdl3TtfContext) -> Option<sdl3::ttf::Font<'static>> {
    let candidates = [
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/google-noto/NotoSans-Regular.ttf",
    ];

    for candidate in candidates {
        if Path::new(candidate).exists() {
            if let Ok(font) = ttf.load_font(candidate, 18.0) {
                return Some(font);
            }
        }
    }

    None
}
