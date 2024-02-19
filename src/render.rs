use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

/// A render job for `process_chapter`.
pub enum Render<'a> {
    /// Left as is.
    Text(&'a str),
    /// Perform syntax highlighting on this text.
    CodeBlock(&'a str, &'a str)
}

/// Render a codeblock with a specific language.
pub fn render(item: &str, lang: &str) -> String {
    // If language isn't set, don't highlight.
    if lang.is_empty() {
        return item.into();
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    if let Some(syntax) = ss.find_syntax_by_extension(lang) {
        let theme = &ts.themes["base16-ocean.dark"];

        highlighted_html_for_string(item, &ss, syntax, theme).unwrap()
    } else {
        item.into()
    }
}
