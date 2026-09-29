pub enum HueBuilder {
    Analogous {
        primary: f64,
        spread: f64,
    },
    Complementary {
        axis: f64,
        spread: f64,
        bend: f64,
        reverse: bool,
    },
    Triadic {
        primary: f64,
        spread: f64,
    },
}

/// Opinionated style choices for syntax highlighting.
pub enum CodeVocabulary {
    /// Use the color vocabulary of Visual Studio.
    ///
    /// For example:
    /// - Keywords are colored blue.
    /// - Comments are  green.
    /// - Literals are dull red or purple.
    /// - Types are teal.
    VisualStudio,

    /// Use the color vocabulary of Vim.
    ///
    /// For example:
    /// - Keywords are colored yellow.
    /// - Comments are cyan.
    /// - Literals are pink and red .
    /// - Types are bright green.
    Vim,

    /// Use the color vocabulary of Monokai/Sublime Text.
    ///
    /// For example:
    /// - Keywords are colored pink.
    /// - Comments are muted grey.
    /// - Literals are orange.
    /// - Types are light blue.
    Monokai,
}

pub struct CodeOptions {
    bold_keywords: bool,
    bold_punctuation: bool,
    italic_comments: bool,
    italic_strings: bool,
}
