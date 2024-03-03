# ciphercanvas: wifi qr generator

A program written in Rust that generates a QR code from a WiFi network
configuration file. It takes the SSID, encryption type (WPA or WEP), password,
and desired output format as input, and writes the QR code to the specified
output file.

## Table of contents

  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## Usage

For generating a WiFi QR code with the program:

1.  Make a configuration file according to [the
    documentation](docs/configuration.md).

2.  Run `ciphercanvas` to generate the QR code. This will generate a `qrcode.svg`
    image with the QR code for the WiFi network.
    
    ``` console
    $ ./ciphercanvas -s wifi4life -e WPA -c your-config-file.toml
    ```

## Contributing

Contributions are welcome! If you'd like to contribute, please follow this
procedure:

1.  Fork this repository.
2.  Clone the forked repository.
3.  Make the changes and commit them.
4.  Push the branch to your forked repository.

## License

This project is licensed under the [GNU General Public License
version 3.0](LICENSE.md).
