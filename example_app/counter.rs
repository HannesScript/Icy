use Icy::component::ComponentNode;

pub fn counter_view(counter_value: i32) -> ComponentNode {
    ComponentNode::root(vec![
        ComponentNode::paragraph(format!(
            "Counter: {} (Up/+ increment, Down/- decrement, Esc quit)",
            counter_value
        )),
        ComponentNode::button("Increment [Up or +]"),
        ComponentNode::button("Decrement [Down or -]"),
    ])
}
