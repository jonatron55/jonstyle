mod code_panel;
mod color_wheel;
mod hue_panel;
mod lum_panel;
mod lum_plot;
mod meta_panel;
mod palette_panel;
mod plot;
mod preview_content;
mod sat_panel;
mod slider;
mod toolbar;

use anyhow::Error as AnyError;
use components::Expander;
use components::{Dialog, Severity};
use leptos::prelude::*;
use themelib::theme::{ColorRef, Theme, ThemeBuilder, ThemeVariant};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::code_panel::CodePanel;
use crate::hue_panel::HuePanel;
use crate::lum_panel::LumPanel;
use crate::meta_panel::MetaPanel;
use crate::palette_panel::PalettePanel;
use crate::preview_content::PreviewContent;
use crate::sat_panel::SatPanel;
use crate::toolbar::Toolbar;

fn apply_theme_to_root(theme: &Theme, variant: &ThemeVariant) {
    let Some(root) = document().document_element().and_then(|el| el.dyn_into::<HtmlElement>().ok()) else {
        return;
    };
    let style = root.style();
    for indexer in ColorRef::iter(variant) {
        let color = theme.get(&indexer).to_srgba();
        _ = style.set_property(&format!("--{indexer}"), &format!("#{color:X}"));
    }

    _ = style.set_property(
        "--keyword-weight",
        if theme.code.bold_keywords { "bold" } else { "normal" },
    );
    _ = style.set_property(
        "--punctuation-weight",
        if theme.code.bold_punctuation { "bold" } else { "normal" },
    );
    _ = style.set_property(
        "--comment-style",
        if theme.code.italic_comments { "italic" } else { "normal" },
    );
    _ = style.set_property(
        "--string-style",
        if theme.code.italic_strings { "italic" } else { "normal" },
    );

    let body = root.owner_document().and_then(|doc| doc.body()).unwrap();
    let body_style = body.style();
    let _ = body_style.set_property(
        &format!("background-image"),
        &format!("url('background-{}.jpg')", variant.to_string().to_lowercase()),
    );
}

#[component]
fn App() -> impl IntoView {
    let builder = RwSignal::new_local(ThemeBuilder::default());
    apply_theme_to_root(&builder.with_untracked(|b| b.build_theme()), &ThemeVariant::night());

    let theme = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.build_theme())
    });
    let (preview_variant, set_preview_variant) = signal_local(ThemeVariant::night());

    _ = Effect::new(move |_| {
        theme.with(|theme| apply_theme_to_root(theme, &preview_variant.get()));
    });

    let preview_variant_changed = Callback::new(move |variant| {
        set_preview_variant.set(variant);
    });

    let (load_error, set_load_error) = signal_local(None::<String>);
    let on_error = Callback::new(move |_err: AnyError| set_load_error.set(Some(_err.to_string())));

    view! {
        {move || {
            load_error
                .get()
                .map(|load_error| {
                    view! {
                        <Dialog
                            title="Error parsing configuration"
                            yes_caption="OK"
                            severity=Severity::Danger
                            on_yes=move || set_load_error.set(None)
                        >
                            {load_error}
                        </Dialog>
                    }
                })
        }}
        <div class="app">
            <div class="top">
                <Toolbar
                    builder
                    on_variant_changed=preview_variant_changed
                    on_error=on_error.clone()
                />
            </div>
            <div class="left">
                <MetaPanel builder />
                <CodePanel builder />
                <HuePanel builder />
                <LumPanel builder />
                <SatPanel builder />
            </div>
            <div class="right">
                <PalettePanel theme on_error=on_error.clone() />
                <Expander title="Preview" expanded_by_default=true>
                    <PreviewContent />
                </Expander>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
