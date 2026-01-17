use crate::Result;
use crate::cli::cmd::ToggleProfileArgs;
use crate::support::{alacritty, jsons, tomls, zed};
use serde::{Deserialize, Serialize};
use serde_json::json;
use simple_fs::{SPath, read_to_string};
use std::collections::HashMap;
use std::fs;

// region:    --- Types

#[derive(Deserialize, Serialize)]
struct ProfilesConfig {
	order: Vec<String>,
	#[serde(flatten)]
	profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Serialize)]
struct Profile {
	zed_config: Vec<ConfigEntry>,
	alacritty_config: Vec<ConfigEntry>,
}

#[derive(Deserialize, Serialize)]
struct ConfigEntry {
	config_path: Vec<String>,
	value: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
struct CurrentProfile {
	current_profile: String,
}

// endregion: --- Types

pub fn exec_command(args: ToggleProfileArgs) -> Result<()> {
	toggle_profile(args.profile)
}

// region:    --- Support

fn toggle_profile(target_profile: Option<String>) -> Result<()> {
	let home = home::home_dir().ok_or("Could not find home directory")?;
	let config_dir = SPath::from_std_path(&home)?.join(".config/jc-zed-tasks");
	let profiles_path = config_dir.join("profiles.json");
	let current_path = config_dir.join("profile-current.json");
	let settings_path = SPath::from_std_path(&home)?.join(".config/zed/settings.json");

	init_profiles_if_missing(&config_dir, &profiles_path, &current_path)?;

	if !settings_path.exists() {
		return Err(format!("Zed settings file not found at: {settings_path}").into());
	}

	// -- Load configs
	let profiles_content = read_to_string(&profiles_path)?;
	let profiles_config: ProfilesConfig = serde_json::from_str(&profiles_content)?;

	if profiles_config.order.is_empty() {
		return Err("No profiles defined in 'order' array in profiles.json".into());
	}

	let current_profile_name = if current_path.exists() {
		let current_content = read_to_string(&current_path)?;
		let current_config: CurrentProfile = serde_json::from_str(&current_content)?;
		current_config.current_profile
	} else {
		profiles_config.order[0].clone()
	};

	// -- Determine next profile name
	let next_profile_name = if let Some(target) = target_profile {
		if !profiles_config.profiles.contains_key(&target) {
			return Err(format!("Profile '{target}' not found in profiles.json").into());
		}
		if target == current_profile_name {
			"default".to_string()
		} else {
			target
		}
	} else {
		let current_idx = profiles_config.order.iter().position(|p| p == &current_profile_name);
		let next_idx = match current_idx {
			Some(idx) => (idx + 1) % profiles_config.order.len(),
			None => 0,
		};
		profiles_config.order[next_idx].clone()
	};

	let next_profile = profiles_config
		.profiles
		.get(&next_profile_name)
		.ok_or_else(|| format!("Profile '{next_profile_name}' not found in profiles.json"))?;

	// -- Update settings.json
	let mut settings_content = read_to_string(&settings_path)?;
	for entry in &next_profile.zed_config {
		let path_refs: Vec<&str> = entry.config_path.iter().map(|s| s.as_str()).collect();
		settings_content = jsons::update_json_value_text_mode(&settings_content, &path_refs, &entry.value)?;
	}
	fs::write(settings_path.std_path(), settings_content)?;

	// -- Update alacritty.toml
	if !next_profile.alacritty_config.is_empty() {
		let alacritty_path = alacritty::get_config_path()?;
		let mut alacritty_content = fs::read_to_string(alacritty_path.std_path())?;
		for entry in &next_profile.alacritty_config {
			let path_refs: Vec<&str> = entry.config_path.iter().map(|s| s.as_str()).collect();
			alacritty_content = tomls::update_toml_value_text_mode(&alacritty_content, &path_refs, &entry.value)?;
		}
		fs::write(alacritty_path.std_path(), alacritty_content)?;
	}

	// -- Save changes
	let new_current = CurrentProfile {
		current_profile: next_profile_name.clone(),
	};
	fs::write(current_path.std_path(), serde_json::to_string_pretty(&new_current)?)?;

	println!("Switched to profile: {next_profile_name}");

	zed::touch_tasks_json()?;

	Ok(())
}

fn init_profiles_if_missing(config_dir: &SPath, profiles_path: &SPath, current_path: &SPath) -> Result<()> {
	if profiles_path.exists() {
		return Ok(());
	}

	fs::create_dir_all(config_dir.std_path())?;

	let settings = zed::load_settings()?;
	let ui_font_size = settings
		.get("ui_font_size")
		.ok_or("ui_font_size not found in settings.json")?
		.clone();
	let buffer_font_size = settings
		.get("buffer_font_size")
		.ok_or("buffer_font_size not found in settings.json")?
		.clone();

	// -- Get Alacritty Font Size
	let alacritty_settings = alacritty::load_settings().ok();
	let alacritty_font_size = alacritty_settings
		.as_ref()
		.and_then(|s| s.get("font"))
		.and_then(|f| f.get("size"))
		.cloned()
		.unwrap_or(serde_json::json!(16));

	// -- Build initial profiles.json
	let mut profiles = HashMap::new();

	// Default Profile
	profiles.insert(
		"default".to_string(),
		Profile {
			zed_config: vec![
				ConfigEntry {
					config_path: vec!["ui_font_size".to_string()],
					value: ui_font_size,
				},
				ConfigEntry {
					config_path: vec!["buffer_font_size".to_string()],
					value: buffer_font_size,
				},
			],
			alacritty_config: vec![ConfigEntry {
				config_path: vec!["font".to_string(), "size".to_string()],
				value: alacritty_font_size,
			}],
		},
	);

	// Demo Profile
	profiles.insert(
		"demo".to_string(),
		Profile {
			zed_config: vec![
				ConfigEntry {
					config_path: vec!["ui_font_size".to_string()],
					value: json!(24),
				},
				ConfigEntry {
					config_path: vec!["buffer_font_size".to_string()],
					value: json!(24),
				},
			],
			alacritty_config: vec![ConfigEntry {
				config_path: vec!["font".to_string(), "size".to_string()],
				value: json!(20),
			}],
		},
	);

	let profiles_config = ProfilesConfig {
		order: vec!["default".to_string(), "demo".to_string()],
		profiles,
	};

	fs::write(
		profiles_path.std_path(),
		serde_json::to_string_pretty(&profiles_config)?,
	)?;

	// -- Build initial profile-current.json
	let current_profile = CurrentProfile {
		current_profile: "demo".to_string(),
	};
	fs::write(current_path.std_path(), serde_json::to_string_pretty(&current_profile)?)?;

	Ok(())
}

// endregion: --- Support
