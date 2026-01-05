use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::{use_effect_update_with_deps, use_interval, use_window_scroll, UseMeasureState};
use yew_router::prelude::*;

use super::{NodeComponent, DebugPanel, LoadingView, ErrorView, NodeRegistryBuilder};
use crate::home::data_loader::use_articles_data;
use crate::home::physics_sim::{PhysicsWorld, Viewport};
use crate::home::types::*;
use crate::home::routes::Route;
use crate::styles::Theme;

#[derive(Properties, PartialEq)]
pub struct NodeGraphContainerProps {
    pub container_ref: NodeRef,
    pub container_measure: UseMeasureState,
    pub container_bound: ContainerBound,
}

#[function_component(NodeGraphContainer)]
pub fn node_graph_container(props: &NodeGraphContainerProps) -> Html {
    // State management
    let dragged_node_id = use_state(|| None::<NodeId>);
    let viewport = use_state(Viewport::default);
    let force_settings = use_state(ForceSettings::default);
    let drag_start_pos = use_state(|| None::<(i32, i32)>);
    let is_dragging = use_state(|| false);
    let rerender = use_state(|| ());

    // Data loading
    let (articles_data, loading, error) = use_articles_data();

    // Physics world and node registry
    let node_registry = use_state(|| Rc::new(RefCell::new(NodeRegistry::new())));
    let node_slug_mapping = use_state(|| HashMap::<NodeId, String>::new());
    let physics_world = use_state(|| {
        let empty_registry = Rc::new(RefCell::new(NodeRegistry::new()));
        let default_bound = ContainerBound::default();
        Rc::new(RefCell::new(PhysicsWorld::new(
            empty_registry,
            &viewport,
            *force_settings,
            default_bound,
        )))
    });

    // Initialize physics world when articles data is loaded
    let initialized = use_state(|| false);
    if let Some(data) = articles_data.as_ref() {
        if !*initialized {
            let (new_registry, slug_mapping) =
                NodeRegistryBuilder::build_from_articles(data, &props.container_bound);
            let registry_rc = Rc::new(RefCell::new(new_registry));
            node_registry.set(Rc::clone(&registry_rc));
            node_slug_mapping.set(slug_mapping);

            let new_physics_world = PhysicsWorld::new(
                registry_rc,
                &viewport,
                *force_settings,
                props.container_bound.clone(),
            );
            physics_world.set(Rc::new(RefCell::new(new_physics_world)));
            initialized.set(true);
        }
    }

    // Update physics world when settings change
    use_physics_world_updates(&physics_world, &force_settings, &props.container_bound);

    // Mouse event handlers
    let mouse_handlers = use_mouse_handlers(
        &dragged_node_id,
        &drag_start_pos,
        &is_dragging,
        &physics_world,
        &viewport,
        &node_slug_mapping,
    );

    // Animation loop
    use_animation_loop(&physics_world, &viewport, &rerender);

    // Force settings callbacks
    let force_callbacks = use_force_callbacks(&force_settings);

    // Early returns for loading/error states
    if *loading {
        return html! { <LoadingView /> };
    }

    if let Some(err) = error.as_ref() {
        return html! { <ErrorView error_message={err.to_string()} /> };
    }

    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div
                style={format!("display: flex; width: 100%; height: 100%; background: {};", Theme::BACKGROUND_PRIMARY)}
                onmousemove={mouse_handlers.on_mouse_move}
                onmouseup={mouse_handlers.on_mouse_up}
                ref={props.container_ref.clone()}
            >
                <AuthorInfoOverlay container_bound={props.container_bound.clone()} />
                
                <DebugPanel
                    force_settings={*force_settings}
                    on_repulsion_strength_change={force_callbacks.on_repulsion_strength_change}
                    on_repulsion_distance_change={force_callbacks.on_repulsion_distance_change}
                    on_author_repulsion_distance_change={force_callbacks.on_author_repulsion_distance_change}
                    on_link_strength_change={force_callbacks.on_link_strength_change}
                    on_center_strength_change={force_callbacks.on_center_strength_change}
                    on_center_damping_change={force_callbacks.on_center_damping_change}
                />

                <EdgeRenderer node_registry={node_registry.clone()} />
                <NodeRenderer 
                    node_registry={node_registry.clone()}
                    on_mouse_down={mouse_handlers.on_node_mouse_down}
                />
            </div>
        </>
    }
}

// Helper components and hooks
#[derive(Properties, PartialEq)]
struct AuthorInfoOverlayProps {
    container_bound: ContainerBound,
}

