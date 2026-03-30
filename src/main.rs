mod markdown_elements;
mod parser;
mod utils;
use iced::{
    Application, Command, Element, Length, Settings, Theme, alignment, executor, theme,
    widget::{button, column, row, scrollable, text},
};

fn main() -> iced::Result {
    Kitaab::run(Settings::default())
}

struct Kitaab {
    counter: u32,
    content: parser::ParsedContent,
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
        let md = utils::read_md("content/kitaab.md");
        let content = parser::ParsedContent::parse_md(md);
        (
            Self {
                counter: 0,
                content,
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
                self.counter += 1;
            }
            Message::Previous => {
                if self.counter > 0 {
                    self.counter -= 1;
                }
            }
            _ => {}
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
                text(format!("Page: {}", self.counter))
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
