mod parser;
use iced::{
    Application, Command, Element, Length, Settings, Theme, alignment, executor,
    widget::{button, column, row, text},
};

fn main() -> iced::Result {
    Kitaab::run(Settings::default())
}

struct Kitaab {
    counter: u32,
    content: parser::ParsedContent,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
}

impl Application for Kitaab {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let md = String::from("# Hello world \nWhats up?? \n###Yessir");
        let content = parser::ParsedContent::parse_md(md);
        dbg!(&content);
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
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Welcome to Kitaab!")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
            row![
                button("Previous").on_press(Message::Previous),
                text(format!("Page: {}", self.counter))
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
                button("Next").on_press(Message::Next),
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
