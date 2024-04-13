use std::borrow::Cow;

use mdbook::{book::Book, errors::Error, preprocess::{Preprocessor, PreprocessorContext}, BookItem};

use crate::{cfg::{get_config, SyntectConfig}, embed::inject_stylesheet, render::{self, Render}, scan::{Event, Scan}};

pub struct SyntectProcessor;

impl SyntectProcessor {
    pub fn new() -> SyntectProcessor {
        SyntectProcessor
    }
}

impl Preprocessor for SyntectProcessor {
    fn name(&self) -> &str {
        "syntect"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = get_config(&ctx.config);

        // Load the book chapters.
        let mut chapters = Vec::with_capacity(book.sections.len());
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapters.push(chapter.content.clone());
            }
        });

        // Process all the book chapters.
        let mut contents: Vec<_> = chapters
            .into_iter()
            .rev()
            .map(|raw_content| {
                let styled_content = inject_stylesheet(&raw_content).unwrap();
                process_chapter(
                    &cfg, styled_content
                )
            })
            .collect();

        // Update the book with the processed contents.
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = contents.pop().expect("Chapter number mismatch.");
            }
        });
        
        Ok(book)
    }
}

/// Do the syntax highlighting for codeblocks using syntect.
pub fn process_chapter(
    cfg: &SyntectConfig,
    raw_content: String
) -> String {
    get_render_tasks(&raw_content)
        .into_iter()
        .map(|rend| match rend {
            Render::Text(t) => t.into(),
            Render::CodeBlock(item, lang) => {
                render::render(cfg, item, lang).into()
            }
        })
        .collect::<Vec<Cow<_>>>()
        .join("")
}

/// Scan a document and extract render events.
pub fn get_render_tasks<'a>(
    raw_content: &'a str
) -> Vec<Render<'a>> {
    let scan = Scan::new(
        raw_content
    );

    let mut rendering = Vec::new();

    let mut checkpoint = 0;
    for event in scan {
        match event {
            Event::Begin(begin) => checkpoint = begin,
            Event::TextEnd(end) => rendering.push(Render::Text(&raw_content[checkpoint..end])),
            Event::BlockEnd(end) => {
                // Extract the language from the first line of the codeblock "```rs ..."
                if let Some(lang) = raw_content[checkpoint..end].lines().next() {
                    // Skip any newline characters after the language specifier.
                    let mut start = checkpoint + lang.len();
                    while raw_content.chars().nth(start) == Some('\r') || raw_content.chars().nth(start) == Some('\n') {
                        start += 1;
                    }
                    rendering.push(Render::CodeBlock(&raw_content[start..end], lang));
                } else {
                    rendering.push(Render::CodeBlock(&raw_content[checkpoint..end], ""));
                }
            }
        }
    }

    if raw_content.len() - 1 > checkpoint {
        rendering.push(Render::Text(&raw_content[checkpoint..raw_content.len()]));
    }
    rendering
}
