use crate::{Error, Result};
use jsonc_parser::ParseOptions;
use lazy_regex::regex;
use serde_json::Value;
use simple_fs::SPath;
use std::borrow::Cow;

// region:    --- JSONC Parser

// Prase a json string content that can have
/// - Comments
/// - Trailing commas
///
/// Note: Property names still need to be quoted.
pub fn parse_jsonc_to_serde_value(content: &str) -> Result<Option<serde_json::Value>> {
	static OPTIONS: ParseOptions = ParseOptions {
		allow_comments: true,
		allow_trailing_commas: true,
		// this one is set to FALSE, for better IDE compatibility
		allow_loose_object_property_names: false,
		allow_single_quoted_strings: false,
		allow_hexadecimal_numbers: false,
		allow_unary_plus_numbers: false,
	};

	let json_value = jsonc_parser::parse_to_serde_value(content, &OPTIONS).map_err(|err| {
		let content = truncate_with_ellipsis(content, 300, "...");
		Error::custom(format!("Fail to parse json.\nCause: {err}\nJson Content:\n{content}"))
	})?;

	Ok(json_value)
}

/// Read & parse a json or jsonc/trailing-commas
pub fn load_jsons_to_serde_value(file: &SPath) -> Result<Option<serde_json::Value>> {
	let content = simple_fs::read_to_string(file)?;

	let value = parse_jsonc_to_serde_value(&content)?;

	Ok(value)
}

// endregion: --- JSONC Parser

pub fn update_json_value_text_mode(content: &str, prop_path: &[&str], value: &Value) -> Result<String> {
	let Some(key) = prop_path.last() else {
		return Err(crate::Error::custom("prop_path cannot be empty"));
	};

	// This regex matches "key": value
	// value can be "string", true, false, or number
	let pattern = format!(r#""{}":\s*("[^"]*"|true|false|[0-9.]+)"#, regex::escape(key));
	let re = regex::Regex::new(&pattern).map_err(|_| crate::Error::custom("Failed to compile regex"))?;

	let matches: Vec<_> = re.find_iter(content).collect();
	if matches.len() > 1 {
		return Err(crate::Error::custom(format!(
			"Ambiguous key '{key}': found {} matches in settings.json",
			matches.len()
		)));
	}
	if matches.is_empty() {
		return Err(crate::Error::custom(format!("Key '{key}' not found in settings.json")));
	}

	let new_value_str = match value {
		Value::String(s) => format!(r#""{s}""#),
		Value::Bool(b) => b.to_string(),
		Value::Number(n) => n.to_string(),
		_ => return Err(crate::Error::custom(format!("Unsupported value type: {value:?}"))),
	};

	let new_content = re.replace(content, format!(r#""{}": {}"#, key, new_value_str));

	Ok(new_content.to_string())
}

pub fn toggle_bool_text_mode(content: &str, prop_path: &[&str]) -> Result<String> {
	// For now, we focus on the last part of the path as the key.
	// In Zed settings, these are usually unique enough or flat keys with dots.
	let Some(key) = prop_path.last() else {
		return Err(crate::Error::custom("prop_path cannot be empty"));
	};

	let re = regex!(r#""([^"]+)"\s*:\s*(true|false)"#);

	let mut found = false;
	let new_content = re.replace_all(content, |caps: &regex::Captures| {
		let k = &caps[1];
		let v = &caps[2];

		if k == *key {
			found = true;
			let new_v = if v == "true" { "false" } else { "true" };
			format!(r#""{}": {}"#, k, new_v)
		} else {
			caps[0].to_string()
		}
	});

	if found {
		Ok(new_content.to_string())
	} else {
		// If not found, add it at the top (after the first '{')
		let mut lines: Vec<String> = content.lines().map(String::from).collect();
		let mut insert_idx = None;

		for (i, line) in lines.iter().enumerate() {
			if line.contains('{') {
				insert_idx = Some(i + 1);
				break;
			}
		}

		if let Some(idx) = insert_idx {
			lines.insert(idx, format!(r#"  "{}": true,"#, key));
			Ok(lines.join("\n"))
		} else {
			// Fallback if no '{' found (rare for JSON)
			Ok(format!("{{\n  \"{}\": true\n}}", key))
		}
	}
}

// region:    --- Support

pub fn truncate_with_ellipsis<'a>(content: &'a str, max_chars: usize, ellipsis: &str) -> Cow<'a, str> {
	let s_len = content.chars().count();
	let ellipsis_len = ellipsis.chars().count();

	if s_len > max_chars {
		if ellipsis_len >= max_chars {
			// Ellipsis itself takes all the space (or more)
			Cow::from(ellipsis.chars().take(max_chars).collect::<String>())
		} else {
			let keep_chars = max_chars - ellipsis_len;
			let truncated: String = content.chars().take(keep_chars).collect();
			Cow::from(format!("{truncated}{ellipsis}"))
		}
	} else {
		Cow::from(content)
	}
}

// endregion: --- Support
