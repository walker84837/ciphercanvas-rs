use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use log::{info, warn};
use qrcode::{render::svg, EcLevel, QrCode};
use resvg::render;
use std::{
    fmt,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use tiny_skia::{Pixmap, Transform};
use toml::Value;
use usvg::{fontdb, Options, Tree};

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "The WiFi network's SSID (its name)")]
    ssid: String,

    #[arg(
        short,
        long,
        default_value = "WPA",
        help = "The encryption of the WiFi network. It can be 'WPA' or 'WEP'."
    )]
    encryption: Encryption,

    #[arg(short, long, help = "The TOML configuration file.")]
    config: PathBuf,

    #[arg(short, help = "The output file to export the QR code to.")]
    output: PathBuf,
}

#[derive(ValueEnum, Clone)]
enum Encryption {
    WPA,
    WEP,
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encryption_str = match self {
            Encryption::WPA => "WPA",
            Encryption::WEP => "WEP",
        };
        write!(f, "{}", encryption_str)
    }
}

struct Consts;

impl Consts {
    const FORMAT: &'static str = "svg";
    const BACKGROUND: &'static str = "#000000";
    const FOREGROUND: &'static str = "#ffffff";
    const SIZE: u32 = 512;
}

fn main() -> Result<()> {
    simple_logger::init().unwrap();

    let args = Args::parse();

    let config_str = read_config(&args.config)?;

    let toml_config: Value =
        toml::from_str(&config_str).context("Failed to parse the TOML configuration file")?;

    let export_format = get_config_str(&toml_config, "qrcode", "export", Consts::FORMAT);
    let size = get_config_int(&toml_config, "qrcode", "size", Consts::SIZE as i64) as u32;

    if size < 256 {
        warn!("The image size is lower than 256. The resulting QR code may look cropped.");
    }

    let foreground = get_config_str(&toml_config, "colors", "foreground", Consts::FOREGROUND);
    let background = get_config_str(&toml_config, "colors", "background", Consts::BACKGROUND);

    let password = toml_config["qrcode"]["password"]
        .as_str()
        .context("Failed to get the password from the configuration file")?;

    let contents_to_encode = format!(
        "WIFI:S:{};T:{};P:{};;",
        args.ssid,
        args.encryption.to_string().to_uppercase(),
        password
    );

    let qrcode = QrCode::with_error_correction_level(contents_to_encode.as_bytes(), EcLevel::H)
        .context("Failed to generate the QR code")?;

    let image = qrcode
        .render()
        .min_dimensions(size, size)
        .dark_color(svg::Color(foreground))
        .light_color(svg::Color(background))
        .build();

    save_image(&args.output, &export_format, &image, size)?;

    Ok(())
}

fn read_config(config_path: &PathBuf) -> Result<String> {
    let f = File::open(config_path)
        .with_context(|| format!("Failed to open config file '{:?}'", config_path))?;

    let mut reader = BufReader::new(f);
    let mut config_str = String::new();

    reader
        .read_to_string(&mut config_str)
        .context("Failed to read the config file")?;
    Ok(config_str)
}

fn get_config_str<'a>(config: &'a Value, key: &str, second_key: &str, default: &'a str) -> &'a str {
    config[key][second_key].as_str().unwrap_or_else(|| {
        warn!(
            "Failed to get '{}', falling back to default: {}",
            key, default
        );
        default
    })
}

fn get_config_int(config: &Value, key: &str, second_key: &str, default: i64) -> i64 {
    config[key][second_key].as_integer().unwrap_or_else(|| {
        warn!(
            "Failed to get '{}', falling back to default: {}",
            key, default
        );
        default
    })
}

fn load_svg(contents: &[u8], size: u32) -> Result<Pixmap> {
    let options = Options::default();
    let fontdb = fontdb::Database::new();
    let tree = Tree::from_data(contents, &options, &fontdb)?;

    let mut pixmap = match Pixmap::new(size, size) {
        Some(pxmap) => pxmap,
        None => {
            bail!("Failed to make a new Pixmap");
        }
    };

    render(&tree, Transform::default(), &mut pixmap.as_mut());

    Ok(pixmap)
}

fn save_image(output: &PathBuf, format: &str, image: &str, size: u32) -> Result<()> {
    let file_path = output.with_extension(
        if ["svg", "png"].contains(&format) {
            format
        } else {
            bail!("Invalid image format: '{}'", format);
        }
    );

    match format {
        "svg" => {
            let mut writer = BufWriter::new(
                File::create(output)
                    .with_context(|| format!("Failed to create output file {:?}", output))?,
            );
            writer
                .write_all(image.as_bytes())
                .context("Failed to write SVG image to file")?;
            info!("Saved SVG image to {:?}", file_path);
        }
        "png" => {
            let pixmap = load_svg(image.as_bytes(), size)?;
            pixmap
                .save_png(&file_path)
                .context("Failed to save PNG image to file")?;
            info!("Saved PNG image to {:?}", file_path);
        }
        _ => {
            bail!("Invalid image format: '{}'", format);
        }
    }
    Ok(())
}
