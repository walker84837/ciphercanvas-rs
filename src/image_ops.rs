use anyhow::{bail, Context, Result};
use log::{error, info};
use resvg::render;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};
use tiny_skia::{Pixmap, Transform};
use usvg::{fontdb, Options, Tree};

fn load_svg(contents: &[u8], size: u32) -> Result<Pixmap> {
    info!("Loading SVG content with size {}x{}", size, size);

    let options = Options::default();
    let fontdb = fontdb::Database::new();

    let tree: Tree = Tree::from_data(contents, &options, &fontdb).with_context(|| {
        format!(
            "Failed to create SVG tree from data of size {}x{}",
            size, size
        )
    })?;
    info!("Successfully created SVG tree");

    let mut pixmap: Pixmap = Pixmap::new(size, size).context("Failed to create a new Pixmap")?;
    info!("Created Pixmap of size {}x{}", size, size);

    render(&tree, Transform::default(), &mut pixmap.as_mut());
    info!("Rendered SVG to Pixmap");

    Ok(pixmap)
}

/// Save an image to a file
/// Fails when the format is not supported, or when saving fails
/// # Examples:
/// Save an SVG image
/// ```rust
/// use ciphercanvas::save_image;
/// let image = "<svg>...</svg>";
/// let format = "svg";
/// let size = 128;
/// let output = PathBuf::from("output.svg");
/// save_image(&output, &format, &image, size).unwrap();
/// ```
///
/// Save a PNG image
/// ```rust
/// use ciphercanvas::save_image;
/// let image = "<svg>...</svg>";
/// let format = "png";
/// let size = 128;
/// let output = PathBuf::from("output.png");
/// save_image(&output, &format, &image, size).unwrap();
/// ```
pub fn save_image(output: &PathBuf, format: &str, image: &str, size: u32) -> Result<()> {
    const SUPPORTED_FORMATS: &[&str] = &["svg", "png"];
    info!(
        "Starting to save image with format '{}' to {:?}",
        format, output
    );

    let file_path = output.with_extension(if SUPPORTED_FORMATS.contains(&format) {
        format
    } else {
        bail!("Unsupported image format: '{}'", format);
    });

    match format {
        "svg" => {
            let mut writer = BufWriter::new(
                File::create(&file_path)
                    .with_context(|| format!("Failed to create output file {:?}", file_path))?,
            );
            writer
                .write_all(image.as_bytes())
                .with_context(|| format!("Failed to write SVG image to file {:?}", file_path))?;
            info!("Saved SVG image to {:?}", file_path);
        }
        "png" => {
            if size <= 256 {
                error!(
                    "Warning: Image size is {}x{}, which may result in lower quality.",
                    size, size
                );
            }
            let pixmap = load_svg(image.as_bytes(), size)?;
            pixmap
                .save_png(&file_path)
                .with_context(|| format!("Failed to save PNG image to file {:?}", file_path))?;
            info!("Saved PNG image to {:?}", file_path);
        }
        _ => {
            bail!("Unsupported image format: '{}'", format);
        }
    }

    info!("Image saved successfully to {:?}", file_path);
    Ok(())
}
