use leptos::prelude::*;

#[component]
pub fn ValueSlider(
    id: &'static str,
    label: &'static str,
    value: RwSignal<f32, LocalStorage>,
    min: f32,
    max: f32,
    step: f32,
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
                    let new_value = ev.target().value().parse::<f32>().unwrap();
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
                    let new_value = ev.target().value().parse::<f32>().unwrap();
                    value.set(new_value);
                }
            />
        </div>
    }
}
