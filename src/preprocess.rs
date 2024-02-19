use std::borrow::Cow;

use mdbook::{book::Book, errors::Error, preprocess::{Preprocessor, PreprocessorContext}, BookItem};

use crate::{render::{self, Render}, scan::{Event, Scan}};

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

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
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
                process_chapter(
                    raw_content
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
    raw_content: String
) -> String {
    get_render_tasks(&raw_content)
        .into_iter()
        .map(|rend| match rend {
            Render::Text(t) => t.into(),
            Render::CodeBlock(item, lang) => {
                render::render(item, lang).into()
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
                    // The "+2" is to skip the \r\n from the first line.
                    rendering.push(Render::CodeBlock(&raw_content[(checkpoint+lang.len()+2)..end], lang));
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
