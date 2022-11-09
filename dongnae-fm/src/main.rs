
use iced::{
    button, alignment, Alignment, Button, Column, Element, Sandbox, Settings, Text, Length
};

#[derive(Default)]
struct Player {
    name: String,
    age: i32,
    picture: String // url
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NothingPressed,
}

impl Sandbox for Player {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("Player {}", self.name)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NothingPressed => {
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title())
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(title)
            .into()
    }
}

pub fn main() -> iced::Result {
    Player::run(Settings::default())
}
