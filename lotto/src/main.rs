use iced::{
    button, alignment, Alignment, Button, Column, Element, Sandbox, Settings, Text, Length
};

// cannot use rand in wasm
use wbg_rand::{Rng, wasm_rng};

const LOTTO_COUNT : usize     = 6;
const MAX_LOTTO_NUM  : i32  = 45;


#[derive(Default)]
struct Lotto {
    values: [i32; LOTTO_COUNT],
    create_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CreateBtnPressed,
}

impl Lotto {
    fn create_lotto(&mut self) {
        let mut bitmap : i64  = 0;
        for i in 0..6 {
            loop {
                let value = wasm_rng().gen_range(0, MAX_LOTTO_NUM);
                if (bitmap & (1 << value)) == 0 {
                    self.values[i] = value + 1;
                    bitmap |= 1 << value;
                    break;
                }
            }
        }
        self.values.sort();
    } 

    fn view_lotto(&self) -> String {
        let mut s = String::with_capacity(LOTTO_COUNT*3);

        for i in 0..6 {
            s.push_str(&(self.values[i].to_string()));
            s.push_str(" ");
        }

        s
    }
}

impl Sandbox for Lotto {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Lotto")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CreateBtnPressed => {
                Lotto::create_lotto(self);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Lotto")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);
        let lotto_string = Text::new(Lotto::view_lotto(self))
            .size(50);
        let create_btn = Button::new(&mut self.create_button, Text::new("Create"))
            .on_press(Message::CreateBtnPressed);

        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(title)
            .push(lotto_string)
            .push(create_btn)
            .into()
    }
}

pub fn main() -> iced::Result {
    Lotto::run(Settings::default())
}


