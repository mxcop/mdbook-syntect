use rust_embed::RustEmbed;
use mdbook::errors::Error;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn inject_stylesheet(content: &str) -> Result<String, Error> {
    let style = Asset::get("style.css").expect("style.css not found in assets");
    let style = std::str::from_utf8(style.data.as_ref())?;
    Ok(format!("<style>\n{style}\n</style>\n{content}"))
}
