use qrcode::{render::svg::Color, types::QrError, EcLevel, QrCode};
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long)]
    ssid: String,

    #[structopt(short, long)]
    password: String,

    #[structopt(short, long)]
    encryption: String,
}

fn generate_qr_code(wifi_info: &str) -> Result<QrCode, QrError> {
    QrCode::with_error_correction_level(wifi_info.as_bytes(), EcLevel::H).map_err(|e| QrError::from(e))
}

fn main() {
    let options = Options::from_args();

    let wifi_info = format!(
        "WIFI:T:{};S:{};P:{};;",
        options.encryption,
        options.ssid,
        options.password
    );

    match generate_qr_code(&wifi_info) {
        Ok(qr_code) => {
            let renderer: String = qr_code.render::<Color>().build();
            let svg_data: String = renderer.to_string();

            match File::create("qrcode.svg") {
                Ok(mut f) => {
                    f.write_all(svg_data.as_bytes())
                        .expect("Error exporting QR code to SVG");
                }
                Err(e) => {
                    eprintln!("Error exporting QR code to SVG: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        }
    }
}

