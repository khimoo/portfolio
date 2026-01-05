use yew::prelude::*;
use crate::home::types::{NodeId, Position, NodeContent};
use crate::styles::Theme;

#[derive(Properties, PartialEq)]
pub struct NodeComponentProps {
    pub id: NodeId,
    pub pos: Position,
    pub radius: i32,
    pub content: NodeContent,
    pub on_mouse_down: Callback<MouseEvent>,
    pub importance: Option<u8>,
    pub inbound_count: usize,
}

#[function_component(NodeComponent)]
pub fn node_component(props: &NodeComponentProps) -> Html {
    let dynamic_radius = calculate_dynamic_radius(
        props.radius, 
        props.importance, 
        props.inbound_count
    );

    let content_container_style = match &props.content {
        NodeContent::Author { .. } => {
            "width: 80%; height: 80%; object-fit: contain; overflow: hidden; pointer-events: none;"
        }
        _ => {
            "max-width: 80%; max-height: 80%; overflow: hidden; pointer-events: none;"
        }
    };

    let node_style = format!(
        "position: absolute; \
         width: {}px; \
         height: {}px; \
         background-color: {}; \
         border-radius: 50%; \
         transform: translate(-50%, -50%); \
         left: {}px; \
         top: {}px; \
         box-shadow: 0 4px 8px {}; \
         z-index: 10; \
         display: flex; \
         justify-content: center; \
         align-items: center; \
         cursor: pointer; \
         transition: {}; \
         user-select: none;",
        2 * dynamic_radius,
        2 * dynamic_radius,
        Theme::NODE_DEFAULT,
        props.pos.x,
        props.pos.y,
        Theme::NODE_BORDER,
        Theme::TRANSITION_FAST
    );

    html! {
        <div
            key={props.id.0.to_string()}
            onmousedown={props.on_mouse_down.clone()}
            style={node_style}
        >
            <div style={content_container_style}>
                {props.content.render_content()}
            </div>
        </div>
    }
}

/// Calculate dynamic radius based on importance and inbound links
fn calculate_dynamic_radius(base_radius: i32, importance: Option<u8>, inbound_count: usize) -> i32 {
    let mut size = base_radius;

    // Importance-based size adjustment (1-5 scale)
    if let Some(imp) = importance {
        let importance_bonus = match imp {
            1 => -10,
            2 => 0,
            3 => 10,
            4 => 20,
            5 => 40, // Author node
            _ => 0,
        };
        size += importance_bonus;
    }

    // Inbound links-based size adjustment
    let popularity_bonus = (inbound_count as f32).sqrt() as i32 * 3;
    size + popularity_bonus
}