#[function_component(AuthorInfoOverlay)]
fn author_info_overlay(props: &AuthorInfoOverlayProps) -> Html {
    let overlay_style = format!(
        "position: absolute; \
         left: 50%; \
         top: {}px; \
         transform: translateX(-50%); \
         border-radius: 20px; \
         z-index: 50; \
         text-align: center; \
         font-size: 23px; \
         color: white; \
         backdrop-filter: blur(10px); \
         pointer-events: none;",
        (props.container_bound.height / 2.0 + 100.0) as i32
    );

    html! {
        <div style={overlay_style}>
            <span style="display:flex; flex-direction: column; margin-bottom:12px">
                <span style="font-size: 32px; font-weight: bold;">{"日比野 文"}</span>
                <span style="font-size: 16px; font-weight: bold;">{"Bun Hibino"}</span>
            </span>
            <div style="white-space: nowrap; margin: 10px 0; line-height: 1.5;">
                {"筑波大学 理工情報生命学術院 数理物質科学研究群 数学学位プログラム"}<br/>
                {"専門：幾何学/連続体理論"}<br/>
                {"Rust, neovim, NixOS, HoTTにも興味があります！"}
            </div>
        </div>
    }
}

#[derive(Properties)]
struct EdgeRendererProps {
    node_registry: UseStateHandle<Rc<RefCell<NodeRegistry>>>,
}

impl PartialEq for EdgeRendererProps {
    fn eq(&self, _other: &Self) -> bool {
        // UseStateHandle comparison is not meaningful for props
        false
    }
}

#[function_component(EdgeRenderer)]
fn edge_renderer(props: &EdgeRendererProps) -> Html {
    let reg = props.node_registry.borrow();
    html! {
        <svg style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; z-index: 1; pointer-events: none;">
            {
                reg.iter_edges().filter_map(|(a, b)| {
                    let p1 = reg.positions.get(a)?;
                    let p2 = reg.positions.get(b)?;
                    Some(html! {
                        <line
                            x1={format!("{:.2}", p1.x)}
                            y1={format!("{:.2}", p1.y)}
                            x2={format!("{:.2}", p2.x)}
                            y2={format!("{:.2}", p2.y)}
                            stroke="#8a8a8a"
                            stroke-width="1.5"
                        />
                    })
                }).collect::<Html>()
            }
        </svg>
    }
}

#[derive(Properties)]
struct NodeRendererProps {
    node_registry: UseStateHandle<Rc<RefCell<NodeRegistry>>>,
    on_mouse_down: Callback<(NodeId, MouseEvent)>,
}

impl PartialEq for NodeRendererProps {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the callback, UseStateHandle comparison is not meaningful
        self.on_mouse_down == other.on_mouse_down
    }
}

#[function_component(NodeRenderer)]
fn node_renderer(props: &NodeRendererProps) -> Html {
    let registry = props.node_registry.borrow();
    html! {
        <>
            {
                registry.iter().map(|(id, pos, radius, content)| {
                    let importance = registry.get_node_importance(*id);
                    let inbound_count = registry.get_node_inbound_count(*id);

                    let on_mouse_down = {
                        let on_mouse_down = props.on_mouse_down.clone();
                        let id = *id;
                        Callback::from(move |e: MouseEvent| {
                            e.stop_propagation();
                            on_mouse_down.emit((id, e));
                        })
                    };

                    html! {
                        <NodeComponent
                            key={id.0}
                            id={*id}
                            pos={*pos}
                            radius={*radius}
                            content={content.clone()}
                            {importance}
                            {inbound_count}
                            {on_mouse_down}
                        />
                    }
                }).collect::<Html>()
            }
        </>
    }
}

// Custom hooks for better organization
struct MouseHandlers {
    on_mouse_move: Callback<MouseEvent>,
    on_mouse_up: Callback<MouseEvent>,
    on_node_mouse_down: Callback<(NodeId, MouseEvent)>,
}

