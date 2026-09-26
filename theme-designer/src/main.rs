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
use leptos::prelude::*;
use themelib::theme::{Indexer, Theme, ThemeBuilder, ThemeVariant};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::hue_panel::HuePanel;
use crate::lum_panel::LumPanel;
use crate::meta_panel::MetaPanel;
use crate::palette_panel::PalettePanel;
use crate::preview_content::PreviewContent;
use crate::sat_panel::SatPanel;
use crate::toolbar::Toolbar;

fn apply_theme_to_root(theme: &Theme, variant: ThemeVariant) {
    let Some(root) = document().document_element().and_then(|el| el.dyn_into::<HtmlElement>().ok()) else {
        return;
    };
    let style = root.style();
    for indexer in Indexer::iter(variant) {
        let color = theme.get(&indexer).to_srgba();
        let _ = style.set_property(&format!("--{indexer}"), &format!("#{color:X}"));
    }

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
    apply_theme_to_root(&builder.with_untracked(|b| b.build_theme()), ThemeVariant::NIGHT);

    let theme = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.build_theme())
    });
    let (preview_variant, set_preview_variant) = signal_local(ThemeVariant::NIGHT);

    _ = Effect::new(move |_| {
        theme.with(|theme| apply_theme_to_root(theme, preview_variant.get()));
    });

    let preview_variant_changed = Callback::new(move |variant| {
        set_preview_variant.set(variant);
    });

    let on_error = Callback::new(move |_err: AnyError| {});

    view! {
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
                <HuePanel builder />
                <LumPanel builder />
                <SatPanel builder />
                <PalettePanel theme on_error=on_error.clone() />
            </div>
            <div class="right">
                <div class="panel">
                    <h1 class="caption">"Preview"</h1>
                    <PreviewContent />
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
