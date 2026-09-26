use std::path::PathBuf;

use anyhow::{Error as AnyError, Result as AnyResult};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn ExportButton(
    #[prop(into)] caption: String,
    #[prop(into)] name: Signal<String, LocalStorage>,
    #[prop(into)] extension: String,
    on_write: impl Callable<i32, AnyResult<(Vec<u8>, String)>> + 'static,
    on_error: impl Callable<AnyError, ()> + 'static,
) -> impl IntoView {
    let export = move |_| match on_write.run(0) {
        Ok((buf, mime_type)) => {
            let props = web_sys::BlobPropertyBag::new();
            props.set_type(&mime_type);
            let array = js_sys::Uint8Array::from(buf.as_slice());
            let blob =
                web_sys::Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(&array), &props).unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            let mut path = PathBuf::from(&name.get());
            path.add_extension(&extension);
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &path.to_string_lossy()).unwrap();
            a.set_attribute("style", "display: none;").unwrap();
            document.body().unwrap().append_child(&a).unwrap();

            let a: web_sys::HtmlAnchorElement = a.dyn_into().unwrap();
            a.click();

            document.body().unwrap().remove_child(&a).unwrap();
            web_sys::Url::revoke_object_url(&url).unwrap();
        }
        Err(err) => {
            on_error.run(err);
        }
    };

    view! {
        <button class="icon-button" on:click=export>
            <span class="icon">"⤓"</span>
            <span class="caption">{caption}</span>
        </button>
    }
}
