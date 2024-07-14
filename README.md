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
CipherCanvas, please follow these steps:

1.  Fork the repository by clicking [this
    link](https://github.com/walker84837/ciphercanvas-rs/fork).

2.  Clone the forked repository to your local machine.
    
    ``` console
    $ git clone https://github.com/walker84837/ciphercanvas-rs.git
    ```

3.  **Make your changes**: Create a new branch, make your changes, and commit
    them. I recommend following [Conventional
    Commits](https://www.conventionalcommits.org/) and <https://commit.style/>.
    
    ``` console
    $ git checkout -b your-feature-branch
    $ git commit -m "Description of your changes"
    ```

4.  **Push your branch**: Push your changes to your forked repository.
    
    ``` console
    $ git push origin your-feature-branch
    ```

5.  **Create a PR**: Open a pull request from your forked repository
    to the main repository. Provide any relevant information for reviewers.

## License

This project is licensed under the [GNU General Public License
version 3.0](LICENSE.md). By contributing to this project, you agree to license
your contributions under the same license.
