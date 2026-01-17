use crate::Result;
use lazy_regex::regex;
use serde_json::Value;
use simple_fs::SPath;

pub fn load_toml_to_serde_value(file: &SPath) -> Result<Option<serde_json::Value>> {
	if !file.exists() {
		return Ok(None);
	}
	let content = simple_fs::read_to_string(file)?;
	let value: Value = toml::from_str(&content).map_err(|e| crate::Error::custom(format!("Fail to parse TOML: {e}")))?;
	Ok(Some(value))
}

pub fn update_toml_value_text_mode(content: &str, prop_path: &[&str], value: &Value) -> Result<String> {
	let Some(key) = prop_path.last() else {
		return Err(crate::Error::custom("prop_path cannot be empty"));
	};

	// TOML key = value (at start of line, allowing leading whitespace)
	// Supports simple keys and values (string, bool, number)
	let pattern = format!(r#"(?m)^(\s*{}\s*=\s*)("[^"]*"|true|false|[0-9.]+)"#, regex::escape(key));
	let re = regex::Regex::new(&pattern).map_err(|_| crate::Error::custom("Failed to compile regex"))?;

	let matches: Vec<_> = re.find_iter(content).collect();
	if matches.len() > 1 {
		return Err(crate::Error::custom(format!(
			"Ambiguous key '{key}': found {} matches in TOML content",
			matches.len()
		)));
	}
	if matches.is_empty() {
		return Err(crate::Error::custom(format!("Key '{key}' not found in TOML content")));
	}

	let new_value_str = match value {
		Value::String(s) => format!(r#""{s}""#),
		Value::Bool(b) => b.to_string(),
		Value::Number(n) => n.to_string(),
		_ => return Err(crate::Error::custom(format!("Unsupported value type for surgical update: {value:?}"))),
	};

	let new_content = re.replace(content, |caps: &regex::Captures| {
		format!("{}{}", &caps[1], new_value_str)
	});

	Ok(new_content.to_string())
}
