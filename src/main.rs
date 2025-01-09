use image::io::Reader as ImageReader;
use image::{ImageError, RgbImage, Luma, Pixel};
use image::Rgb;

fn pixel_luminositer(img: &RgbImage, x: u32, y: u32) -> u8 {
    let pixel = img.get_pixel(x, y);
    let Luma(luminosite_) = pixel.to_luma();
    return luminosite_[0];
}

fn seuillage(img: &mut RgbImage){
    println!("Seuillage");
    println!("Luminosité est {}", pixel_luminositer(img, 0, 0));
}

fn save_img(img: &RgbImage, path: &str) -> Result<(), ImageError> {
    img.save(path)?;
    return Ok(())
}

fn image_quadrillage(img: &mut RgbImage) -> Result<(), ImageError> {
    for (x,y,color) in img.enumerate_pixels_mut() {
        if (x+y) % 2 == 0 {
            *color = Rgb([255, 255, 255]);
        }
    }
    save_img(img, "src/images/output_quadrillage.png")?;
    return Ok(())
}

fn image_noir_blanc(img: &mut RgbImage)-> Result<(), ImageError> {
    for (_x,_y,color) in img.enumerate_pixels_mut() {
        if color[0] > 128 {
            *color = Rgb([255, 255, 255]);
        } else {
            *color = Rgb([0, 0, 0]);
        }
    }
    save_img(img, "src/images/output_noir_blanc.png")?;
    return Ok(())
}

fn main() -> Result<(), ImageError> {
    let img_file = ImageReader::open("src/images/images.jpeg")?;
    let mut img: image::RgbImage = img_file.decode()?.into_rgb8();
    println!("Image dimensions: {}x{}", img.width(), img.height());
    println!("Pixel au coordonnées (32, 32): {:?}", img.get_pixel(32, 32));

    image_quadrillage(&mut img)?;
    image_noir_blanc(&mut img)?;

    return Ok(())
}
