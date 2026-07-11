use leptos::prelude::*;
use themelib::theme::ThemeVariant;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn Toolbar(on_variant_changed: Callback<ThemeVariant>) -> impl IntoView {
    view! {
        <div class="toolbar panel">
            <input
                type="radio"
                name="variant"
                class="toolbar button"
                id="dawn"
                on:change=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if input.checked() {
                        on_variant_changed.run(ThemeVariant::DAWN);
                    }
                }
            />
            <label for="dawn" class="toolbar button">
                "Dawn"
            </label>
            <input
                type="radio"
                name="variant"
                class="toolbar button"
                id="noon"
                on:change=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if input.checked() {
                        on_variant_changed.run(ThemeVariant::NOON);
                    }
                }
            />
            <label for="noon" class="toolbar button">
                "Noon"
            </label>
            <input
                type="radio"
                name="variant"
                class="toolbar button"
                id="dusk"
                on:change=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if input.checked() {
                        on_variant_changed.run(ThemeVariant::DUSK);
                    }
                }
            />
            <label for="dusk" class="toolbar button">
                "Dusk"
            </label>
            <input
                type="radio"
                name="variant"
                class="toolbar button"
                id="night"
                checked=true
                on:change=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if input.checked() {
                        on_variant_changed.run(ThemeVariant::NIGHT);
                    }
                }
            />
            <label for="night" class="toolbar button">
                "Night"
            </label>
        </div>
    }
}
