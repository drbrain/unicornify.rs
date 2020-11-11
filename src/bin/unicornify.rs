use unicornify::Avatar;

use image::DynamicImage;

fn main() {
    let hash = String::from("58479f76374a3ba3c69b9804163f39f4");
    let avatar = Avatar::new(hash, false).unwrap();
    let image_buffer = avatar.draw(128, true, false, false, false);

    let image = DynamicImage::ImageRgba8(image_buffer);
    image.save("out.png").unwrap();
}
