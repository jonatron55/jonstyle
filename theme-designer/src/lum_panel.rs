use leptos::prelude::*;

use crate::{
    lum_plot::LumPlot,
    slider::{RangeSlider, ValueSlider},
};

#[component]
pub fn LumPanel(
    lum_range: RwSignal<(f64, f64), LocalStorage>,
    lum_power: RwSignal<f64, LocalStorage>,
    lum_gamma: RwSignal<f64, LocalStorage>,
) -> impl IntoView {
    view! {
        <div class="panel">
            <h1 class="caption">"Luminance"</h1>
            <div class="split-3">
                <div>
                    <RangeSlider
                        id="lum-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=lum_range
                        label="Range"
                    />
                    <ValueSlider
                        id="lum-power"
                        min=0.33
                        max=3.0
                        step=0.01
                        value=lum_power
                        label="Power"
                    />
                    <ValueSlider
                        id="lum-gamma"
                        min=0.5
                        max=2.0
                        step=0.01
                        value=lum_gamma
                        label="Gamma"
                    />
                </div>
                <LumPlot lum_range=lum_range lum_power=lum_power lum_gamma=lum_gamma />
            </div>
        </div>
    }
}
