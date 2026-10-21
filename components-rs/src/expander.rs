use leptos::prelude::*;

#[slot]
pub struct SecondaryCommands {
    children: Children,
}

#[component]
pub fn Expander(
    #[prop(into)] title: String,
    children: ChildrenFn,
    #[prop(optional)] expanded_by_default: bool,
    #[prop(optional)] secondary_commands: Option<SecondaryCommands>,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal_local(expanded_by_default);

    let icon = Signal::derive_local({
        let is_expanded = is_expanded.clone();
        move || {
            if is_expanded.get() {
                "⯆"
            } else {
                "⯈"
            }
        }
    });

    let secondary_commands = secondary_commands.map(|sc| {
        view! { <div>{(sc.children)().into_any()}</div> }
    });

    view! {
        <div class="expander panel">
            <div class="caption">
                <button
                    class="toolbar"
                    on:click=move |_| { set_is_expanded.update(|expanded| *expanded = !*expanded) }
                >
                    <h1>{icon} {title}</h1>
                </button>
                {secondary_commands}
            </div>
            <Show when=move || is_expanded.get()>{children().into_any()}</Show>
        </div>
    }
}
