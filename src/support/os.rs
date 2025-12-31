#[allow(unused)]
pub fn is_unix() -> bool {
	cfg!(target_os = "macos") || cfg!(target_os = "linux")
}

#[allow(unused)]
pub fn is_mac() -> bool {
	cfg!(target_os = "macos")
}
