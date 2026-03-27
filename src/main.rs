use iced::{
    Application, Command, Element, Settings, Theme, executor,
    widget::{button, column, row, text},
};

fn main() -> iced::Result {
    Kitaab::run(Settings::default())
}

struct Kitaab {
    counter: u32,
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
        (Self { counter: 0 }, Command::none())
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
            text("Welcome to Kitaab!"),
            row![
                button("Previous").on_press(Message::Previous),
                text(format!("Page: {}", self.counter)),
                button("Next").on_press(Message::Next)
            ],
        ]
        .into()
    }
}
