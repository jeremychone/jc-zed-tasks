use std::sync::Arc;

// region:    --- AppWindow

impl std::fmt::Debug for AppWindow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AppWindow")
			.field("app", &self.app)
			.field("win_idx", &self.win_idx)
			.field("win_name", &self.win_name)
			.finish()
	}
}

// endregion: --- AppWindow

// region:    --- Window Bounds

#[derive(Debug, Clone, Copy)]
pub struct WindowBounds {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

pub struct AppWindow {
	pub app: Arc<str>,    // the application name
	pub win_idx: i32,     // Index of the apple script tell process
	pub win_name: String, // the windows name
}

// endregion: --- Window Bounds
