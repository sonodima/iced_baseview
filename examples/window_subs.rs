use iced_baseview::{
    executor, settings, Align, Application, Color, Column, Command, Container,
    Element, Length, Parent, Runner, Settings, Subscription, Text,
    WindowScalePolicy, WindowSubs,
};
use std::time::{Duration, Instant};

static COUNT_INTERVAL: Duration = Duration::from_millis(10);

fn main() {
    let settings = Settings {
        window: settings::Window {
            title: String::from("iced_baseview window subscriptions"),
            logical_size: (500, 300),
            scale_policy: WindowScalePolicy::SystemScaleFactor,
        },
        flags: (),
    };

    let (_, opt_app_runner) = Runner::<MyProgram>::open(settings, Parent::None);

    opt_app_runner.unwrap().app_run_blocking();
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnFrame,
    WillClose,
}

struct MyProgram {
    next_interval: Instant,
    count: usize,
}

impl Application for MyProgram {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                next_interval: Instant::now() + COUNT_INTERVAL,
                count: 0,
            },
            Command::none(),
        )
    }

    fn subscription(
        &self,
        window_subs: &mut WindowSubs<Message>,
    ) -> Subscription<Message> {
        window_subs.on_frame = Some(Message::OnFrame);
        window_subs.on_window_will_close = Some(Message::WillClose);
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OnFrame => {
                let now = Instant::now();
                while now >= self.next_interval {
                    self.next_interval += COUNT_INTERVAL;
                    self.count += 1;
                }
            }
            Message::WillClose => {
                println!("The window will close!");
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(Text::new(format!("{}", self.count)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }
}
