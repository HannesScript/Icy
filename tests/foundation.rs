use std::cell::RefCell;
use std::rc::Rc;

use Icy::app::AppState;
use Icy::component::ComponentNode;
use Icy::events::{AppEvent, EventResult};
use Icy::state_management::new_state;
use Icy::{IcyWindow, create_icy_app};

#[test]
fn state_notifies_subscribers_on_set_and_update() {
    let mut state = new_state(0);
    let observed = Rc::new(RefCell::new(Vec::new()));

    let observed_for_subscriber = Rc::clone(&observed);
    state.subscribe(move |value| {
        observed_for_subscriber.borrow_mut().push(*value);
    });

    state.set(5);
    state.update(|value| *value += 2);

    assert_eq!(*state.get(), 7);
    assert_eq!(observed.borrow().as_slice(), &[5, 7]);
}

#[test]
fn component_validation_rejects_empty_non_root_nodes() {
    let malformed = ComponentNode::button("").with_children(vec![]);
    assert!(malformed.validate().is_ok());

    let invalid = Icy::component::ComponentNode {
        element: Icy::component::ElementType::Paragraph,
        text: None,
        children: vec![],
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn app_lifecycle_transitions_and_window_mounting_work() {
    let mut app = create_icy_app();
    assert_eq!(app.state(), &AppState::Created);

    app.mount_main_component(ComponentNode::root(vec![ComponentNode::h1("Icy")]));
    app.mount_to_window(IcyWindow::new(800, 600, "Test"));

    assert_eq!(app.state(), &AppState::Mounted);
    assert_eq!(app.window().map(|w| w.width), Some(800));

    app.show_window();
    assert_eq!(app.state(), &AppState::Closed);
}

#[test]
fn quit_events_request_exit() {
    let mut app = create_icy_app();

    assert_eq!(app.handle_event(AppEvent::Tick), EventResult::Continue);
    assert_eq!(
        app.handle_event(AppEvent::QuitRequested),
        EventResult::ExitRequested
    );
}
