#![allow(non_snake_case)]

pub mod app;
pub mod component;
pub mod events;
pub mod renderer;
pub mod state_management;

pub use app as App;
pub use component as Component;
pub use events as Events;
pub use renderer as Renderer;
pub use state_management as StateManagement;

pub use app::{IcyApp, IcyWindow, create_icy_app};
pub use component::{ComponentNode, ElementType, TextNode};
pub use state_management::{IcyState, new_state};
