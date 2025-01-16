use argh::FromArgs;

mod traitement_image;

use traitement_image::{image_noir_blanc, save_img};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette rÃ©duite de couleurs.
struct DitherArgs {

    /// le fichier dâentrÃ©e
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode dâopÃ©ration
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de lâimage par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de lâimage avec une palette contenant un nombre limitÃ© de couleurs
struct OptsPalette {

    /// le nombre de couleurs Ã  utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
    
    #[argh(option), long="couleurs"]
    couleurs: Vec<Couleur>
}

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let input_path = args.input;
    let output_path = args.output.unwrap_or_else(|| "output.png".to_string());

    let mut img = load_image(&input_path)?;

    match args.mode {
        Mode::Seuil(_) => {
            image_noir_blanc(&mut img);
            save_img(&img, &output_path)?;
        }
        Mode::Palette(opts) => {
            println!("Mode palette non encore implémenté. Couleurs demandées : {:?} avec {} couleurs.", opts.couleurs, opts.n_couleurs);
            // Implémentation supplémentaire ici pour gérer le mode palette.
        }
    }

    Ok(())
}

fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let img_file = ImageReader::open(path)?;
    Ok(img_file.decode()?.into_rgb8())
}