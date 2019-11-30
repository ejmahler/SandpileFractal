
use image::ImageBuffer;

pub fn render_fractal_data(input_data: &[u8], counting_data: &[u32], side_length: usize) {
	let mut data_img = ImageBuffer::new(side_length as u32, side_length as u32);
	for ((_,_, pixel), (fractal_value, _)) in data_img.enumerate_pixels_mut().zip(input_data.iter().zip(counting_data)) {
		*pixel = match fractal_value {
			0 => image::Rgb([0,0,0]),
			1 => image::Rgb([64,64,255]),
			2 => image::Rgb([255,255,64]),
			_ => image::Rgb([255,64,64]),
		}
	}

	data_img.save("output.png").unwrap();
}

