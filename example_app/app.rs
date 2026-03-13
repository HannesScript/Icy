use Icy::component::ComponentNode;

use crate::counter::counter_view;

pub fn app_main(counter_value: i32) -> ComponentNode {
    ComponentNode::root(vec![
        ComponentNode::h1("Counter App"),
        counter_view(counter_value),
    ])
}
