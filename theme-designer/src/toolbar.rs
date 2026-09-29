use anyhow::{Error as AnyError, Result as AnyResult};
use components::{ExportButton, ImportButton};
use leptos::prelude::*;
use themelib::{
    template::fmt_string,
    theme::{ThemeBuilder, ThemeVariant},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn Toolbar(
    builder: RwSignal<ThemeBuilder, LocalStorage>,
    on_variant_changed: Callback<ThemeVariant>,
    on_error: impl Callable<AnyError, ()> + Clone + 'static,
) -> impl IntoView {
    let write_theme = Callback::new({
        let theme = builder.clone();
        move |_| -> AnyResult<(Vec<u8>, String)> {
            let bytes = theme.with(|theme| -> AnyResult<Vec<u8>> {
                let s = toml::to_string(theme)?;
                Ok(s.as_bytes().to_vec())
            })?;
            Ok((bytes, "application/toml".to_string()))
        }
    });

    let read_theme = Callback::new({
        let theme = builder.clone();
        move |(bytes, _filename): (Vec<u8>, String)| -> AnyResult<()> {
            let s = String::from_utf8(bytes)?;
            let new_theme: ThemeBuilder = toml::from_str(&s)?;
            theme.set(new_theme);
            Ok(())
        }
    });

    let name = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|builder| fmt_string(&builder.meta.name, "k"))
    });

    view! {
        <div class="toolbar split-caption panel">
            <div class="primary">
                <ImportButton
                    caption="Import"
                    extensions=".toml"
                    on_read=read_theme.clone()
                    on_error=on_error.clone()
                    {..}
                    class="toolbar button"
                />
                <ExportButton
                    caption="Export"
                    name=name
                    extension="toml"
                    on_write=write_theme.clone()
                    on_error=on_error.clone()
                    {..}
                    class="toolbar button"
                />
            </div>
            <div class="title">
                <h1 class="title">"Theme Designer"</h1>
            </div>
            <div class="secondary">
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
        </div>
    }
}
