use leptos::prelude::*;
use themelib::theme::{HueBuilder, ThemeBuilder};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::color_wheel::ColorWheel;
use crate::slider::ValueSlider;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PanelMode {
    Analogous,
    Complementary,
    Triadic,
    Custom,
}

#[component]
pub fn HuePanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let mode = Memo::new({
        let builder = builder.clone();
        move |_| {
            builder.with(|b| match &b.hue {
                HueBuilder::Analogous { .. } => PanelMode::Analogous,
                HueBuilder::Complementary { .. } => PanelMode::Complementary,
                HueBuilder::Triadic { .. } => PanelMode::Triadic,
                HueBuilder::Custom { .. } => PanelMode::Custom,
            })
        }
    });

    let ctrls = {
        let builder = builder.clone();
        move || match mode.get() {
            PanelMode::Analogous => view! { <AnalogousControls builder=builder.clone() /> }.into_any(),
            PanelMode::Complementary => view! { <ComplementaryControls builder=builder.clone() /> }.into_any(),
            PanelMode::Triadic => view! { <TriadicControls builder=builder.clone() /> }.into_any(),
            PanelMode::Custom => view! { <CustomControls builder=builder.clone() /> }.into_any(),
        }
    };

    let hues = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.hue.clone().build_hues().map(|h| h.to_degrees()))
    });

    let primary_index = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.hue.clone().primary_index())
    });

    let on_mode_selected = Callback::new({
        let builder = builder.clone();
        move |selected: PanelMode| {
            if mode.get_untracked() == selected {
                return;
            }

            let hue = match selected {
                PanelMode::Analogous => HueBuilder::default_analogous(),
                PanelMode::Complementary => HueBuilder::default_complementary(),
                PanelMode::Triadic => HueBuilder::default_triadic(),
                PanelMode::Custom => HueBuilder::default_custom(),
            };

            builder.update(|b| b.hue = hue);
        }
    });

    view! {
        <div class="panel">
            <div class="caption split-caption toolbar">
                <h1 class="primary">"Hues"</h1>
                <div>
                    <input
                        type="radio"
                        class="toolbar button"
                        name="hue-mode"
                        value="analogous"
                        id="analogous"
                        prop:checked=move || mode.get() == PanelMode::Analogous
                        on:change:target=move |ev| {
                            if ev.target().checked() {
                                on_mode_selected.run(PanelMode::Analogous);
                            }
                        }
                    />
                    <label for="analogous">"Analogous"</label>
                    <input
                        type="radio"
                        class="toolbar button"
                        name="hue-mode"
                        value="complementary"
                        id="complementary"
                        prop:checked=move || mode.get() == PanelMode::Complementary
                        on:change:target=move |ev| {
                            if ev.target().checked() {
                                on_mode_selected.run(PanelMode::Complementary);
                            }
                        }
                    />
                    <label for="complementary">"Complementary"</label>
                    <input
                        type="radio"
                        class="toolbar button"
                        name="hue-mode"
                        value="triadic"
                        id="triadic"
                        prop:checked=move || mode.get() == PanelMode::Triadic
                        on:change:target=move |ev| {
                            if ev.target().checked() {
                                on_mode_selected.run(PanelMode::Triadic);
                            }
                        }
                    />
                    <label for="triadic">"Triadic"</label>
                    <input
                        type="radio"
                        class="toolbar button"
                        name="hue-mode"
                        value="custom"
                        id="custom"
                        prop:checked=move || mode.get() == PanelMode::Custom
                        on:change:target=move |ev| {
                            if ev.target().checked() {
                                on_mode_selected.run(PanelMode::Custom);
                            }
                        }
                    />
                    <label for="custom">"Custom"</label>
                </div>
            </div>
            <div class="split-2">{ctrls} <ColorWheel hues primary_index /></div>
        </div>
    }
}

#[component]
fn AnalogousControls(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let primary = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Analogous { primary, .. } => *primary,
            _ => 0.0,
        }
    });

    let spread = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Analogous { spread, .. } => *spread,
            _ => 0.0,
        }
    });

    view! {
        <div>
            <ValueSlider
                label="Primary"
                id="primary"
                min=-359.0
                max=359.0
                step=1.0
                value=primary
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Analogous { primary, .. } = &mut b.hue {
                                    *primary = new_value;
                                }
                            })
                    }
                })
            />

            <ValueSlider
                label="Spread"
                id="spread"
                min=-359.0
                max=359.0
                step=1.0
                value=spread
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Analogous { spread, .. } = &mut b.hue {
                                    *spread = new_value;
                                }
                            })
                    }
                })
            />
        </div>
    }
}

