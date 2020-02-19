use std::io::Write;

fn main()
{
	let input = std::env::args().nth(1).expect("input png");

	let png;
	match image::open(&input).expect("opening")
	{
		image::DynamicImage::ImageRgb8(image)
			=> png = image,
		_ => panic!("image type not supported"),
	}

	let rows = png.rows()
		.map(
			|row|
			{
				row
					.map(
						|c|
							(c[0],c[1],c[2])
					)
					.collect()
			}
		);

	let data = libavif::encode_rgb(
		png.width(),
		png.height(),
		rows,
		0,
	).expect("encoding avif");

	std::io::stdout().write_all(&data).expect("output avif");
}