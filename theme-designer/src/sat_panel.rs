use leptos::prelude::*;
use themelib::theme::ThemeBuilder;

use crate::slider::RangeSlider;

#[component]
pub fn SatPanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let muted_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.sat.muted_range)
    });
    let base_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.sat.base_range)
    });
    let intense_range = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.sat.intense_range)
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
                                        b.sat.muted_range = new_value;
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
                                        b.sat.base_range = new_value;
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
                                        b.sat.intense_range = new_value;
                                    });
                            }
                        })
                    />
                </div>
            </div>
        </div>
    }
}
