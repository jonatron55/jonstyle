use leptos::prelude::*;
use themelib::theme::ThemeBuilder;

use crate::slider::RangeSlider;

#[component]
pub fn SatPanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let muted_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.muted_sat_range)
    });
    let base_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.base_sat_range)
    });
    let intense_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.intense_sat_range)
    });

    view! {
        <div class="panel">
            <h1 class="caption">"Saturation"</h1>
            <div>
                <div>
                    <RangeSlider
                        label="Muted Range"
                        id="muted-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=muted_range
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.muted_sat_range = new_value;
                                    });
                            }
                        })
                    />
                    <RangeSlider
                        label="Base Range"
                        id="base-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=base_range
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.base_sat_range = new_value;
                                    });
                            }
                        })
                    />
                    <RangeSlider
                        label="Intense Range"
                        id="intense-range"
                        min=0.0
                        max=100.0
                        step=1.0
                        value=intense_range
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.intense_sat_range = new_value;
                                    });
                            }
                        })
                    />
                </div>
            </div>
        </div>
    }
}
