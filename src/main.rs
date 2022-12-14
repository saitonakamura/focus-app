use iced::widget::{column, progress_bar, slider, button, text};
use iced::{Element, Sandbox, Settings};
use windows::{
    UI::Shell::*
};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

// impl Default for windows::core::Result<FocusSessionManager> {
//     fn default() -> Self {
//         todo!()
//     }
// }


// #[derive(Default)]
struct MyApp {
    value: f32,
    manager: Option<windows::core::Result<FocusSessionManager>>,
    // session: FocusSession,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
    Click,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { value: Default::default(), manager: None }
    }
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
            Message::Click => {
                let manager = Some(FocusSessionManager::GetDefault());
                match manager {
                    Some(Ok(manager)) => {
                        match manager.IsFocusActive() {
                            Ok(true) => self.value = 25.0,
                            Ok(false) => self.value = 50.0,
                            Err(_) => self.value = 75.0
                        }
                    },
                    Some(Err(_)) => self.value -= 1.0,
                    None => self.value -= 1.0
                }
                // match FocusSessionManager::GetDefault() {
                //     Ok(manager) => {
                //         self.manager = manager;
                //         match self.manager.TryStartFocusSession() {
                //             Ok(session) => self.session = session,
                //             Err(_) => self.value -= 2.0
                //         }
                //     },
                //     Err(_) => self.value -= 1.0
                // }
                self.value += 1.0
            },
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            progress_bar(0.0..=100.0, self.value),
            slider(0.0..=100.0, self.value, Message::SliderChanged).step(0.01),
            button("Click me").on_press(Message::Click),
            // text()
        ]
        .padding(20)
        .into()
    }
}