use iced::{
    widget::{button, Button, Column, container, Container, Text},
    Alignment, Border, Color, Element, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    selected: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed(usize),
}

struct AppBackground;  // Spostato fuori dall'impl Sandbox

impl container::StyleSheet for AppBackground {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgb8(30, 30, 40).into()),
            border: Border::default(),
            text_color: None,
            shadow: Default::default(),
        }
    }
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self { selected: None }
    }

    fn title(&self) -> String {
        String::from("Pokémon Shiny Hunt GUI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(index) => {
                self.selected = Some(index);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar_width = Length::Fixed(200.0);

        let games = vec![
            "Red / Blue / Yellow",
            "Gold / Silver / Crystal",
            "FireRed / LeafGreen",
            "Ruby / Sapphire / Emerald",
            "Diamond / Pearl / Platinum",
            "Black / White",
            "Black 2 / White 2",
            "X / Y",
            "Omega Ruby / Alpha Sapphire",
            "Sun / Moon ",
            "Ultra Sun / Ultra Moon",
            "Sword / Shield",
            "Scarlet / Violet",
        ];

        let mut col = Column::new()
            .width(sidebar_width)
            .padding(10)
            .spacing(10)
            .align_items(Alignment::Start);

        for (i, game) in games.iter().enumerate() {
            let is_selected = Some(i) == self.selected;
            let btn = Button::new(Text::new(*game))
                .style(if is_selected {
                    iced::theme::Button::Custom(Box::new(SelectedButtonStyle))
                } else {
                    iced::theme::Button::Custom(Box::new(NormalButtonStyle))
                })
                .on_press(Message::ButtonPressed(i))
                .width(Length::Fill);

            col = col.push(btn);
        }

        let sidebar = Container::new(col)
            .width(sidebar_width)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(BlackBackground)));

        let content = Container::new(Text::new(
            self.selected
                .map(|i| format!("Selected: {}", games[i]))
                .unwrap_or_else(|| "Select a Pokémon game".into()),
        ))
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill);

        Container::new(
            iced::widget::row![sidebar, content]
                .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(AppBackground)))
        .into()
    }
}

struct SelectedButtonStyle;

impl button::StyleSheet for SelectedButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb8(52, 122, 235).into()),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: Color::TRANSPARENT,
            },
            text_color: style.palette().text,
            shadow_offset: Default::default(),
            shadow: Default::default(),
        }
    }
}

struct NormalButtonStyle;

impl button::StyleSheet for NormalButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::from_rgb8(235, 52, 229).into()),
            border: Border {
                radius: 5.0.into(),
                width: 1.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::WHITE,
            shadow_offset: Default::default(),
            shadow: Default::default(),
        }
    }
}

struct BlackBackground;

impl container::StyleSheet for BlackBackground {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::BLACK.into()),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
}