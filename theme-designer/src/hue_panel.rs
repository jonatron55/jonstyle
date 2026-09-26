use leptos::prelude::*;
use themelib::theme::ThemeBuilder;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::color_wheel::ColorWheel;
use crate::slider::ValueSlider;

#[component]
pub fn HuePanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let (symmetric, set_symmetric) = signal_local(false);

    let cool_start = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.cool_range.0)
    });
    let cool_end = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.cool_range.1)
    });
    let warm_start = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.warm_range.0)
    });
    let warm_end = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.warm_range.1)
    });
    let offset = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.offset)
    });

    view! {
        <div class="panel">
            <h1 class="caption">"Hues"</h1>
            <div class="split-2">
                <div style="display: grid; grid-template-columns: 1fr auto">
                    <ValueSlider
                        label="Offset"
                        id="offset"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=offset
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder.update(|b| b.offset = new_value);
                            }
                        })
                    />
                    <div></div>
                    <ValueSlider
                        label="Cool Start"
                        id="cool-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_start
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.cool_range = (new_value, b.cool_range.1);
                                        if symmetric.get() {
                                            let (new_warm_start, new_warm_end) = symmetric_hue_range(
                                                new_value,
                                                b.cool_range.1,
                                                (b.cool_range.0 - b.cool_range.1).signum(),
                                            );
                                            b.warm_range = (new_warm_start, new_warm_end);
                                        }
                                    })
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        <button on:click={
                            let builder = builder.clone();
                            move |_| {
                                builder
                                    .update(|b| {
                                        let (cool_start, cool_end) = b.cool_range;
                                        b.cool_range = (cool_end, cool_start);
                                    });
                            }
                        }>"Swap"</button>
                    </div>

                    <ValueSlider
                        label="Cool End"
                        id="cool-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_end
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.cool_range = (b.cool_range.0, new_value);
                                        if symmetric.get() {
                                            let (new_warm_start, new_warm_end) = symmetric_hue_range(
                                                b.cool_range.0,
                                                new_value,
                                                (b.cool_range.0 - b.cool_range.1).signum(),
                                            );
                                            b.warm_range = (new_warm_start, new_warm_end);
                                        }
                                    })
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        {move || { format!("{}°", (cool_end.get() - cool_start.get())) }}
                    </div>

                    <ValueSlider
                        label="Warm Start"
                        id="warm-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_start
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.warm_range = (new_value, b.warm_range.1);
                                        if symmetric.get() {
                                            let (new_cool_start, new_cool_end) = symmetric_hue_range(
                                                new_value,
                                                b.warm_range.1,
                                                (b.cool_range.0 - b.cool_range.1).signum(),
                                            );
                                            b.cool_range = (new_cool_start, new_cool_end);
                                        }
                                    })
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        <button on:click={
                            let builder = builder.clone();
                            move |_| {
                                builder
                                    .update(|b| {
                                        let (warm_start, warm_end) = b.warm_range;
                                        b.warm_range = (warm_end, warm_start);
                                    });
                            }
                        }>"Swap"</button>
                    </div>

                    <ValueSlider
                        label="Warm End"
                        id="warm-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_end
                        on_change=Callback::new({
                            let builder = builder.clone();
                            move |new_value| {
                                builder
                                    .update(|b| {
                                        b.warm_range = (b.warm_range.0, new_value);
                                        if symmetric.get() {
                                            let (new_cool_start, new_cool_end) = symmetric_hue_range(
                                                b.warm_range.0,
                                                new_value,
                                                (b.cool_range.0 - b.cool_range.1).signum(),
                                            );
                                            b.cool_range = (new_cool_start, new_cool_end);
                                        }
                                    });
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        {move || { format!("{}°", (warm_end.get() - warm_start.get())) }}
                    </div>

                    <div style="display: flex; justify-content: center; align-items: center; grid-column: span 2">
                        <input
                            type="checkbox"
                            id="symmetric"
                            class="toggle-switch"
                            checked=symmetric
                            on:change=move |ev| {
                                let is_checked = ev
                                    .target()
                                    .unwrap()
                                    .unchecked_into::<HtmlInputElement>()
                                    .checked();
                                set_symmetric.set(is_checked);
                            }
                        />
                        <label for="symmetric">"Symmetric"</label>
                    </div>
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

fn symmetric_hue_range(start: f64, end: f64, symmetric_sign: f64) -> (f64, f64) {
    let range = end - start;
    let half_range = range / 2.0;

    let center = start + half_range + 180.0;
    let sign = (start - end).signum();

    let sign = sign * symmetric_sign;

    let symmetric_start = center - half_range * sign;
    let symmetric_end = center + half_range * sign;

    if symmetric_start > 360.0 || symmetric_end > 360.0 {
        (symmetric_start - 360.0, symmetric_end - 360.0)
    } else if symmetric_start < -360.0 || symmetric_end < -360.0 {
        (symmetric_start + 360.0, symmetric_end + 360.0)
    } else {
        (symmetric_start, symmetric_end)
    }
}
