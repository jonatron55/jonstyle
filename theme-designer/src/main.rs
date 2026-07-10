mod color_wheel;
mod meta_panel;
mod palette_panel;
mod parameter_panel;
mod plot;
mod slider;
mod toolbar;

use leptos::prelude::*;
use semver::Version;

use meta_panel::MetaPanel;
use palette_panel::PalettePanel;
use parameter_panel::ParameterPanel;
use themelib::theme::ThemeBuilder;
use toolbar::Toolbar;

#[component]
fn App() -> impl IntoView {
    let builder = StoredValue::new_local(ThemeBuilder::default());

    let name = RwSignal::new_local(builder.with_value(|b| b.name.clone()));
    let author = RwSignal::new_local(builder.with_value(|b| b.author.clone().unwrap_or_default()));
    let description = RwSignal::new_local(builder.with_value(|b| b.description.clone().unwrap_or_default()));
    let version = RwSignal::new_local(builder.with_value(|b| b.version.clone().to_string()));
    let cool_center = RwSignal::new_local(builder.with_value(|b| (b.cool_range.0 + b.cool_range.1) / 2.0));
    let cool_spread = RwSignal::new_local(builder.with_value(|b| b.cool_range.1 - b.cool_range.0));
    let warm_center = RwSignal::new_local(builder.with_value(|b| (b.warm_range.0 + b.warm_range.1) / 2.0));
    let warm_spread = RwSignal::new_local(builder.with_value(|b| b.warm_range.1 - b.warm_range.0));
    let offset = RwSignal::new_local(builder.with_value(|b| b.offset));

    _ = Effect::new(move |_| {
        builder.update_value(|b| {
            b.name = name.get();
            b.author = if author.get().is_empty() {
                None
            } else {
                Some(author.get())
            };
            b.description = if description.get().is_empty() {
                None
            } else {
                Some(description.get())
            };
            b.version = version.get().parse().unwrap_or(Version::new(0, 1, 0));
        });
    });

    view! {
        <div class="app">
            <div class="top">
                <Toolbar />
            </div>
            <div class="left">
                <MetaPanel name author description version />
                <ParameterPanel cool_center cool_spread warm_center warm_spread offset />
            </div>
            <div class="right">
                <PalettePanel />
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
