# Configuration

The configuration file should include sections for `qrcode` and `colors` with the following variables.

|Variable|Description|
|---|---|
|`export`|The format in which the QR code should be exported (`svg` or `png`. default: `svg`)|
|`size`|Defines the dimensions of the QR code. (default: `512`)|
|`password`|Contains the WiFi password. (mandatory)|
|`foreground`|The foreground color of the QR code. (default: white)|
|`background`|The background color of the QR code. (default: black)|

> [!WARNING]
> Only the SVG format is implemented as of now.

Here is a sample configuration template:

```toml
[qrcode]
password = "your_wifi_password"
export = "svg"
size = 512

[colors]
background = "#111111"
foreground = "#00d0d0"
```
