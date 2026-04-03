mod book;
mod markdown_elements;
mod page_loader;
mod parser;
use iced::{
    Application, Command, Element, Length, Settings, Theme, alignment, executor, theme,
    widget::{button, column, row, scrollable, text},
};

use crate::{book::Book, page_loader::FsPageLoader, parser::ParsedContent};

fn main() -> iced::Result {
    Kitaab::run(Settings::default())
}

struct Kitaab {
    content: parser::ParsedContent,
    book: Book<FsPageLoader>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    Noop,
}

impl Application for Kitaab {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let path = String::from("content");
        let loader = FsPageLoader::new(path);
        let book_handler = Book::new(loader);
        let content = match book_handler.load_page() {
            Some(c) => c,
            None => {
                eprintln!(
                    "Error parsing content for page {}",
                    book_handler.get_current()
                );
                // todo: replace with a proper message like "Not found" or something,
                // maybe should handle this inside book handler
                ParsedContent { content: vec![] }
            }
        };
        (
            Self {
                content,
                book: book_handler,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kitaab")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next => {
                if let Some(content) = self.book.next_page() {
                    self.content = content;
                }
            }
            Message::Previous => {
                if let Some(content) = self.book.prev_page() {
                    self.content = content;
                }
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = &self.content.content;

        let prev = button(text("<"))
            .on_press(Message::Previous)
            .style(theme::Button::Text);

        let next = button(text(">"))
            .on_press(Message::Next)
            .style(theme::Button::Text);

        column![
            text("Welcome to Kitaab!")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
            scrollable(markdown_elements::render_md(content)).height(Length::Fill),
            row![
                prev,
                text(format!("Page: {}", self.book.get_current()))
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
                next,
            ]
            .width(Length::Fill)
            .align_items(iced::Alignment::Center),
        ]
        .width(Length::Fill)
        .padding(20)
        .spacing(20)
        .into()
    }
}
