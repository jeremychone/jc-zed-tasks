use crate::Result;
use arboard::Clipboard;
use simple_fs::SPath;

/// Saves the current image from the clipboard to a PNG file at the specified path.
/// Uses the `arboard` crate to retrieve the image and the `image` crate for PNG encoding.
pub fn save_to_png_image(dest_file_path: &SPath) -> Result<()> {
	let mut clipboard = Clipboard::new().map_err(|e| format!("Could not initialize clipboard: {e}"))?;

	let image = clipboard
		.get_image()
		.map_err(|e| format!("Could not get image from clipboard. Cause: {e}"))?;

	let width = image.width as u32;
	let height = image.height as u32;

	// Note: arboard returns images in RGBA8 format.
	image::save_buffer(
		dest_file_path.std_path(),
		&image.bytes,
		width,
		height,
		image::ExtendedColorType::Rgba8,
	)
	.map_err(|e| format!("Failed to save clipboard image to {dest_file_path}. Cause: {e}"))?;

	Ok(())
}

/// Sets the specified text to the clipboard.
pub fn set_text(text: impl Into<String>) -> Result<()> {
	let mut clipboard = Clipboard::new().map_err(|e| format!("Could not initialize clipboard: {e}"))?;

	clipboard
		.set_text(text.into())
		.map_err(|e| format!("Could not set text to clipboard. Cause: {e}"))?;

	Ok(())
}