#[hook]
fn use_mouse_handlers(
    dragged_node_id: &UseStateHandle<Option<NodeId>>,
    drag_start_pos: &UseStateHandle<Option<(i32, i32)>>,
    is_dragging: &UseStateHandle<bool>,
    physics_world: &UseStateHandle<Rc<RefCell<PhysicsWorld>>>,
    viewport: &UseStateHandle<Viewport>,
    node_slug_mapping: &UseStateHandle<HashMap<NodeId, String>>,
) -> MouseHandlers {
    let scroll = use_window_scroll();
    let navigator = use_navigator().unwrap();

    let on_mouse_move = {
        let dragged_node_id = dragged_node_id.clone();
        let physics_world = physics_world.clone();
        let viewport = viewport.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();
        
        Callback::from(move |e: MouseEvent| {
            if let Some(id) = *dragged_node_id {
                if let Some((start_x, start_y)) = *drag_start_pos {
                    let dx = e.client_x() - start_x;
                    let dy = e.client_y() - start_y;
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();

                    if distance > 5.0 && !*is_dragging {
                        is_dragging.set(true);
                        physics_world.borrow_mut().set_node_kinematic(id);
                    }

                    if *is_dragging {
                        let mut world = physics_world.borrow_mut();
                        let screen_pos = Position {
                            x: (e.client_x() + scroll.0 as i32) as f32,
                            y: (e.client_y() + scroll.1 as i32) as f32,
                        };
                        world.set_node_position(id, &screen_pos, &viewport);
                    }
                }
            }
        })
    };

    let on_node_click = {
        let navigator = navigator.clone();
        let node_slug_mapping = node_slug_mapping.clone();
        Callback::from(move |node_id: NodeId| {
            if let Some(slug) = node_slug_mapping.get(&node_id) {
                if slug == "author" {
                    return;
                }
                let route = Route::ArticleShow { slug: slug.clone() };
                navigator.push(&route);
            }
        })
    };

    let on_node_mouse_down = {
        let dragged_node_id = dragged_node_id.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();
        
        Callback::from(move |(id, e): (NodeId, MouseEvent)| {
            drag_start_pos.set(Some((e.client_x(), e.client_y())));
            is_dragging.set(false);
            dragged_node_id.set(Some(id));
        })
    };

    let on_mouse_up = {
        let dragged_node_id = dragged_node_id.clone();
        let physics_world = physics_world.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();
        let on_node_click = on_node_click.clone();
        
        Callback::from(move |_: MouseEvent| {
            if let Some(id) = *dragged_node_id {
                if *is_dragging {
                    physics_world.borrow_mut().set_node_dynamic(id);
                } else {
                    on_node_click.emit(id);
                }
            }
            dragged_node_id.set(None);
            drag_start_pos.set(None);
            is_dragging.set(false);
        })
    };

    MouseHandlers {
        on_mouse_move,
        on_mouse_up,
        on_node_mouse_down,
    }
}

struct ForceCallbacks {
    on_repulsion_strength_change: Callback<Event>,
    on_repulsion_distance_change: Callback<Event>,
    on_author_repulsion_distance_change: Callback<Event>,
    on_link_strength_change: Callback<Event>,
    on_center_strength_change: Callback<Event>,
    on_center_damping_change: Callback<Event>,
}

#[hook]
fn use_force_callbacks(force_settings: &UseStateHandle<ForceSettings>) -> ForceCallbacks {
    let create_callback = |field: &'static str| {
        let force_settings = force_settings.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0);
            let mut settings = *force_settings;
            
            match field {
                "repulsion_strength" => settings.repulsion_strength = value,
                "repulsion_min_distance" => settings.repulsion_min_distance = value,
                "author_repulsion_min_distance" => settings.author_repulsion_min_distance = value,
                "link_strength" => settings.link_strength = value,
                "center_strength" => settings.center_strength = value,
                "center_damping" => settings.center_damping = value,
                _ => {}
            }
            
            force_settings.set(settings);
        })
    };

    ForceCallbacks {
        on_repulsion_strength_change: create_callback("repulsion_strength"),
        on_repulsion_distance_change: create_callback("repulsion_min_distance"),
        on_author_repulsion_distance_change: create_callback("author_repulsion_min_distance"),
        on_link_strength_change: create_callback("link_strength"),
        on_center_strength_change: create_callback("center_strength"),
        on_center_damping_change: create_callback("center_damping"),
    }
}

#[hook]
fn use_physics_world_updates(
    physics_world: &UseStateHandle<Rc<RefCell<PhysicsWorld>>>,
    force_settings: &UseStateHandle<ForceSettings>,
    container_bound: &ContainerBound,
) {
    // Update force settings
    {
        let physics_world = physics_world.clone();
        let force_settings_clone = force_settings.clone();
        use_effect_update_with_deps(
            move |_| {
                physics_world.borrow_mut().update_force_settings(*force_settings_clone);
                || {}
            },
            force_settings.clone(),
        );
    }

    // Update container bounds
    {
        let physics_world = physics_world.clone();
        use_effect_update_with_deps(
            move |container_bound| {
                physics_world.borrow_mut().update_container_bound(container_bound.clone());
                || {}
            },
            container_bound.clone(),
        );
    }
}

#[hook]
fn use_animation_loop(
    physics_world: &UseStateHandle<Rc<RefCell<PhysicsWorld>>>,
    viewport: &UseStateHandle<Viewport>,
    rerender: &UseStateHandle<()>,
) {
    let physics_world = physics_world.clone();
    let viewport = viewport.clone();
    let rerender = rerender.clone();
    
    use_interval(
        move || {
            let mut world = physics_world.borrow_mut();
            world.step(&viewport);
            rerender.set(());
        },
        8, // ~120fps
    );
}