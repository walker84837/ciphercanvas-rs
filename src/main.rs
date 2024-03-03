use anyhow::{anyhow, Context, Result};
use clap::Parser;
use qrcode::{render::svg, EcLevel, QrCode};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use toml::Value;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "the wifi network's ssid (aka its name)")]
    ssid: String,

    #[arg(
        short,
        long,
        default_value = "WPA",
        help = "the encryption of the wifi network. it can be 'WPA' or 'WEP'."
    )]
    encryption: String,

    #[arg(short, long, help = "the toml configuration file.")]
    config: PathBuf,

    #[arg(short, long, default_value = "qrcode.svg")]
    output: PathBuf,
}

struct Consts;

impl Consts {
    const FORMAT: &'static str = "svg";
    const BACKGROUND: &'static str = "#000000";
    const FOREGROUND: &'static str = "#ffffff";
    const SIZE: u32 = 512;
}

fn main() -> Result<()> {
    let arguments = Args::parse();

    let f = File::open(&arguments.config)?;
    let mut reader = BufReader::new(f);
    let mut config_str = String::new();
    reader.read_to_string(&mut config_str)?;

    let toml_config: Value = toml::from_str(&config_str)?;

    let export_format = toml_config["qrcode"]["export"].as_str().unwrap_or_else(|| {
        eprintln!("Found invalid or missing value for output format. Falling back to {:?}...", Consts::FORMAT);
        Consts::FORMAT
    });

    let size = toml_config["qrcode"]["size"]
        .as_str()
        .unwrap_or_else(|| {
            eprintln!("Failed to get size: falling back to {0}x{0}...", Consts::SIZE);
            "512"
        })
        .parse::<u32>()?;

    let foreground = toml_config["colors"]["foreground"]
        .as_str()
        .unwrap_or_else(|| {
            eprintln!(
                "Failed to get foreground color: falling back to {:?}...",
                Consts::FOREGROUND
            );
            Consts::FOREGROUND
        });

    let background = toml_config["colors"]["background"]
        .as_str()
        .unwrap_or_else(|| {
            eprintln!(
                "Failed to get background color: falling back to {:?}...",
                Consts::BACKGROUND
            );
            Consts::BACKGROUND
        });

    let password = toml_config["qrcode"]["password"]
        .as_str()
        .with_context(|| format!("failed to get password from '{:?}'", &arguments.config))?;

    if &arguments.encryption.to_lowercase() != "wpa"
        && &arguments.encryption.to_lowercase() != "wep"
    {
        return Err(anyhow!(
            "Invalid encryption '{}'. Choose WEP or WPA.",
            &arguments.encryption
        ));
    }

    let contents_to_encode = format!(
        "WIFI:S:{};T:{};P:{};;",
        arguments.ssid,
        arguments.encryption.to_uppercase(),
        password
    );

    let qrcode = QrCode::with_error_correction_level(contents_to_encode.as_bytes(), EcLevel::H)?;

    let image = qrcode
        .render()
        .min_dimensions(size, size)
        .dark_color(svg::Color(foreground))
        .light_color(svg::Color(background))
        .build();

    match export_format {
        "svg" => {
            let mut writer = BufWriter::new(File::create(&arguments.output)?);
            writer.write_all(image.as_bytes())?;
        },

        "png" => {
            unimplemented!()
        }

        _ => {
            return Err(anyhow!("Invalid image format: '{}'", export_format));
        }
    }

    Ok(())
}
