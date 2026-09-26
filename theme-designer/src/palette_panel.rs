use anyhow::{Error as AnyError, Result as AnyResult};
use components::ExportButton;
use leptos::prelude::*;
use themelib::{
    scripts::pal::write_pal,
    template::fmt_string,
    theme::{Indexer, Lum, Sat, Temp, Theme},
};

#[component]
pub fn PalettePanel(
    theme: Signal<Theme, LocalStorage>,
    on_error: impl Callable<AnyError, ()> + Clone + 'static,
) -> impl IntoView {
    let name = Signal::derive_local({
        let theme = theme.clone();
        move || theme.with(|theme| fmt_string(&theme.name, "k"))
    });

    let export_pal = Callback::new({
        let theme = theme.clone();
        move |_| -> AnyResult<(Vec<u8>, String)> {
            let bytes = theme.with(|theme| -> AnyResult<Vec<u8>> {
                let mut bytes = vec![];
                write_pal(theme, &mut bytes)?;
                Ok(bytes)
            })?;
            Ok((bytes, "application/octet-stream".to_string()))
        }
    });

    view! {
        <div class="palette panel">
            <div class="caption split-caption toolbar">
                <h1 class="primary">"Base Palette"</h1>
                <div class="secondary">
                    <ExportButton
                        caption="Export PAL"
                        name=name
                        extension="pal"
                        on_write=export_pal.clone()
                        on_error=on_error.clone()
                        {..}
                        class="toolbar"
                    />
                </div>
            </div>
            <div class="content palette-grid">
                {Lum::iter()
                    .map(|lum| {
                        view! {
                            <div class="palette-row">
                                <span class="swatches">
                                    {Temp::iter()
                                        .flat_map(|temp| {
                                            Sat::iter()
                                                .map(move |sat| {
                                                    view! {
                                                        <span
                                                            class="swatch"
                                                            style:background-color=move || {
                                                                let color = theme
                                                                    .with(|theme| {
                                                                        theme
                                                                            .get(&Indexer::Base(sat, temp, lum))
                                                                            .without_a()
                                                                            .to_srgb()
                                                                    });
                                                                format!("#{color:X}")
                                                            }
                                                        ></span>
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                        })
                                        .collect::<Vec<_>>()}
                                </span>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
