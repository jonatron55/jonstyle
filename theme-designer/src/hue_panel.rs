use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

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
    let (symmetric, set_symmetric) = signal_local(false);

    view! {
        <div class="panel">
            <h1 class="caption">"Hues"</h1>
            <div class="split-2">
                <div style="display: grid; grid-template-columns: 1fr auto">
                    <ValueSlider
                        id="offset"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=offset
                        label="Offset"
                    />
                    <div></div>
                    <ValueSlider
                        id="cool-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_start
                        label="Cool Start"
                        on_change=Callback::new(move |new_value| {
                            if symmetric.get() {
                                let (new_warm_start, new_warm_end) = symmetric_hue_range(
                                    new_value,
                                    cool_end.get(),
                                    (cool_start.get() - cool_end.get()).signum(),
                                );
                                warm_start.set(new_warm_start);
                                warm_end.set(new_warm_end);
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        <button on:click=move |_| {
                            let cool_start_val = cool_start.get();
                            let cool_end_val = cool_end.get();
                            cool_start.set(cool_end_val);
                            cool_end.set(cool_start_val);
                        }>"Swap"</button>
                    </div>

                    <ValueSlider
                        id="cool-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=cool_end
                        label="Cool End"
                        on_change=Callback::new(move |new_value| {
                            if symmetric.get() {
                                let (new_warm_start, new_warm_end) = symmetric_hue_range(
                                    cool_start.get(),
                                    new_value,
                                    (cool_start.get() - cool_end.get()).signum(),
                                );
                                warm_start.set(new_warm_start);
                                warm_end.set(new_warm_end);
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        {move || { format!("{}°", (cool_end.get() - cool_start.get())) }}
                    </div>

                    <ValueSlider
                        id="warm-start"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_start
                        label="Warm Start"
                        on_change=Callback::new(move |new_value| {
                            if symmetric.get() {
                                let (new_cool_start, new_cool_end) = symmetric_hue_range(
                                    new_value,
                                    warm_end.get(),
                                    (cool_start.get() - cool_end.get()).signum(),
                                );
                                cool_start.set(new_cool_start);
                                cool_end.set(new_cool_end);
                            }
                        })
                    />

                    <div style="display: flex; justify-content: center; align-items: center;">
                        <button on:click=move |_| {
                            let warm_start_val = warm_start.get();
                            let warm_end_val = warm_end.get();
                            warm_start.set(warm_end_val);
                            warm_end.set(warm_start_val);
                        }>"Swap"</button>
                    </div>

                    <ValueSlider
                        id="warm-end"
                        min=-359.0
                        max=359.0
                        step=1.0
                        value=warm_end
                        label="Warm End"
                        on_change=Callback::new(move |new_value| {
                            if symmetric.get() {
                                let (new_cool_start, new_cool_end) = symmetric_hue_range(
                                    warm_start.get(),
                                    new_value,
                                    (cool_start.get() - cool_end.get()).signum(),
                                );
                                cool_start.set(new_cool_start);
                                cool_end.set(new_cool_end);
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
