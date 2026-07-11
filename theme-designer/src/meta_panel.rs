use leptos::prelude::*;

#[component]
pub fn MetaPanel(
    name: RwSignal<String, LocalStorage>,
    author: RwSignal<String, LocalStorage>,
    description: RwSignal<String, LocalStorage>,
    version: RwSignal<String, LocalStorage>,
) -> impl IntoView {
    view! {
        <div class=" panel">
            <h1 class="caption">"Metadata"</h1>
            <div class="meta content">
                <label for="theme-name">"Name"</label>
                <input
                    id="theme-name"
                    type="text"
                    prop:value=name
                    on:input:target=move |ev| {
                        name.set(ev.target().value());
                    }
                    placeholder="Theme name"
                />
                <label for="theme-author">"Author"</label>
                <input
                    id="theme-author"
                    type="text"
                    prop:value=author
                    on:input:target=move |ev| {
                        author.set(ev.target().value());
                    }
                    placeholder="Theme author"
                />
                <label for="theme-description">"Description"</label>
                <input
                    id="theme-description"
                    type="text"
                    prop:value=description
                    on:input:target=move |ev| {
                        description.set(ev.target().value());
                    }
                    placeholder="Theme description"
                />
                <label for="theme-version">"Version"</label>
                <input
                    id="theme-version"
                    type="text"
                    prop:value=version
                    on:input:target=move |ev| {
                        version.set(ev.target().value());
                    }
                    placeholder="Theme version"
                />
            </div>
        </div>
    }
}
