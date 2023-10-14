# wifi-qrcode-rs

This is a simple WiFi QR code generator written in Rust. It generates the QR code as an SVG image, allowing easy sharing and scanning of WiFi information.

  - [How to Use](#how-to-use)
      - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## How to Use

1.  Open the program.

2.  Input the WiFi name or SSID, the WiFi password, and the WiFi encryption type (WPA, WEP, or nopass for no password).

3.  The program will generate a `qrcode.svg` SVG image with the provided WiFi information.

### Usage

For generating a WiFi QR code with the program, use the following command:

``` $ ./wifi-qrcode-rs-linux-x86_64 --ssid mywifiname --password iusearchbtw123 --encryption WPA
```

This will generate a `qrcode.svg` image with the QR code for the WiFi network.

## Contributing

Contributions are welcome\! If you'd like to contribute, please follow this procedure:

1.  Fork this repository.
2.  Clone the forked repository.
3.  Make the changes and commit them.
4.  Push the branch to your forked repository.

## License

This project is licensed under the GNU General Public License version 3.0 (GPLv3). The full version of the license is [here](LICENSE.md). You can obtain a copy [here](https://www.gnu.org/licenses/gpl-3.0.html).
