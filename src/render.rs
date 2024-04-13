use syntect::easy::HighlightLines;
use syntect::html::{append_highlighted_html_for_styled_line, IncludeBackground};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::util::LinesWithEndings;
use syntect::Error;

use crate::cfg::SyntectConfig;

/// A render job for `process_chapter`.
pub enum Render<'a> {
    /// Left as is.
    Text(&'a str),
    /// Perform syntax highlighting on this text.
    CodeBlock(&'a str, &'a str)
}

/// Start highlighted HTML snippet, with class="code-block"
pub fn start_highlighted_html_snippet(t: &Theme) -> (String, Color) {
    let c = t.settings.background.unwrap_or(Color::WHITE);
    (
        format!(
            "<pre class=\"code-block\" style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
            c.r, c.g, c.b
        ),
        c,
    )
}

/// Highlight a string into HTML.
fn highlight_string(
    s: &str,
    ss: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
) -> Result<String, Error> {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);

    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight_line(line, ss)?;
        append_highlighted_html_for_styled_line(
            &regions[..],
            IncludeBackground::IfDifferent(bg),
            &mut output,
        )?;
    }
    output.push_str("</pre>\n");
    Ok(output)
}

/// Render a codeblock with a specific language.
pub fn render(
    cfg: &SyntectConfig,item: &str, lang: &str) -> String {
    // If language isn't set, don't highlight.
    if lang.is_empty() {
        return item.into();
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme = if let Some(path) = &cfg.custom_theme{
        let maybe_theme = ThemeSet::get_theme(path);
        if let Ok(tmtheme) = maybe_theme {
            eprintln!(
                "Loaded theme!"
            );
            tmtheme
        } else {
            eprintln!(
                "Failed to load theme!"
            );
            ts.themes["base16-ocean.dark"].clone()
        }
    } else {
        eprintln!(
            "Using default theme!"
        );
        ts.themes["base16-ocean.dark"].clone()
    };

    if let Some(syntax) = ss.find_syntax_by_extension(lang) {
        highlight_string(item, &ss, syntax, &theme).unwrap()
    } else {
        item.into()
    }
}
