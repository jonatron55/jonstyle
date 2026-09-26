use crate::{
    lum_plot::LumPlot,
    slider::{RangeSlider, ValueSlider},
};
use leptos::prelude::*;
use themelib::theme::ThemeBuilder;

#[component]
pub fn LumPanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let lum_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.lum_range)
    });
    let lum_alpha = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.lum_alpha)
    });
    let lum_gamma = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.lum_gamma)
    });

    view! {
        <div class="panel">
            <h1 class="caption">"Luminance"</h1>
            <div class="split-2">
                <div>
                    <RangeSlider
                        label="Range"
                        id="lum-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=lum_range
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder.update(|b| b.lum_range = new_value);
                            }
                        })
                    />

                    <ValueSlider
                        label="α"
                        id="lum-alpha"
                        min=0.33
                        max=3.0
                        step=0.01
                        value=lum_alpha
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder.update(|b| b.lum_alpha = new_value);
                            }
                        })
                    />

                    <ValueSlider
                        label="γ"
                        id="lum-gamma"
                        min=0.5
                        max=2.0
                        step=0.01
                        value=lum_gamma
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder.update(|b| b.lum_gamma = new_value);
                            }
                        })
                    />

                </div>
                <LumPlot lum_range=lum_range lum_alpha=lum_alpha lum_gamma=lum_gamma />
            </div>
        </div>
    }
}
