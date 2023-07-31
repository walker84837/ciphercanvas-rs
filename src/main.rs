use std::io::Write;
use qrcode::{QrCode, EcLevel, render::svg::Color};
use qrcode::types::QrError;
use std::fs::File;
use std::io;

fn generate_qr_code(wifi_info: &str) -> Result<QrCode, QrError> {
    QrCode::with_error_correction_level(wifi_info.as_bytes(), EcLevel::H)
        .map_err(|e| qrcode::types::QrError::from(e))
}

fn main() {
    let mut ssid = String::new();
    let mut passwd = String::new();
    let mut encrypt = String::new();

    println!("SSID (Wi-Fi network name):");
    io::stdin()
        .read_line(&mut ssid)
        .expect("Failed to read SSID");
    println!("Wi-Fi network password:");
    io::stdin()
        .read_line(&mut passwd)
        .expect("Failed to read password");
    println!("Wi-Fi encryption (WPA|WEP|nopass):");
    io::stdin()
        .read_line(&mut encrypt)
        .expect("Failed to read encryption");

    let wifi_info = format!(
        "WIFI:T:{};S:{};P:{};;",
        encrypt.trim(),
        ssid.trim(),
        passwd.trim()
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
