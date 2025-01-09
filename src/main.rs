use image::io::Reader as ImageReader;
use image::ImageError;
use image::Rgb;

fn pixel_luminositer(img: &RgbImage, x: u32, y: u32){
    let pixel = img.get_pixel(x, y);
    let Lima(luminosite_) = pixel.to_luma();
    return luminosite_[0];
}

fn seuillage(img: &mut RgbImage){
    println!("Seuillage");
    println!("Luminosité est {}", pixel_luminositer(img, 0, 0));
}

fn main() -> Result<(), ImageError> {
    let img_file = ImageReader::open("./src/images.jpeg")?;
    let mut img: image::RgbImage = img_file.decode()?.into_rgb8();
    println!("Image dimensions: {}x{}", img.width(), img.height());
    println!("Pixel au coordonnées (5, 12): {:?}", img.get_pixel(5, 12));

    for (x,y,color) in img.enumerate_pixels_mut() {
        // if (x+y) % 2 == 0 {
        //     *color = Rgb([255, 255, 255]);
        // }
        if color[]
    }
    img.save("src/output.png")?;

    return Ok(())
}
