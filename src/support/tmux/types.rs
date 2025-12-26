use macro_rules_attribute as mra;
use crate::Id;
use std::sync::Arc;

// region:    --- Data Structures

#[mra::derive(Id!)]
pub struct SessionId(Arc<str>);

#[mra::derive(Id!)]
pub struct WindowId(Arc<str>);

#[mra::derive(Id!)]
pub struct PaneId(Arc<str>);

#[derive(Debug, Clone)]
pub struct TmuxPane {
	pub id: PaneId,
	pub session_id: SessionId,
	pub window_id: WindowId,
	pub index: usize,
	pub title: String,
	pub path: String,
	pub command: String,
}

#[derive(Debug, Clone)]
pub struct TmuxWindow {
	pub id: WindowId,
	pub index: usize,
	pub name: String,
	pub panes: Vec<TmuxPane>,
}

#[derive(Debug, Clone)]
pub struct TmuxSession {
	pub id: SessionId,
	pub name: String,
	pub attached: bool,
	pub windows: Vec<TmuxWindow>,
}

#[derive(Debug, Clone)]
pub struct TmuxSessions(pub Vec<TmuxSession>);

impl TmuxSessions {
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for TmuxSessions {
	type Item = TmuxSession;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// endregion: --- Data Structures
