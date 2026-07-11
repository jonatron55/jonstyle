mod color_wheel;
mod hue_panel;
mod lum_panel;
mod lum_plot;
mod meta_panel;
mod palette_panel;
mod plot;
mod sat_panel;
mod slider;
mod toolbar;

use leptos::prelude::*;
use semver::Version;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use hue_panel::HuePanel;
use lum_panel::LumPanel;
use meta_panel::MetaPanel;
use palette_panel::PalettePanel;
use sat_panel::SatPanel;
use themelib::theme::{Indexer, Theme, ThemeBuilder, ThemeVariant};
use toolbar::Toolbar;

fn apply_theme_to_root(theme: &Theme, variant: ThemeVariant) {
    let Some(root) = document().document_element().and_then(|el| el.dyn_into::<HtmlElement>().ok()) else {
        return;
    };
    let style = root.style();

    for indexer in Indexer::iter(variant) {
        let color = theme[&indexer].to_srgb();
        let _ = style.set_property(&format!("--{indexer}"), &format!("#{color:X}"));
    }
}

#[component]
fn App() -> impl IntoView {
    let builder = StoredValue::new_local(ThemeBuilder::default());
    let (theme, set_theme) = signal_local(ThemeBuilder::default().into_theme());
    let (preview_variant, set_preview_variant) = signal_local(ThemeVariant::NIGHT);

    let name = RwSignal::new_local(builder.with_value(|b| b.name.clone()));
    let author = RwSignal::new_local(builder.with_value(|b| b.author.clone().unwrap_or_default()));
    let description = RwSignal::new_local(builder.with_value(|b| b.description.clone().unwrap_or_default()));
    let version = RwSignal::new_local(builder.with_value(|b| b.version.clone().to_string()));
    let cool_start = RwSignal::new_local(builder.with_value(|b| b.cool_range.0));
    let cool_end = RwSignal::new_local(builder.with_value(|b| b.cool_range.1));
    let warm_start = RwSignal::new_local(builder.with_value(|b| b.warm_range.0));
    let warm_end = RwSignal::new_local(builder.with_value(|b| b.warm_range.1));
    let offset = RwSignal::new_local(builder.with_value(|b| b.offset));
    let lum_range = RwSignal::new_local(builder.with_value(|b| b.lum_range));
    let lum_power = RwSignal::new_local(builder.with_value(|b| b.lum_power));
    let lum_gamma = RwSignal::new_local(builder.with_value(|b| b.lum_gamma));
    let muted_sat_range = RwSignal::new_local(builder.with_value(|b| b.muted_sat_range));
    let base_sat_range = RwSignal::new_local(builder.with_value(|b| b.base_sat_range));
    let intense_sat_range = RwSignal::new_local(builder.with_value(|b| b.intense_sat_range));

    _ = Effect::new(move |_| {
        let name = name.get();
        let author = author.get();
        let description = description.get();
        let version = version.get();
        let cool_start = cool_start.get();
        let cool_end = cool_end.get();
        let warm_start = warm_start.get();
        let warm_end = warm_end.get();
        let offset = offset.get();
        let lum_range = lum_range.get();
        let lum_power = lum_power.get();
        let lum_gamma = lum_gamma.get();
        let muted_sat_range = muted_sat_range.get();
        let base_sat_range = base_sat_range.get();
        let intense_sat_range = intense_sat_range.get();

        builder.update_value(|b| {
            b.name = name;
            b.author = if author.is_empty() { None } else { Some(author) };
            b.description = if description.is_empty() {
                None
            } else {
                Some(description)
            };
            b.version = version.parse().unwrap_or(Version::new(0, 1, 0));
            b.cool_range = (cool_start, cool_end);
            b.warm_range = (warm_start, warm_end);
            b.offset = offset;

            b.lum_range = lum_range;
            b.lum_power = lum_power;
            b.lum_gamma = lum_gamma;
            b.muted_sat_range = muted_sat_range;
            b.base_sat_range = base_sat_range;
            b.intense_sat_range = intense_sat_range;

            set_theme.update(|theme| *theme = b.build_theme());
        })
    });

    Effect::new(move |_| {
        theme.with(|theme| apply_theme_to_root(theme, preview_variant.get()));
    });

    let preview_variant_changed = Callback::new(move |variant| {
        set_preview_variant.set(variant);
    });

    view! {
        <div class="app">
            <div class="top">
                <Toolbar on_variant_changed=preview_variant_changed />
            </div>
            <div class="left">
                <MetaPanel name author description version />
                <HuePanel cool_start cool_end warm_start warm_end offset />
                <LumPanel lum_range lum_power lum_gamma />
                <SatPanel
                    muted_range=muted_sat_range
                    base_range=base_sat_range
                    intense_range=intense_sat_range
                />
                <PalettePanel theme />
            </div>
            <div class="right"></div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
