use argh::FromArgs;
use image::{ImageError, RgbImage, Rgb};
use image::io::Reader as ImageReader;
use std::str::FromStr;

mod traitement_image;

use traitement_image::{image_noir_blanc, save_img, image_deux_couleur, image_palette, image_tramage_aleatoire, ordered_dithering_rgb};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    BiColor(OptsBiColor),
    TramageAleatoire(OptsTramageAleatoire),
    Bayer(OptsBayer)
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "bayer")]
/// Rendu de l’image par seuillage monochrome.
struct OptsBayer {
    /// le nombre de couleurs à utiliser
    #[argh(option)]
    order: u32,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage_aleatoire")]
/// Rendu de l’image par seuillage monochrome.
struct OptsTramageAleatoire {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs.
struct OptsPalette {
    /// le nombre de couleurs à utiliser
    #[argh(option)]
    n_couleurs: usize,

    /// les couleurs spécifiques
    #[argh(option)]
    #[argh(long = "couleurs")]
    couleurs: Vec<Couleur>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "bicolor")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs.
struct OptsBiColor {

    /// les couleurs spécifiques
    #[argh(option)]
    #[argh(long = "couleurs")]
    couleurs: Vec<Couleur>,
}

#[derive(Debug, Clone, PartialEq)]
enum Couleur {
    Noir,
    Blanc,
    Rouge,
    Vert,
    Bleu,
    Jaune,
    Cyan,
    Magenta,
}

impl FromStr for Couleur {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Couleur::Noir),
            "blanc" => Ok(Couleur::Blanc),
            "rouge" => Ok(Couleur::Rouge),
            "vert" => Ok(Couleur::Vert),
            "bleu" => Ok(Couleur::Bleu),
            "jaune" => Ok(Couleur::Jaune),
            "cyan" => Ok(Couleur::Cyan),
            "magenta" => Ok(Couleur::Magenta),
            _ => Err(format!("Couleur inconnue: {}", s)),
        }
    }
}

impl Couleur {
    fn rgb(&self) -> Rgb<u8> {
        match self {
            Couleur::Noir => Rgb([0, 0, 0]),
            Couleur::Blanc => Rgb([255, 255, 255]),
            Couleur::Rouge => Rgb([255, 0, 0]),
            Couleur::Vert => Rgb([0, 255, 0]),
            Couleur::Bleu => Rgb([0, 0, 255]),
            Couleur::Jaune => Rgb([255, 255, 0]),
            Couleur::Cyan => Rgb([0, 255, 255]),
            Couleur::Magenta => Rgb([255, 0, 255]),
        }
    }
}


fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let input_path = args.input;
    let output_path = args.output.unwrap_or_else(|| "output.png".to_string());

    let mut img = load_image(&input_path)?;

    match args.mode {
        Mode::Seuil(_) => {
            let new_image = image_noir_blanc(&mut img)?;
            save_img(&new_image, &output_path)?;
        }
        Mode::BiColor(opts) => {
            println!("{}", opts.couleurs.len().to_string());
            if opts.couleurs.len() == 0 {
                let new_image = image_noir_blanc(&mut img)?;
                save_img(&new_image, &output_path)?;
            }
            else if opts.couleurs.len() != 2 {
                eprintln!("Erreur : le mode bicolore nécessite deux couleurs");
                std::process::exit(1);
            }
            else {
                let new_image = image_deux_couleur(&mut img, opts.couleurs[0].rgb(), opts.couleurs[1].rgb())?;
                save_img(&new_image, &output_path)?;
            }
        }
        Mode::Palette(opts) => {
            if opts.couleurs.len() != opts.n_couleurs {
                eprintln!(
                    "Erreur : le nombre de couleurs spécifiées ({}) ne correspond pas à n_couleurs ({})",
                    opts.couleurs.len(),
                    opts.n_couleurs
                );
                std::process::exit(1);
            }
            else {
                let new_image = image_palette(&mut img, &opts.couleurs.iter().map(|c| c.rgb()).collect::<Vec<_>>())?;
                save_img(&new_image, &output_path)?;
            }
            println!(
                "Mode palette sélectionné avec {} couleurs : {:?}",
                opts.n_couleurs, opts.couleurs
            );
        }
        Mode::TramageAleatoire(_) => {
            let new_image = image_tramage_aleatoire(&mut img)?;
            save_img(&new_image, &output_path)?;
        }
        Mode::Bayer(opts) => {
            let new_image = ordered_dithering_rgb(&mut img, opts.order)?;
            save_img(&new_image, &output_path)?;
        }

    }

    Ok(())
}

fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let img_file = ImageReader::open(path)?;
    Ok(img_file.decode()?.into_rgb8())
}
