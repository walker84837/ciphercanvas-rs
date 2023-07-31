# wifi-qrcode-rs

This is a simple WiFi QR code generator written in Rust. It generates the QR code as an SVG image, allowing easy sharing and scanning of WiFi information.

## How to Use

1. Open the program.

2. Input the WiFi name or SSID, the WiFi password, and the WiFi encryption type (WPA, WEP, or nopass for no password).

3. The program will generate a `qrcode.svg` SVG image with the provided WiFi information.

## Usage example

For generating a WiFi QR code with the program, use the following command:

```$ ./wifi-qrcode-rs-linux-x86_64
$ SSID (Wi-Fi network name):
  example
$ Wi-Fi network password:
  1234
$ Wi-Fi encryption (WPA|WEP|nopass):
  WPA
```

This will generate a `qrcode.svg` image with the QR code for the WiFi.

## License

This project is licensed under the GNU General Public License version 3.0 (GPLv3). The full version of the license is [here](LICENSE). You can obtain a copy [here](https://www.gnu.org/licenses/gpl-3.0.html).
