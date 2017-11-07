extern crate image;

use self::image::ImageBuffer;
use self::image::math::utils::clamp;

pub fn render_fractal_data(input_data: &[u8], counting_data: &[u32], side_length: usize) {
	let mut data_img = ImageBuffer::new(side_length as u32, side_length as u32);
	for ((_,_, pixel), (fractal_value, counting_value)) in data_img.enumerate_pixels_mut().zip(input_data.iter().zip(counting_data)) {
		let red = clamp_to_u8(256f64 * ring_percent(*fractal_value as u32, 4));
		let green = clamp_to_u8(256f64 * ring_percent(*counting_value, 3) * 0.9f64);
		let blue = clamp_to_u8(256f64 * ring_percent(*fractal_value as u32, 4) + 40f64);

		*pixel = if red == 255 || (red == 0 && green == 0) {
			image::Rgb([0,0,0])
		} else {
			image::Rgb([
				red,
				green,
				blue,
			])
		}
	}

	data_img.save("output.png").unwrap();
}

fn ring_percent(val: u32, ring: u32) -> f64 {
	(val % ring) as f64 / ((ring - 1) as f64)
}

fn clamp_to_u8(val: f64) -> u8 {
	clamp(val, 0f64, 255f64) as u8
}


