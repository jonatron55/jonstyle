use leptos::prelude::*;
use themelib::theme::{lum_fn, LUM_COUNT};

const LUM_PLOT_SIZE: f64 = 192.0;
const LUM_PLOT_SIZE_HALF: f64 = LUM_PLOT_SIZE / 2.0;

#[component]
pub fn LumPlot(
    lum_range: Signal<(f64, f64), LocalStorage>,
    lum_power: Signal<f64, LocalStorage>,
    lum_gamma: Signal<f64, LocalStorage>,
) -> impl IntoView {
    view! {
        <div style="margin: auto;">
            <svg
                viewBox=format!("0 0 {LUM_PLOT_SIZE} {LUM_PLOT_SIZE}")
                style=format!("width: {LUM_PLOT_SIZE}px; height: {LUM_PLOT_SIZE}px;")
            >
                <path
                    class="plot-area"
                    d=format!(
                        "M0 0 L{LUM_PLOT_SIZE} 0 L{LUM_PLOT_SIZE} {LUM_PLOT_SIZE} L0 {LUM_PLOT_SIZE} Z",
                    )
                />

                <g>
                    {move || {
                        (1..(LUM_COUNT - 1))
                            .map(|i| {
                                let x = LUM_PLOT_SIZE * i as f64 / (LUM_COUNT - 1) as f64;
                                view! {
                                    <path
                                        class="minor-gridline"
                                        d=format!("M{x} 0 L{x} {LUM_PLOT_SIZE}")
                                    />
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </g>

                <path class="major-gridline" d=format!("M0 0 L{LUM_PLOT_SIZE} 0") />
                <path
                    class="major-gridline"
                    d=format!("M0 {LUM_PLOT_SIZE_HALF} L{LUM_PLOT_SIZE} {LUM_PLOT_SIZE_HALF}")
                />

                <path
                    class="plot"
                    d=move || {
                        let points = (0..101)
                            .map(|i| {
                                let t = i as f64 / 100.0;
                                let x = LUM_PLOT_SIZE * t;
                                let y = LUM_PLOT_SIZE
                                    - LUM_PLOT_SIZE
                                        * (lum_fn(
                                            t,
                                            lum_range.get(),
                                            lum_power.get(),
                                            lum_gamma.get(),
                                        ));
                                format!("{x} {y}")
                            })
                            .collect::<Vec<_>>();
                        format!("M {}", points.join(" L "))
                    }
                />

                <path
                    class="axis"
                    d=format!(
                        "M0 0 L{LUM_PLOT_SIZE} 0 L{LUM_PLOT_SIZE} {LUM_PLOT_SIZE} L0 {LUM_PLOT_SIZE} Z",
                    )
                />
            </svg>
        </div>
    }
}
