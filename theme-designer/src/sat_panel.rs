use leptos::prelude::*;

use crate::slider::RangeSlider;

#[component]
pub fn SatPanel(
    muted_range: RwSignal<(f64, f64), LocalStorage>,
    base_range: RwSignal<(f64, f64), LocalStorage>,
    intense_range: RwSignal<(f64, f64), LocalStorage>,
) -> impl IntoView {
    view! {
        <div class="panel">
            <h1 class="caption">"Saturation"</h1>
            <div class="split-panel">
                <div>
                    <RangeSlider
                        id="muted-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=muted_range
                        label="Muted Range"
                    />
                    <RangeSlider
                        id="base-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=base_range
                        label="Base Range"
                    />
                    <RangeSlider
                        id="intense-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=intense_range
                        label="Intense Range"
                    />
                </div>
            </div>
        </div>
    }
}
