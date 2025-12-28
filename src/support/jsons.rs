use crate::Result;
use lazy_regex::regex;

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
