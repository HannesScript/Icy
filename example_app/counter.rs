use Icy::Component::*;
use Icy::StateManagement::*;


pub fn counter() {
    let counter_state: IcyState = new_state(i32, 0);
    return Component {
        children: vec![
            p {
                content: Text { text: format!("Counter: {}", counter_state.value) }
            },
            button {
                children: vec![
                    Text { text: "Increment" },
                ],
                action: increment_counter(&counter_state), 
            },
            button {
                children: vec![
                    Text { text: "Decrement" },
                ],
                action: decrement_counter(&counter_state), 
            },
        ],
    }
}

fn increment_counter(counter_state: &IcyState) {
    counter_state.value += 1;
}

fn decrement_counter(counter_state: &IcyState) {
    counter_state.value -= 1;
}
