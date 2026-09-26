use leptos::prelude::*;

#[component]
pub fn ValueSlider(
    id: &'static str,
    label: &'static str,
    value: Signal<f64, LocalStorage>,
    min: f64,
    max: f64,
    step: f64,
    on_change: Callback<f64, ()>,
) -> impl IntoView {
    view! {
        <div class="value-slider">
            <label for=id>{label}</label>
            <input
                type="range"
                id
                min=min
                max=max
                step=step
                prop:value=value
                on:input:target=move |ev| {
                    let new_value = ev.target().value().parse::<f64>().unwrap();
                    on_change.run(new_value);
                }
            />
            <input
                type="number"
                id
                min=min
                max=max
                step=step
                prop:value=value
                on:input:target=move |ev| {
                    let new_value = ev.target().value().parse::<f64>().unwrap();
                    on_change.run(new_value);
                }
            />
        </div>
    }
}

#[component]
pub fn RangeSlider(
    id: &'static str,
    label: &'static str,
    value: Signal<(f64, f64), LocalStorage>,
    min: f64,
    max: f64,
    step: f64,
    on_change: Callback<(f64, f64), ()>,
) -> impl IntoView {
    view! {
        <div class="range-slider">
            <label for=id>{label}</label>
            <input
                type="range"
                id
                min=min
                max=max
                step=step
                prop:value=move || value.get().0
                on:input:target=move |ev| {
                    let new_value = ev
                        .target()
                        .value()
                        .parse::<f64>()
                        .unwrap()
                        .min(value.get().1 - step);
                    on_change.run((new_value, value.get().1));
                }
            />
            <input
                type="number"
                id
                min=min
                max=move || value.get().1 - step
                step=step
                prop:value=move || value.get().0
                on:input:target=move |ev| {
                    let new_value = ev.target().value().parse::<f64>().unwrap();
                    on_change.run((new_value, value.get().1));
                }
            />
            <input
                type="range"
                id
                min=min
                max=max
                step=step
                prop:value=move || value.get().1
                on:input:target=move |ev| {
                    let new_value = ev
                        .target()
                        .value()
                        .parse::<f64>()
                        .unwrap()
                        .max(value.get().0 + step);
                    on_change.run((value.get().0, new_value));
                }
            />
            <input
                type="number"
                id
                min=move || value.get().0 + step
                max=max
                step=step
                prop:value=move || value.get().1
                on:input:target=move |ev| {
                    let new_value = ev.target().value().parse::<f64>().unwrap();
                    on_change.run((value.get().0, new_value));
                }
            />
        </div>
    }
}
