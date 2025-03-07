# ciphercanvas: Wi-Fi QR Code Generator

A robust and efficient program written in Rust that generates QR codes for Wi-Fi
networks. It takes inputs such as SSID, encryption type (WPA or WEP), password,
and desired output format, producing a QR code that simplifies Wi-Fi access
sharing.

## Table of Contents

  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## Usage

To generate a Wi-Fi QR code using CipherCanvas:

1.  **Create a configuration file**: Follow the guidelines in [the
    documentation](docs/configuration.md) to create a configuration file that
    includes your Wi-Fi network details.

2.  **Generate the QR code**: Run the `ccanvas` command with the
    appropriate options to generate your QR code. This example command creates a
    `qrcode.svg` file based on your configuration:
    
    ``` console
    $ ccanvas -s wifi4life -e wpa -c your-config-file.toml -o qrcode.svg
    ```

## Contributing

We welcome contributions from the community! If you would like to contribute to
CipherCanvas, here is a few things you can do:

### Roadmap

- [ ] Support exporting images in other formats
- [ ] Switch to a crate which provides fancier logging (like `env_logger`)
- [ ] Create config file if not found ([directories](https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html))
- [ ] Resolve issue where an image that is smaller than 256x256 is smaller
  - Consider using a bigger image and [rescaling](https://docs.rs/image/latest/image/imageops/fn.resize.html) it.

## License

This project is licensed under the [GNU General Public License
version 3.0](LICENSE.md). By contributing to this project, you agree to license
your contributions under the same license.
