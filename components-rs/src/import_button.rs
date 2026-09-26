use anyhow::{Error as AnyError, Result as AnyResult};
use leptos::ev::Targeted;
use leptos::prelude::*;
use rand::RngExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileReader, HtmlInputElement, ProgressEvent};

#[component]
pub fn ImportButton(
    #[prop(into)] caption: String,
    #[prop(into)] extensions: String,
    on_read: impl Callable<(Vec<u8>, String), AnyResult<()>> + Clone + 'static,
    on_error: impl Callable<AnyError, ()> + Clone + 'static,
) -> impl IntoView {
    let file_changed = {
        let on_read = on_read.clone();
        let on_error = on_error.clone();
        move |ev: Targeted<Event, HtmlInputElement>| {
            let input: HtmlInputElement = ev.target().dyn_into().unwrap();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let filename = file.name();
                let reader = FileReader::new().unwrap();
                let onload = Closure::once_into_js({
                    let reader = reader.clone();
                    let on_read = on_read.clone();
                    let on_error = on_error.clone();
                    move |_: ProgressEvent| {
                        let data = js_sys::Uint8Array::new(&reader.result().unwrap()).to_vec();
                        if let Err(err) = on_read.run((data, filename)) {
                            on_error.run(err);
                        }
                    }
                });

                reader.set_onload(Some(onload.unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
            }
        }
    };

    let mut rng = rand::rng();
    let id = (0..8)
        .map(|_| {
            let r = rng.random_range(0..36);
            if r < 10 {
                (b'0' + r as u8) as char
            } else {
                (b'a' + (r - 10) as u8) as char
            }
        })
        .collect::<String>();
    let id = format!("import-button-{}", id);

    view! {
        <input
            id=id.clone()
            class="button"
            type="file"
            accept=extensions
            on:change:target=file_changed
        />
        <label for=id class="icon-button">
            <span class="icon">"↥"</span>
            <span class="caption">{caption}</span>
        </label>
    }
}
