use leptos::prelude::*;
use semver::Version;
use themelib::theme::ThemeBuilder;

#[component]
pub fn MetaPanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let name = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.meta.name.clone())
    });
    let author = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.meta.author.clone().unwrap_or_default())
    });
    let description = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.meta.description.clone().unwrap_or_default())
    });
    let version = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.meta.version.clone().to_string())
    });

    view! {
        <div class="panel">
            <h1 class="caption">"Metadata"</h1>
            <div class="meta content">
                <label for="theme-name">"Name"</label>
                <input
                    id="theme-name"
                    type="text"
                    prop:value=name
                    on:input:target=move |ev| {
                        builder.update(|b| b.meta.name = ev.target().value());
                    }
                    placeholder="Theme name"
                />
                <label for="theme-author">"Author"</label>
                <input
                    id="theme-author"
                    type="text"
                    prop:value=author
                    on:input:target=move |ev| {
                        builder.update(|b| b.meta.author = Some(ev.target().value()));
                    }
                    placeholder="Theme author"
                />
                <label for="theme-description">"Description"</label>
                <input
                    id="theme-description"
                    type="text"
                    prop:value=description
                    on:input:target=move |ev| {
                        builder.update(|b| b.meta.description = Some(ev.target().value()));
                    }
                    placeholder="Theme description"
                />
                <label for="theme-version">"Version"</label>
                <input
                    id="theme-version"
                    type="text"
                    prop:value=version
                    on:input:target=move |ev| {
                        builder
                            .update(|b| {
                                b.meta.version = ev
                                    .target()
                                    .value()
                                    .parse()
                                    .unwrap_or(Version::new(0, 1, 0));
                            });
                    }
                    placeholder="Theme version"
                />
            </div>
        </div>
    }
}
