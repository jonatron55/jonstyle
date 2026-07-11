use leptos::prelude::*;
use themelib::theme::{Indexer, Lum, Sat, Temp, Theme};

#[component]
pub fn PalettePanel(theme: ReadSignal<Theme, LocalStorage>) -> impl IntoView {
    view! {
        <div class="palette panel">
            <h1 class="caption">"Base Palette"</h1>
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
                                                                        theme[&Indexer::Base(sat, temp, lum)].to_srgb()
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
