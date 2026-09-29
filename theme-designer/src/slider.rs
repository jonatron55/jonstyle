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
    let num_id = format!("{id}-num");
    view! {
        <div class="value-slider">
            <label for=id>{label}</label>
            <input
                type="range"
                id=id
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
                id=num_id
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
    let min_id = format!("{id}-min");
    let max_id = format!("{id}-max");
    let min_num_id = format!("{id}-min-num");
    let max_num_id = format!("{id}-max-num");

    view! {
        <div class="range-slider">
            <label for=id>{label}</label>
            <input
                type="range"
                id=min_id
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
                id=min_num_id
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
                id=max_id
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
                id=max_num_id
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
