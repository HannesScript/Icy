use Icy::App::*;

use crate::app::app_main;

fn main() {
    let app: IcyApp = create_icy_app();
    let window: IcyWindow = IcyWindow {
        height: 540,
        width: 960,
    };

    app.mount_main_component();

    app.mount_to_window(&window);

    app.show_window();
}
