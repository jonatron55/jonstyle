use leptos::prelude::*;

use crate::slider::ValueSlider;

#[component]
pub fn ParameterPanel(
    cool_center: RwSignal<f32, LocalStorage>,
    cool_spread: RwSignal<f32, LocalStorage>,
    warm_center: RwSignal<f32, LocalStorage>,
    warm_spread: RwSignal<f32, LocalStorage>,
    offset: RwSignal<f32, LocalStorage>,
) -> impl IntoView {
    view! {
        <div class="panel">
            <h1 class="caption">"Hues"</h1>
            <div class="split-panel">
                <div>
                    <ValueSlider
                        id="offset"
                        min=-179.0
                        max=180.0
                        step=1.0
                        value=offset
                        label="Offset"
                    />
                    <ValueSlider
                        id="cool-center"
                        min=0.0
                        max=359.0
                        step=1.0
                        value=cool_center
                        label="Cool Center"
                    />
                    <ValueSlider
                        id="cool-spread"
                        min=-180.0
                        max=180.0
                        step=1.0
                        value=cool_spread
                        label="Cool Spread"
                    />
                    <ValueSlider
                        id="warm-center"
                        min=0.0
                        max=359.0
                        step=1.0
                        value=warm_center
                        label="Warm Center"
                    />
                    <ValueSlider
                        id="warm-spread"
                        min=-180.0
                        max=180.0
                        step=1.0
                        value=warm_spread
                        label="Warm Spread"
                    />
                </div>
                <div>// <ColorWheel markers={hues} {offset} />
                </div>
            </div>
        </div>
    }
}
