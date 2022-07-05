use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Pixel};

fn main() {
    let image = gradient(40, 40);

    image.save("output/image.png").unwrap();
}

fn gradient(width: i8, height: i8) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(40, 40);

    for y in 0..height {
        for x in 0..width {
            let pixel = image::Rgb([255, 0, 0]);
            image.put_pixel(x as u32, y as u32, pixel);
        }
    }

    image
}
