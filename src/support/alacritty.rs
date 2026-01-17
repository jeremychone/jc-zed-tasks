use crate::Result;
use crate::support::tomls;
use simple_fs::SPath;

pub fn load_settings() -> Result<serde_json::Value> {
	let settings_path = get_config_path()?;

	let value = tomls::load_toml_to_serde_value(&settings_path)?
		.ok_or_else(|| crate::Error::custom(format!("Alacritty settings file is empty: {settings_path}")))?;

	Ok(value)
}

pub fn get_config_path() -> Result<SPath> {
	let home = home::home_dir().ok_or("Could not find home directory")?;
	let config_dir = SPath::from_std_path(&home)?.join(".config");

	let paths = [
		config_dir.join("alacritty/alacritty.toml"),
		config_dir.join("alacritty.toml"),
		SPath::from_std_path(&home)?.join(".alacritty.toml"),
	];

	for path in paths {
		if path.exists() {
			return Ok(path);
		}
	}

	Err(crate::Error::custom(
		"Alacritty config file not found (checked ~/.config/alacritty/alacritty.toml, etc.)",
	))
}
