
use crate::common::FractalResult;
use crate::compute;
use crate::render;
use crate::render::ColorChannel;
use std::sync::Arc;

use iced::{
    button, image, slider, text_input, 
    Application, Background, Button, Color, Column, Command, Container, Element, HorizontalAlignment, Image, Length, Row, Slider, Text, TextInput,
};

#[derive(Default)]
pub struct FractalGUI {
    ui_state: UIData,
    compute_params: compute::ComputeParams,
    render_params: render::RenderParams,
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
    ColorChanged(SliderColor, ColorChannel, f32),
    BeginComputingFractal,
    FractalComputed(Arc<FractalResult>),
    FractalRendered(image::Handle),
}

#[derive(Debug, Clone)]
pub enum SliderColor {
    Color0,
    Color1,
    Color2,
    Color3,
}

#[derive(Default)]
struct UIData {
    compute_button: button::State,
    initial_size_text: text_input::State,
    background_color_red_slider: slider::State,
    background_color_green_slider: slider::State,
    background_color_blue_slider: slider::State,
    color1_red_slider: slider::State,
    color1_green_slider: slider::State,
    color1_blue_slider: slider::State,
    color2_red_slider: slider::State,
    color2_green_slider: slider::State,
    color2_blue_slider: slider::State,
    color3_red_slider: slider::State,
    color3_green_slider: slider::State,
    color3_blue_slider: slider::State,
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
            Message::ColorChanged(which_color, channel, value) => {
                match which_color {
                    SliderColor::Color0 => self.render_params.color0.set_normalized(channel, value),
                    SliderColor::Color1 => self.render_params.color1.set_normalized(channel, value),
                    SliderColor::Color2 => self.render_params.color2.set_normalized(channel, value),
                    SliderColor::Color3 => self.render_params.color3.set_normalized(channel, value),
                }
                if let Some(data) = &self.fractal_data {
                    if self.state == State::Idle {
                        Command::perform(render::render_fractal(self.render_params.clone(), Arc::clone(data)), Message::FractalRendered)
                    } else {
                        Command::none()
                    }
                }
                else {
                    Command::none()
                }
            },
            Message::BeginComputingFractal => {
                self.state = State::Computing;
                Command::perform(compute::compute_fractal(self.compute_params.clone()), Message::FractalComputed)
            },
            Message::FractalComputed(result) => {
                self.fractal_data = Some(Arc::clone(&result));
                self.state = State::Rendering;
                Command::perform(render::render_fractal(self.render_params.clone(), result), Message::FractalRendered)
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
            render_params,
            fractal_data: _,
            fractal_image,
            state,
        } = self;



        let initial_size_text = TextInput::new(
            &mut ui_state.initial_size_text,
            "Initial Count",
            &compute_params.initial_size,
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
                .width(Length::Fill)
                .spacing(10)
                .push(Text::new("Initial Count")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(initial_size_text)
                .push(
                    button(&mut ui_state.compute_button, "Compute", *state == State::Idle, Message::BeginComputingFractal),
                )
                .push(Text::new("Background Color")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(Text::new("R").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.background_color_red_slider, 0.0..=1.0, render_params.color0.get_normalized(ColorChannel::Red), |val| Message::ColorChanged(SliderColor::Color0, ColorChannel::Red, val)))
                    .push(Text::new("G").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.background_color_green_slider, 0.0..=1.0, render_params.color0.get_normalized(ColorChannel::Green), |val| Message::ColorChanged(SliderColor::Color0, ColorChannel::Green, val)))
                    .push(Text::new("B").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.background_color_blue_slider, 0.0..=1.0, render_params.color0.get_normalized(ColorChannel::Blue), |val| Message::ColorChanged(SliderColor::Color0, ColorChannel::Blue, val)))
                )
                .push(Text::new("Color (Value=1)")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(Text::new("R").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color1_red_slider, 0.0..=1.0, render_params.color1.get_normalized(ColorChannel::Red), |val| Message::ColorChanged(SliderColor::Color1, ColorChannel::Red, val)))
                    .push(Text::new("G").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color1_green_slider, 0.0..=1.0, render_params.color1.get_normalized(ColorChannel::Green), |val| Message::ColorChanged(SliderColor::Color1, ColorChannel::Green, val)))
                    .push(Text::new("B").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color1_blue_slider, 0.0..=1.0, render_params.color1.get_normalized(ColorChannel::Blue), |val| Message::ColorChanged(SliderColor::Color1, ColorChannel::Blue, val)))
                )
                .push(Text::new("Color (Value=2)")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(Text::new("R").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color2_red_slider, 0.0..=1.0, render_params.color2.get_normalized(ColorChannel::Red), |val| Message::ColorChanged(SliderColor::Color2, ColorChannel::Red, val)))
                    .push(Text::new("G").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color2_green_slider, 0.0..=1.0, render_params.color2.get_normalized(ColorChannel::Green), |val| Message::ColorChanged(SliderColor::Color2, ColorChannel::Green, val)))
                    .push(Text::new("B").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color2_blue_slider, 0.0..=1.0, render_params.color2.get_normalized(ColorChannel::Blue), |val| Message::ColorChanged(SliderColor::Color2, ColorChannel::Blue, val)))
                )
                .push(Text::new("Color (Value=3)")
                    .color([0.1, 0.1, 0.1])
                    .horizontal_alignment(HorizontalAlignment::Center)
                )
                .push(Row::new()
                    .width(Length::Fill)
                    .spacing(5)
                    .push(Text::new("R").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color3_red_slider, 0.0..=1.0, render_params.color3.get_normalized(ColorChannel::Red), |val| Message::ColorChanged(SliderColor::Color3, ColorChannel::Red, val)))
                    .push(Text::new("G").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color3_green_slider, 0.0..=1.0, render_params.color3.get_normalized(ColorChannel::Green), |val| Message::ColorChanged(SliderColor::Color3, ColorChannel::Green, val)))
                    .push(Text::new("B").color([0.1, 0.1, 0.1]).horizontal_alignment(HorizontalAlignment::Right))
                    .push(Slider::new(&mut ui_state.color3_blue_slider, 0.0..=1.0, render_params.color3.get_normalized(ColorChannel::Blue), |val| Message::ColorChanged(SliderColor::Color3, ColorChannel::Blue, val)))
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
