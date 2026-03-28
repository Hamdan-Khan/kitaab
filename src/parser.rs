use pulldown_cmark::{Event, Parser, Tag};
use std::mem;

#[derive(Debug)]
enum MdElement {
    Paragraph(String),
    Heading { level: u32, content: String },
    CodeBlock(String),
}

enum BufferState {
    Idle,
    Paragraph(String),
    Heading { level: u32, content: String },
    CodeBlock(String),
}

#[derive(Debug)]
pub struct ParsedContent {
    content: Vec<MdElement>,
}

impl ParsedContent {
    pub fn parse_md(content: String) -> Self {
        let mut state: BufferState = BufferState::Idle;
        let mut parsed: Vec<MdElement> = vec![];

        ParsedContent::debug_parsed(&content);

        for event in Parser::new(&content) {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Paragraph => state = BufferState::Paragraph(String::new()),
                    Tag::Heading(lvl) => {
                        state = BufferState::Heading {
                            level: lvl,
                            content: String::new(),
                        }
                    }
                    Tag::CodeBlock(_) => state = BufferState::CodeBlock(String::new()),
                    _ => {}
                },
                Event::Text(text) => match &mut state {
                    BufferState::Paragraph(buf) => *buf += &text,
                    BufferState::Heading { level: _, content } => *content = text.to_string(),
                    BufferState::CodeBlock(buf) => *buf = text.to_string(),
                    _ => {}
                },
                Event::SoftBreak => match &mut state {
                    BufferState::Paragraph(buf) => *buf += "\n",
                    _ => {}
                },
                Event::End(tag) => match tag {
                    Tag::Paragraph => {
                        if let BufferState::Paragraph(text) =
                            mem::replace(&mut state, BufferState::Idle)
                        {
                            parsed.push(MdElement::Paragraph(text));
                        }
                    }
                    Tag::Heading(_) => {
                        if let BufferState::Heading { level, content } =
                            mem::replace(&mut state, BufferState::Idle)
                        {
                            parsed.push(MdElement::Heading { level, content })
                        }
                    }
                    Tag::CodeBlock(_) => {
                        if let BufferState::CodeBlock(text) =
                            mem::replace(&mut state, BufferState::Idle)
                        {
                            parsed.push(MdElement::CodeBlock(text))
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Self { content: parsed }
    }

    fn debug_parsed(content: &String) {
        for event in Parser::new(&content) {
            match &event {
                Event::Start(tag) => println!("Start: {:?}", tag),
                Event::End(tag) => println!("End: {:?}", tag),
                Event::Html(s) => println!("Html: {:?}", s),
                Event::Text(s) => println!("Text: {:?}", s),
                Event::Code(s) => println!("Code: {:?}", s),
                Event::FootnoteReference(s) => println!("FootnoteReference: {:?}", s),
                Event::TaskListMarker(b) => println!("TaskListMarker: {:?}", b),
                Event::SoftBreak => println!("SoftBreak"),
                Event::HardBreak => println!("HardBreak"),
                Event::Rule => println!("Rule"),
            };
        }
    }
}
