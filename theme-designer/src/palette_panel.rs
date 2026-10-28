use anyhow::{Error as AnyError, Result as AnyResult};
use components::{Expander, ExportButton, SecondaryCommands};
use leptos::prelude::*;
use themelib::{
    scripts::pal::write_pal,
    template::{fmt_string, write_color},
    theme::{ColorIndex, Lum, Sat, Temp, Theme},
};

#[component]
pub fn PalettePanel(
    theme: Signal<Theme, LocalStorage>,
    on_error: impl Callable<AnyError, ()> + Clone + 'static,
) -> impl IntoView {
    let on_error = UnsyncCallback::new(move |err: AnyError| on_error.run(err));
    let selected = RwSignal::new_local(None::<ColorIndex>);
    let swizzle = RwSignal::new_local("rgb".to_string());
    let space = RwSignal::new_local("G".to_string());
    let unit = RwSignal::new_local("D".to_string());
    let flags = Signal::derive_local(move || format!("{}{}", space.get(), unit.get()));
    let fmt = RwSignal::new_local("X".to_string());

    let color_value = Signal::derive_local(move || {
        let Some(index) = selected.get() else {
            return "<no selection>".to_string();
        };

        let color = theme.with(|theme| theme.get(&index));

        let mut output = Vec::with_capacity(64);
        let mut errors = Vec::new();
        write_color(
            &mut output,
            &color,
            Some(swizzle.get().as_str()),
            Some(flags.get().as_str()),
            Some(&fmt.get()),
            &mut errors,
        )
        .unwrap();
        String::from_utf8(output).unwrap()
    });

    let name = Signal::derive_local({
        let theme = theme.clone();
        move || theme.with(|theme| fmt_string(&theme.meta.name, "k"))
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
        <Expander title="Base Palette" expanded_by_default=true>
            <SecondaryCommands slot>
                <ExportButton
                    caption="Export PAL"
                    name=name
                    extension="pal"
                    on_write=export_pal.clone()
                    on_error=on_error.clone()
                    {..}
                    class="toolbar"
                />
            </SecondaryCommands>
            <div class="content palette">
                <div class="palette-swatches">
                    {Lum::iter()
                        .map(|lum| {
                            view! {
                                <div class="palette-row">
                                    <span class="swatches">
                                        {Temp::iter()
                                            .flat_map(|temp| {
                                                Sat::iter()
                                                    .map(move |sat| {
                                                        let index = ColorIndex::Base(sat, temp, lum);
                                                        let name = fmt_string(index.to_string(), "t");
                                                        view! {
                                                            <button
                                                                type="button"
                                                                class="swatch"
                                                                class:selected={
                                                                    let index = index.clone();
                                                                    move || selected.get().as_ref() == Some(&index)
                                                                }
                                                                aria-label=name.clone()
                                                                title=name.clone()
                                                                aria-pressed={
                                                                    let index = index.clone();
                                                                    move || {
                                                                        (selected.get().as_ref() == Some(&index)).to_string()
                                                                    }
                                                                }
                                                                on:click={
                                                                    let index = index.clone();
                                                                    move |_| selected.set(Some(index.clone()))
                                                                }
                                                                style:background-color={
                                                                    let index = index.clone();
                                                                    move || {
                                                                        let color = theme
                                                                            .with(|theme| { theme.get(&index).without_a().to_srgb() });
                                                                        format!("#{color:X}")
                                                                    }
                                                                }
                                                            ></button>
                                                        }
                                                    })
                                            })
                                            .collect_view()}
                                    </span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="palette-selection">
                    <div
                        class="swatch"
                        style:background-color=move || {
                            if let Some(index) = selected.get() {
                                let color = theme
                                    .with(|theme| { theme.get(&index).without_a().to_srgb() });
                                format!("#{color:X}")
                            } else {
                                "var(--page-background)".to_string()
                            }
                        }
                    ></div>
                    <input
                        type="text"
                        list="swizzle-options"
                        id="swizzle"
                        name="swizzle"
                        prop:value=swizzle
                        on:input:target=move |event| swizzle.set(event.target().value())
                    />
                    <datalist id="swizzle-options">
                        <option value="rgb" />
                        <option value="rgba" />
                        <option value="argb" />
                        <option value="hsl" />
                        <option value="hsla" />
                    </datalist>

                    <div class="flags">
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-x"
                            class="toolbar button"
                            value="x"
                            prop:checked=move || fmt.get() == "x"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-x">"x"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-X"
                            class="toolbar button"
                            value="X"
                            prop:checked=move || fmt.get() == "X"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-X">"X"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-f"
                            class="toolbar button"
                            value="f"
                            prop:checked=move || fmt.get() == "f"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-f">"f"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-e"
                            class="toolbar button"
                            value="e"
                            prop:checked=move || fmt.get() == "e"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-e">"e"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-E"
                            class="toolbar button"
                            value="E"
                            prop:checked=move || fmt.get() == "E"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-E">"E"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-B"
                            class="toolbar button"
                            value="B"
                            prop:checked=move || fmt.get() == "B"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-B">"B"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-p"
                            class="toolbar button"
                            value="p"
                            prop:checked=move || fmt.get() == "p"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-p">"p"</label>
                        <input
                            type="radio"
                            name="fmt"
                            id="fmt-P"
                            class="toolbar button"
                            value="P"
                            prop:checked=move || fmt.get() == "P"
                            on:change:target=move |event| fmt.set(event.target().value())
                        />
                        <label for="fmt-P">"P"</label>
                        <span class="dim">|</span>
                        <input
                            type="radio"
                            name="space"
                            id="space-L"
                            class="toolbar button"
                            value="L"
                            prop:checked=move || space.get() == "L"
                            on:change:target=move |event| space.set(event.target().value())
                        />
                        <label for="space-L">"L"</label>
                        <input
                            type="radio"
                            name="space"
                            id="space-G"
                            class="toolbar button"
                            value="G"
                            prop:checked=move || space.get() == "G"
                            on:change:target=move |event| space.set(event.target().value())
                        />
                        <label for="space-G">"G"</label>
                        <input
                            type="radio"
                            name="unit"
                            id="unit-D"
                            class="toolbar button"
                            value="D"
                            prop:checked=move || unit.get() == "D"
                            on:change:target=move |event| unit.set(event.target().value())
                        />
                        <label for="unit-D">"D"</label>
                        <input
                            type="radio"
                            name="unit"
                            id="unit-R"
                            class="toolbar button"
                            value="R"
                            prop:checked=move || unit.get() == "R"
                            on:change:target=move |event| unit.set(event.target().value())
                        />
                        <label for="unit-R">"R"</label>
                    </div>

                    <div class="selected-value">{color_value}</div>
                </div>
            </div>
        </Expander>
    }
}
