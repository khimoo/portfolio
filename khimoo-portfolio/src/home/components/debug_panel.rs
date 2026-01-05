use yew::prelude::*;
use crate::home::types::ForceSettings;

#[derive(Properties, PartialEq)]
pub struct DebugPanelProps {
    pub force_settings: ForceSettings,
    pub on_repulsion_strength_change: Callback<Event>,
    pub on_repulsion_distance_change: Callback<Event>,
    pub on_author_repulsion_distance_change: Callback<Event>,
    pub on_link_strength_change: Callback<Event>,
    pub on_center_strength_change: Callback<Event>,
    pub on_center_damping_change: Callback<Event>,
}

#[function_component(DebugPanel)]
pub fn debug_panel(props: &DebugPanelProps) -> Html {
    // Only show in debug builds
    if !cfg!(debug_assertions) {
        return html! {};
    }

    let panel_style = "
        position: absolute; 
        top: 20px; 
        right: 20px; 
        background: rgba(0,0,0,0.8); 
        color: white; 
        padding: 20px; 
        border-radius: 10px; 
        z-index: 100;
        max-width: 300px;
    ";

    html! {
        <div style={panel_style}>
            <h3 style="margin: 0 0 15px 0;">{"力の設定"}</h3>
            
            <DebugSlider
                label="反発力の強さ"
                value={props.force_settings.repulsion_strength as i32}
                min="0"
                max="200000"
                step="1000"
                onchange={props.on_repulsion_strength_change.clone()}
            />
            
            <DebugSlider
                label="反発力の最小距離"
                value={props.force_settings.repulsion_min_distance as i32}
                min="0"
                max="1000"
                step="5"
                onchange={props.on_repulsion_distance_change.clone()}
            />
            
            <DebugSlider
                label="作者ノード反発距離"
                value={props.force_settings.author_repulsion_min_distance as i32}
                min="50"
                max="500"
                step="10"
                onchange={props.on_author_repulsion_distance_change.clone()}
            />
            
            <DebugSlider
                label="中心力の強さ"
                value={props.force_settings.center_strength as i32}
                min="0"
                max="10000"
                step="1"
                onchange={props.on_center_strength_change.clone()}
            />
            
            <DebugSlider
                label="中心減衰"
                value={props.force_settings.center_damping as i32}
                min="0"
                max="50"
                step="1"
                onchange={props.on_center_damping_change.clone()}
            />
            
            <DebugSlider
                label="リンク力の強さ"
                value={props.force_settings.link_strength as i32}
                min="0"
                max="50000"
                step="100"
                onchange={props.on_link_strength_change.clone()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DebugSliderProps {
    pub label: String,
    pub value: i32,
    pub min: String,
    pub max: String,
    pub step: String,
    pub onchange: Callback<Event>,
}

#[function_component(DebugSlider)]
fn debug_slider(props: &DebugSliderProps) -> Html {
    html! {
        <div style="margin-bottom: 15px;">
            <label>{format!("{}: {}", props.label, props.value)}</label><br/>
            <input
                type="range"
                min={props.min.clone()}
                max={props.max.clone()}
                step={props.step.clone()}
                value={props.value.to_string()}
                onchange={props.onchange.clone()}
                style="width: 200px;"
            />
        </div>
    }
}