use crate::Result;
use lazy_regex::regex;
use serde_json::Value;

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
