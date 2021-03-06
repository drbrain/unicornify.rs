use image::DynamicImage;

use std::env::args;

use unicornify::Avatar;

fn main() {
    let hash = match args().nth(1) {
        Some(h) => h,
        None => String::from("58479f76374a3ba3c69b9804163f39f4"),
    };
    let quadrant = match args().nth(2) {
        Some(q) => Some(q.parse::<u8>().unwrap()),
        None => None,
    };

    let avatar = Avatar::new(hash, false).unwrap();
    let image_buffer = avatar.draw(128, quadrant, true, false, false, false, false);

    let image = DynamicImage::ImageRgba8(image_buffer);
    image.save("out.png").unwrap();
}
