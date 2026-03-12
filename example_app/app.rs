use Icy::Component::*;

use crate::counter::counter;

pub fn app_main() {
    return Component {
        children: vec![
            h1 {
                content: Text { text: "Counter App" },
            },
            counter(),
        ],
    };
}
