use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

/// A render job for `process_chapter`.
pub enum Render<'a> {
    /// No need to render.
    Text(&'a str),
    /// A render code block.
    CodeBlock(&'a str, &'a str)
}

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
