use Icy::component::{ComponentNode, ElementType};
use Icy::events::AppEvent;
use Icy::{IcyWindow, create_icy_app};

mod app;
mod counter;

use crate::app::app_main;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClickAction {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy)]
struct UiRect {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl UiRect {
    fn contains(&self, px: i32, py: i32) -> bool {
        let right = self.x + self.w as i32;
        let bottom = self.y + self.h as i32;
        px >= self.x && px <= right && py >= self.y && py <= bottom
    }
}

fn button_rects_from_tree(root: &ComponentNode) -> Vec<UiRect> {
    let mut rects = Vec::new();
    let mut y = 24i32;
    collect_button_rects(root, 0, &mut y, &mut rects);
    rects
}

fn collect_button_rects(node: &ComponentNode, depth: usize, y: &mut i32, rects: &mut Vec<UiRect>) {
    let indent = (depth as i32) * 20;
    let x = 24 + indent;
    let width = (760i32 - indent).max(160) as u32;

    match node.element {
        ElementType::Root => {
            if depth > 0 {
                *y += 34;
            }
            for child in &node.children {
                collect_button_rects(child, depth + 1, y, rects);
            }
        }
        ElementType::H1 => {
            *y += 64 + 12;
        }
        ElementType::Paragraph => {
            *y += 44 + 12;
        }
        ElementType::Button => {
            let height = 52u32;
            rects.push(UiRect {
                x,
                y: *y,
                w: width,
                h: height,
            });
            *y += height as i32 + 12;
        }
    }
}

fn action_from_click(root: &ComponentNode, x: i32, y: i32) -> Option<ClickAction> {
    let button_rects = button_rects_from_tree(root);
    let first = button_rects.first()?;
    if first.contains(x, y) {
        return Some(ClickAction::Increment);
    }

    let second = button_rects.get(1)?;
    if second.contains(x, y) {
        return Some(ClickAction::Decrement);
    }

    None
}

fn main() {
    let mut app = create_icy_app();
    let window = IcyWindow::new(960, 540, "Counter App - Icy Example");
    let mut counter = 0i32;

    app.mount_main_component(app_main(counter));

    app.mount_to_window(window);

    app.show_window_with_event_handler(|event, app| match event {
        AppEvent::IncrementRequested => {
            counter += 1;
            app.mount_main_component(app_main(counter));
            println!("Counter incremented to {}", counter);
        }
        AppEvent::DecrementRequested => {
            counter -= 1;
            app.mount_main_component(app_main(counter));
            println!("Counter decremented to {}", counter);
        }
        AppEvent::MouseClick { x, y } => {
            let action = app
                .root_component()
                .and_then(|root| action_from_click(root, *x, *y));

            match action {
                Some(ClickAction::Increment) => {
                    counter += 1;
                    app.mount_main_component(app_main(counter));
                    println!("Counter incremented to {}", counter);
                }
                Some(ClickAction::Decrement) => {
                    counter -= 1;
                    app.mount_main_component(app_main(counter));
                    println!("Counter decremented to {}", counter);
                }
                None => {}
            }
        }
        _ => {}
    });
}
