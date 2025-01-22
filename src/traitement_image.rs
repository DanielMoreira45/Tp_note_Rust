use image::{ImageError, RgbImage, Luma, Pixel, Rgb};
use image::error::{ParameterError, ParameterErrorKind};
use rand::Rng;
use std::error::Error;

fn pixel_luminositer(img: &RgbImage, x: u32, y: u32) -> u8 {
    let pixel = img.get_pixel(x, y);
    let Luma(luminosite_) = pixel.to_luma();
    luminosite_[0]
}

pub fn save_img(img: &RgbImage, path: &str) -> Result<(), ImageError> {
    img.save(path)?;
    Ok(())
}

pub fn image_quadrillage(img: &mut RgbImage) -> Result<RgbImage, ImageError> {
    for (x, y, color) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *color = Rgb([255, 255, 255]);
        }
    }
    Ok(img.clone())
}

pub fn image_noir_blanc(img: &mut RgbImage) -> Result<RgbImage, ImageError> {
    for (_x, _y, color) in img.enumerate_pixels_mut() {
        let Rgb(data) = *color;
        
        if 0.2126 * data[0] as f32 + 0.7152 * data[1] as f32 + 0.0722 * data[2] as f32 > 128.0 {
            *color = Rgb([255, 255, 255]);
        } else {
            *color = Rgb([0, 0, 0]);
        }
    }
    Ok(img.clone())
}

pub fn image_deux_couleur(img: &mut RgbImage, couleur1: Rgb<u8>, couleur2: Rgb<u8>) -> Result<RgbImage, ImageError> {
    for (_x, _y, color) in img.enumerate_pixels_mut() {
        let Rgb(data) = *color;
        
        if 0.2126 * data[0] as f32 + 0.7152 * data[1] as f32 + 0.0722 * data[2] as f32 > 128.0 {
            *color = couleur1;
        } else {
            *color = couleur2;
        }
    }
    Ok(img.clone())
}

pub fn image_palette(img: &mut RgbImage, palette: &[Rgb<u8>])-> Result<RgbImage, ImageError> {
    fn euclidean_distance(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f32 {
        let Rgb(data1) = c1;
        let Rgb(data2) = c2;
        ((data1[0] as f32 - data2[0] as f32).powi(2)
            + (data1[1] as f32 - data2[1] as f32).powi(2)
            + (data1[2] as f32 - data2[2] as f32).powi(2))
            .sqrt()
    }

    if palette.is_empty() {
        return Err(ImageError::Parameter(ParameterError::from_kind(
            ParameterErrorKind::Generic(String::from("Palette is empty")),
        )));
    }

    for (_x, _y, color) in img.enumerate_pixels_mut() {
        let mut closest_color = &palette[0];
        let mut min_distance = euclidean_distance(color, &palette[0]);

        for palette_color in palette.iter().skip(1) {
            let distance = euclidean_distance(color, palette_color);
            if distance < min_distance {
                min_distance = distance;
                closest_color = palette_color;
            }
        }

        *color = *closest_color;
    }

    Ok(img.clone())
}

pub fn image_tramage_aleatoire(img: &mut RgbImage) -> Result<RgbImage, image::ImageError> {
    let mut rng = rand::thread_rng();

    for (_x, _y, color) in img.enumerate_pixels_mut() {
        let Rgb(data) = *color;
        let luminosity = 0.2126 * data[0] as f32 + 0.7152 * data[1] as f32 + 0.0722 * data[2] as f32;
        let threshold = rng.gen_range(0.0..255.0);

        if luminosity > threshold {
            *color = Rgb([255, 255, 255]);
        } else {
            *color = Rgb([0, 0, 0]);
        }
    }

    Ok(img.clone())
}

fn generate_bayer_matrix(order: u32) -> Vec<Vec<u32>> {
    if order == 0 {
        return vec![vec![0]]; // B0
    }

    let prev = generate_bayer_matrix(order - 1);
    let size = prev.len();
    let mut bayer = vec![vec![0; size * 2]; size * 2];

    for i in 0..size {
        for j in 0..size {
            let value = prev[i][j];
            bayer[i][j] = 4 * value;
            bayer[i][j + size] = 4 * value + 3;
            bayer[i + size][j] = 4 * value + 2;
            bayer[i + size][j + size] = 4 * value + 1;
        }
    }

    bayer
}

pub fn ordered_dithering_rgb(img: &RgbImage, order: u32) -> Result<RgbImage, ImageError> {
    let bayer_matrix = generate_bayer_matrix(order);
    let matrix_size = bayer_matrix.len();
    let max_value = bayer_matrix[matrix_size - 1][matrix_size - 1];

    // Normaliser la matrice de Bayer entre 0 et 1
    let bayer_matrix: Vec<Vec<f32>> = bayer_matrix
        .into_iter()
        .map(|row| row.into_iter().map(|x| x as f32 / max_value as f32).collect())
        .collect();

    let mut dithered_img = img.clone();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let Rgb(data) = *pixel;
            let luminosity = (0.2126 * data[0] as f32 + 0.7152 * data[1] as f32 + 0.0722 * data[2] as f32) / 255.0;

            let threshold = bayer_matrix[y as usize % matrix_size][x as usize % matrix_size];

            // Appliquer le seuil et définir le pixel en noir ou blanc
            let new_value = if luminosity > threshold {
                255 // Blanc
            } else {
                0 // Noir
            };

            // Remplacer le pixel par blanc ou noir
            dithered_img.put_pixel(x, y, Rgb([new_value, new_value, new_value]));
        }
    }

    Ok(dithered_img)
}

