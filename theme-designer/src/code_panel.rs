use leptos::prelude::*;
use themelib::theme::{ColorVocabulary, ThemeBuilder};

#[component]
pub fn CodePanel(builder: RwSignal<ThemeBuilder, LocalStorage>) -> impl IntoView {
    let vocab = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| format!("{:?}", b.code.vocab))
    });

    let bold_keywords = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.code.bold_keywords)
    });

    let bold_punctuation = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.code.bold_punctuation)
    });

    let italic_comments = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.code.italic_comments)
    });

    let italic_strings = Signal::derive_local({
        let builder = builder.clone();
        move || builder.with(|b| b.code.italic_strings)
    });

    let vocab_changed = move |ev| {
        let new_value = event_target_value(&ev);

        let vocab = match new_value.as_str() {
            "JonStyle" => ColorVocabulary::JonStyle,
            "VisualStudio" => ColorVocabulary::VisualStudio,
            "Vim" => ColorVocabulary::Vim,
            "Monokai" => ColorVocabulary::Monokai,
            "Primer" => ColorVocabulary::Primer,
            _ => ColorVocabulary::JonStyle,
        };

        builder.update(|b| b.code.vocab = vocab);
    };

    view! {
        <div class="panel">
            <h1 class="caption">"Code style"</h1>
            <div class="meta content">
                <label>"Vocabulary"</label>
                <select prop:value=vocab on:change=vocab_changed>
                    <option value="JonStyle">"Default"</option>
                    <option value="VisualStudio">"Visual Studio"</option>
                    <option value="Vim">"Vim"</option>
                    <option value="Monokai">"Monokai"</option>
                    <option value="Primer">"Primer"</option>
                </select>

                <div style="grid-column: span 2;">
                    <input
                        id="bold-keywords"
                        type="checkbox"
                        prop:checked=bold_keywords
                        on:change=move |_| {
                            builder.update(|b| b.code.bold_keywords = !b.code.bold_keywords)
                        }
                    />
                    <label for="bold-keywords">"Bold Keywords"</label>
                </div>

                <div style="grid-column: span 2;">
                    <input
                        id="bold-punctuation"
                        type="checkbox"
                        prop:checked=bold_punctuation
                        on:change=move |_| {
                            builder.update(|b| b.code.bold_punctuation = !b.code.bold_punctuation)
                        }
                    />
                    <label for="bold-punctuation">"Bold Punctuation"</label>
                </div>

                <div style="grid-column: span 2;">
                    <input
                        id="italic-comments"
                        type="checkbox"
                        prop:checked=italic_comments
                        on:change=move |_| {
                            builder.update(|b| b.code.italic_comments = !b.code.italic_comments)
                        }
                    />
                    <label for="italic-comments">"Italic Comments"</label>
                </div>

                <div style="grid-column: span 2;">
                    <input
                        id="italic-strings"
                        type="checkbox"
                        prop:checked=italic_strings
                        on:change=move |_| {
                            builder.update(|b| b.code.italic_strings = !b.code.italic_strings)
                        }
                    />
                    <label for="italic-strings">"Italic Strings"</label>
                </div>
            </div>
        </div>
    }
}
