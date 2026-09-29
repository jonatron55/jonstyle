use leptos::prelude::*;
use themelib::{color::okhsl, theme::TEMP_COUNT};

#[component]
pub fn ColorWheel(
    hues: Signal<[f64; TEMP_COUNT], LocalStorage>,
    primary_index: Signal<usize, LocalStorage>,
) -> impl IntoView {
    view! {
        <div>
            <svg
                viewBox="-512 -512 1024 1024"
                style="min-width: 192px; min-height: 192px; max-width: 512px; max-height: 512px;"
            >
                <g>
                    {(0..360)
                        .map(|hue| {
                            let hue = hue as f64;
                            let t0 = (hue - 0.5).to_radians();
                            let t1 = (hue + 1.5).to_radians();
                            let r0 = 256.0;
                            let r1 = 448.0;
                            let a = (-r0 * t0.cos(), r0 * t0.sin());
                            let b = (-r1 * t0.cos(), r1 * t0.sin());
                            let c = (-r1 * t1.cos(), r1 * t1.sin());
                            let d = (-r0 * t1.cos(), r0 * t1.sin());
                            let color = okhsl(hue.to_radians(), 0.95, 0.66).to_srgb();
                            let path = format!(
                                "M{} {} L{} {} L{} {} L{} {} Z",
                                a.0,
                                a.1,
                                b.0,
                                b.1,
                                c.0,
                                c.1,
                                d.0,
                                d.1,
                            );
                            view! { <path d=path style=format!("fill: #{color:X}") /> }
                        })
                        .collect::<Vec<_>>()}
                </g>
                <g>
                    {move || {
                        hues.get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, marker)| {
                                view! {
                                    <g
                                        transform=move || { format!("rotate({})", -marker) }
                                        class="marker-group"
                                    >
                                        <path class="marker-line" d="M 0 0 L -256 0" />
                                        <path class="marker-line-inv" d="M -448 0 L-256 0" />
                                        <path class="marker-line" d="M 0 0 L -256 0" />
                                        {move || {
                                            let is_primary = index == primary_index.get();
                                            view! { <Marker is_primary /> }
                                        }}
                                    </g>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </g>
            </svg>
        </div>
    }
}

#[component]
pub fn Marker(is_primary: bool) -> impl IntoView {
    if is_primary {
        view! { <path class="marker" d="M -448 0 L -512 -24 L -512 24 Z" /> }.into_any()
    } else {
        view! { <path class="marker" d="M -448 6 L -448 -6 L -512 -6 L -512 6 Z" /> }.into_any()
    }
}