#[component]
fn ComplementaryControls(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let primary = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Complementary { primary, .. } => *primary,
            _ => 0.0,
        }
    });

    let spread = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Complementary { spread, .. } => *spread,
            _ => 0.0,
        }
    });

    let offset = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Complementary { offset, .. } => *offset,
            _ => 0.0,
        }
    });

    let reverse = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Complementary { reverse, .. } => *reverse,
            _ => false,
        }
    });

    view! {
        <div>
            <ValueSlider
                label="Primary"
                id="primary"
                min=-359.0
                max=359.0
                step=1.0
                value=primary
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Complementary { primary, .. } = &mut b.hue {
                                    *primary = new_value;
                                }
                            })
                    }
                })
            />

            <ValueSlider
                label="Spread"
                id="spread"
                min=-359.0
                max=359.0
                step=1.0
                value=spread
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Complementary { spread, .. } = &mut b.hue {
                                    *spread = new_value;
                                }
                            })
                    }
                })
            />

            <ValueSlider
                label="Offset"
                id="offset"
                min=-359.0
                max=359.0
                step=1.0
                value=offset
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Complementary { offset, .. } = &mut b.hue {
                                    *offset = new_value;
                                }
                            })
                    }
                })
            />

            <div style="display: flex; justify-content: center; align-items: center; grid-column: span 2">
                <input
                    type="checkbox"
                    id="reverse"
                    class="toggle-switch"
                    checked=reverse
                    on:change={
                        let builder = builder.clone();
                        move |ev| {
                            let is_checked = ev
                                .target()
                                .unwrap()
                                .unchecked_into::<HtmlInputElement>()
                                .checked();
                            builder
                                .update(|b| {
                                    if let HueBuilder::Complementary { reverse, .. } = &mut b.hue {
                                        *reverse = is_checked;
                                    }
                                });
                        }
                    }
                />
                <label for="reverse">"Reverse"</label>
            </div>
        </div>
    }
}

#[component]
fn TriadicControls(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let primary = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Triadic { primary, .. } => *primary,
            _ => 0.0,
        }
    });

    let spread = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Triadic { spread, .. } => *spread,
            _ => 0.0,
        }
    });

    view! {
        <div>
            <ValueSlider
                label="Primary"
                id="primary"
                min=-359.0
                max=359.0
                step=1.0
                value=primary
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Triadic { primary, .. } = &mut b.hue {
                                    *primary = new_value;
                                }
                            })
                    }
                })
            />

            <ValueSlider
                label="Spread"
                id="spread"
                min=-359.0
                max=359.0
                step=1.0
                value=spread
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Triadic { spread, .. } = &mut b.hue {
                                    *spread = new_value;
                                }
                            })
                    }
                })
            />
        </div>
    }
}

#[component]
fn CustomControls(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let cold = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { cold, .. } => *cold,
            _ => 0.0,
        }
    });

    let cool = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { cool, .. } => *cool,
            _ => 0.0,
        }
    });

    let coolish = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { coolish, .. } => *coolish,
            _ => 0.0,
        }
    });

    let warmish = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { warmish, .. } => *warmish,
            _ => 0.0,
        }
    });

    let warm = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { warm, .. } => *warm,
            _ => 0.0,
        }
    });

    let hot = Signal::derive_local({
        let builder = builder.clone();
        move || match &builder.get().hue {
            HueBuilder::Custom { hot, .. } => *hot,
            _ => 0.0,
        }
    });

    view! {
        <div>
            <ValueSlider
                label="Cold"
                id="cold"
                min=-359.0
                max=359.0
                step=1.0
                value=cold
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { cold, .. } = &mut b.hue {
                                    *cold = new_value;
                                }
                            })
                    }
                })
            />
            <ValueSlider
                label="Cool"
                id="cool"
                min=-359.0
                max=359.0
                step=1.0
                value=cool
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { cool, .. } = &mut b.hue {
                                    *cool = new_value;
                                }
                            })
                    }
                })
            />
            <ValueSlider
                label="Coolish"
                id="coolish"
                min=-359.0
                max=359.0
                step=1.0
                value=coolish
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { coolish, .. } = &mut b.hue {
                                    *coolish = new_value;
                                }
                            })
                    }
                })
            />
            <ValueSlider
                label="Warmish"
                id="warmish"
                min=-359.0
                max=359.0
                step=1.0
                value=warmish
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { warmish, .. } = &mut b.hue {
                                    *warmish = new_value;
                                }
                            })
                    }
                })
            />
            <ValueSlider
                label="Warm"
                id="warm"
                min=-359.0
                max=359.0
                step=1.0
                value=warm
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { warm, .. } = &mut b.hue {
                                    *warm = new_value;
                                }
                            })
                    }
                })
            />
            <ValueSlider
                label="Hot"
                id="hot"
                min=-359.0
                max=359.0
                step=1.0
                value=hot
                on_change=Callback::new({
                    let builder = builder.clone();
                    move |new_value| {
                        builder
                            .update(|b| {
                                if let HueBuilder::Custom { hot, .. } = &mut b.hue {
                                    *hot = new_value;
                                }
                            })
                    }
                })
            />
        </div>
    }
}
