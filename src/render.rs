
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use image::math::utils::clamp;
use iced::image::Handle;
use std::io::Cursor;
use std::sync::Arc;
use crate::common::FractalResult;

#[derive(Clone, Debug)]
pub enum ColorChannel {
	Red,
	Green,
	Blue
}

#[derive(Clone, Debug)]
pub struct RenderColor(image::Rgb<u8>);

impl RenderColor {
	pub fn get_normalized(&self, channel: ColorChannel) -> f32 {
		let value = match channel {
			ColorChannel::Red => self.0[0],
			ColorChannel::Green => self.0[1],
			ColorChannel::Blue => self.0[2],
		};
		f32::from(value) / 255.0
	}
	pub fn set_normalized(&mut self, channel: ColorChannel, value: f32) {
		let clamped_value = clamp(value * 255.0, 0.0, 255.0);
		let converted_value = clamped_value as u8;
		match channel {
			ColorChannel::Red => self.0[0] = converted_value,
			ColorChannel::Green => self.0[1] = converted_value,
			ColorChannel::Blue => self.0[2] = converted_value,
		}
	}
}

#[derive(Clone, Debug)]
pub struct RenderParams {
	pub color0: RenderColor,
	pub color1: RenderColor,
	pub color2: RenderColor,
	pub color3: RenderColor,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            color0: RenderColor(image::Rgb([0,0,0])),
            color1: RenderColor(image::Rgb([64,64,255])),
            color2: RenderColor(image::Rgb([255,255,64])),
            color3: RenderColor(image::Rgb([255,64,64])),
        }
    }
}


pub async fn render_fractal(params: RenderParams, fractal_data: Arc<FractalResult>) -> Handle {
    let mut data_img = ImageBuffer::new(fractal_data.side_length as u32, fractal_data.side_length as u32);
	for ((_,_, pixel), (fractal_value, _)) in data_img.enumerate_pixels_mut().zip(fractal_data.sand_data.iter().zip(fractal_data.count_data.iter())) {
		*pixel = match fractal_value {
			0 => params.color0.0,
			1 => params.color1.0,
			2 => params.color2.0,
			_ => params.color3.0,
		}
	}
    let mut cursor = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(data_img).write_to(&mut cursor, ImageOutputFormat::PNG).expect("Failed to encode image data to memory");
    Handle::from_bytes(cursor.into_inner())
}

