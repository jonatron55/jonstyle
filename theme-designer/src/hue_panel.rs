use leptos::prelude::*;

use crate::color_wheel::ColorWheel;
use crate::slider::ValueSlider;

#[component]
pub fn HuePanel(
    cool_start: RwSignal<f64, LocalStorage>,
    cool_end: RwSignal<f64, LocalStorage>,
    warm_start: RwSignal<f64, LocalStorage>,
    warm_end: RwSignal<f64, LocalStorage>,
    offset: RwSignal<f64, LocalStorage>,
) -> impl IntoView {
    view! {
        <div class="panel">
            <h1 class="caption">"Hues"</h1>
            <div class="split">
                <div>
                    <ValueSlider
                        id="offset"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=offset
                        label="Offset"
                    />

                    <ValueSlider
                        id="cool-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_start
                        label="Cool Start"
                    />
                    <ValueSlider
                        id="cool-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_end
                        label="Cool End"
                    />

                    <ValueSlider
                        id="warm-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_start
                        label="Warm Start"
                    />
                    <ValueSlider
                        id="warm-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_end
                        label="Warm End"
                    />
                </div>
                <ColorWheel
                    cool_start=cool_start
                    cool_end=cool_end
                    warm_start=warm_start
                    warm_end=warm_end
                    offset=offset
                />
            </div>
        </div>
    }
}
