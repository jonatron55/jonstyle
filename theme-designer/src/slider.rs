use leptos::prelude::*;

#[component]
pub fn ValueSlider(
    id: &'static str,
    label: &'static str,
    value: RwSignal<f64, LocalStorage>,
    min: f64,
    max: f64,
    step: f64,
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
                    value.set(new_value);
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
                    value.set(new_value);
                }
            />
        </div>
    }
}

#[component]
pub fn RangeSlider(
    id: &'static str,
    label: &'static str,
    value: RwSignal<(f64, f64), LocalStorage>,
    min: f64,
    max: f64,
    step: f64,
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
                    value.update(|v| v.0 = new_value);
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
                    value.update(|v| v.0 = new_value);
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
                    value.update(|v| v.1 = new_value);
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
                    value.update(|v| v.1 = new_value);
                }
            />
        </div>
    }
}
