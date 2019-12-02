
use crate::common::FractalResult;
use crate::compute;
use std::sync::Arc;

use iced::{
    button, image, text_input,
    Application, Background, Button, Color, Column, Command, Container, Element, HorizontalAlignment, Image, Length, Row, Text, TextInput,
};

#[derive(Default)]
pub struct FractalGUI {
    ui_state: UIData,
    compute_params: compute::ComputeParams,
    fractal_data: Option<Arc<FractalResult>>,
    fractal_image: Option<image::Handle>,
    state: State,
}

#[derive(PartialEq)]
enum State {
    Idle,
    Computing,
    Rendering,
}

impl Default for State {
    fn default() -> State {
        State::Idle
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InitialSizeChanged(String),
    BeginComputingFractal,
    FractalComputed(Arc<FractalResult>),
    FractalRendered(image::Handle),
}

#[derive(Default)]
struct UIData {
    compute_button: button::State,
    initial_size_text: text_input::State,
}

impl Application for FractalGUI {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Abelian Sandpile Fractal")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InitialSizeChanged(value) => {
                self.compute_params.initial_size = value;
                Command::none()
            },
            Message::BeginComputingFractal => {
                self.state = State::Computing;
                Command::perform(compute::compute_fractal(self.compute_params.clone()), Message::FractalComputed)
            },
            Message::FractalComputed(result) => {
                self.fractal_data = Some(Arc::clone(&result));
                self.state = State::Rendering;
                Command::perform(crate::render::render_fractal(result), Message::FractalRendered)
            }
            Message::FractalRendered(result) => {
                self.fractal_image = Some(result);
                self.state = State::Idle;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let FractalGUI {
            ui_state,
            compute_params,
            fractal_data: _,
            fractal_image,
            state,
        } = self;



        let initial_size_text = TextInput::new(
            &mut ui_state.initial_size_text,
            "Initial Count",
            &mut compute_params.initial_size,
            |mut value| {
                value.retain(|c| c.is_digit(10));
                Message::InitialSizeChanged(value)
            }
        )
        .padding(15)
        .size(30);

       
        let content = Row::new()
            .width(Length::Fill)
            .spacing(20)
            .push(Column::new()
                .width(Length::Units(200))
                .spacing(10)
                .push(Text::new("Initial Count")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(initial_size_text)
                .push(
                    button(&mut ui_state.compute_button, "Compute", *state == State::Idle, Message::BeginComputingFractal),
                )
            );

        let content = if let Some(image_handle) = fractal_image {
            content.push(Image::new(image_handle.clone()).width(Length::Fill).height(Length::Fill))
        }
        else {
            content
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
    clickable: bool, 
    press_message: Message
) -> Button<'a, Message> {
    let button = Button::new(
        state,
        Text::new(label)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color(Color::WHITE)
            .size(30),
    )
    .padding(10)
    .min_width(100)
    .border_radius(10)
    .background(Background::Color(if clickable { Color::from_rgb8(0x89, 0x80, 0xF5) } else { Color::from_rgb8(0xa0, 0xa0, 0xa0) }));

    if clickable {
        button.on_press(press_message)
    }
    else {
        button
    }
}
