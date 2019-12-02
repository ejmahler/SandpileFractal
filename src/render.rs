
use image::{DynamicImage, ImageBuffer, RgbImage, ImageOutputFormat};
use iced::image::Handle;
use std::io::Cursor;
use std::sync::Arc;
use crate::common::FractalResult;


pub async fn render_fractal(fractal_data: Arc<FractalResult>) -> Handle {
    let result_image = DynamicImage::ImageRgb8(create_fractal_image(&fractal_data.sand_data, &fractal_data.count_data, fractal_data.side_length));
    let mut cursor = Cursor::new(Vec::new());
    result_image.write_to(&mut cursor, ImageOutputFormat::PNG).expect("Failed to encode image data to memory");
    Handle::from_bytes(cursor.into_inner())
}

fn create_fractal_image(input_data: &[u8], counting_data: &[u32], side_length: usize) -> RgbImage {
	let mut data_img = ImageBuffer::new(side_length as u32, side_length as u32);
	for ((_,_, pixel), (fractal_value, _)) in data_img.enumerate_pixels_mut().zip(input_data.iter().zip(counting_data)) {
		*pixel = match fractal_value {
			0 => image::Rgb([0,0,0]),
			1 => image::Rgb([64,64,255]),
			2 => image::Rgb([255,255,64]),
			_ => image::Rgb([255,64,64]),
		}
	}
	data_img
}

