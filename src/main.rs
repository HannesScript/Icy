use Icy::{IcyWindow, create_icy_app};

fn main() {
    let mut app = create_icy_app();
    let window = IcyWindow::new(960, 540, "Icy");

    app.mount_to_window(window);
    app.show_window();
}
