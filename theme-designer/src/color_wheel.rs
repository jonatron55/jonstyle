use leptos::prelude::*;
use themelib::color::okhsl;

#[component]
pub fn ColorWheel(
    cool_start: RwSignal<f64, LocalStorage>,
    cool_end: RwSignal<f64, LocalStorage>,
    warm_start: RwSignal<f64, LocalStorage>,
    warm_end: RwSignal<f64, LocalStorage>,
    offset: RwSignal<f64, LocalStorage>,
) -> impl IntoView {
    view! {
        <div>
            <svg
                viewBox="-512 -512 1024 1024"
                style="min-width: 192px; min-height: 192px; max-width: 512px; max-height: 512px;"
            >
                <g transform=move || {
                    format!("rotate({})", offset.get())
                }>
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
                        let cool_start = cool_start.get();
                        let cool_end = cool_end.get();
                        let warm_start = warm_start.get();
                        let warm_end = warm_end.get();
                        let markers = [
                            cool_start,
                            (cool_start + cool_end) * 0.5,
                            cool_end,
                            warm_start,
                            (warm_start + warm_end) * 0.5,
                            warm_end,
                        ];
                        {
                            markers
                                .into_iter()
                                .map(|marker| {
                                    view! {
                                        <g
                                            transform=move || { format!("rotate({})", -marker) }
                                            class="marker-group"
                                        >
                                            <path class="marker-line" d="M 0 0 L -256 0" />
                                            <path class="marker-line-inv" d="M -448 0 L-256 0" />
                                            <path class="marker-line" d="M 0 0 L -256 0" />
                                            <path class="marker" d="M -448 0 L -512 -24 L -512 24 Z" />
                                        </g>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    }}
                </g>
            </svg>
        </div>
    }
}
