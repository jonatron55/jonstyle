use std::{env, fs, path::PathBuf};

use anyhow::{Result as AnyResult, bail};
use serde_json::{Map as JsonMap, Value as JsonValue, json};

use crate::theme::{Indexer,  Theme,  ThemeVariant};

pub fn make_wt_theme(theme: &Theme, force: bool) -> AnyResult<()> {
    let local_app_data = PathBuf::from(env::var("LOCALAPPDATA")?);
    let settings_path = local_app_data
        .join("Packages")
        .join("Microsoft.WindowsTerminal_8wekyb3d8bbwe")
        .join("LocalState")
        .join("settings.json");

    if !settings_path.exists() {
        bail!("Windows Terminal settings file not found at {:?}", settings_path);
    }

    let mut json: JsonValue = serde_json::from_str(&fs::read_to_string(&settings_path)?)?;
    let Some(json) = json.as_object_mut() else {
        bail!("Failed to parse Windows Terminal settings as an object");
    };

    let Some(schemes) = json.entry("schemes").or_insert_with(|| JsonValue::Array(vec![])).as_array_mut() else {
        bail!("Failed to parse 'schemes' as an array");
    };

    for variant in &theme.meta.variants {
        let name = format!("{} {}", theme.meta.name, variant.to_string());
        let existing = schemes.iter_mut().find_map(|value| {
            if let Some(object) = value.as_object_mut()
                && object.get("name").and_then(|name| name.as_str()) == Some(&name)
            {
                Some(object)
            } else {
                None
            }
        });

        if existing.is_some() && !force {
            eprintln!("Scheme '{name}' already exists, overwrite with --force");
            continue;
        }

        let mut new = JsonMap::new();
        new.insert("name".to_string(), json!(name));
        new.insert(
            "foreground".to_string(),
            json_color(&theme, *variant, "primary-foreground"),
        );
        new.insert(
            "background".to_string(),
            json_color(&theme, *variant, "page-background"),
        );
        new.insert(
            "selectionBackground".to_string(),
            json_color(&theme, *variant, "selection-background"),
        );
        new.insert("red".to_string(), json_color(&theme, *variant, "dark-red"));
        new.insert("green".to_string(), json_color(&theme, *variant, "dark-green"));
        new.insert("blue".to_string(), json_color(&theme, *variant, "dark-blue"));
        new.insert("cyan".to_string(), json_color(&theme, *variant, "dark-cyan"));
        new.insert("purple".to_string(), json_color(&theme, *variant, "dark-magenta"));
        new.insert("yellow".to_string(), json_color(&theme, *variant, "dark-yellow"));
        new.insert("brightRed".to_string(), json_color(&theme, *variant, "bright-red"));
        new.insert("brightGreen".to_string(), json_color(&theme, *variant, "bright-green"));
        new.insert("brightBlue".to_string(), json_color(&theme, *variant, "bright-blue"));
        new.insert("brightCyan".to_string(), json_color(&theme, *variant, "bright-cyan"));
        new.insert(
            "brightPurple".to_string(),
            json_color(&theme, *variant, "bright-magenta"),
        );
        new.insert(
            "brightYellow".to_string(),
            json_color(&theme, *variant, "bright-yellow"),
        );
        new.insert("black".to_string(), json_color(&theme, *variant, "dark-black"));
        new.insert("brightBlack".to_string(), json_color(&theme, *variant, "bright-black"));
        new.insert("white".to_string(), json_color(&theme, *variant, "dark-white"));
        new.insert("brightWhite".to_string(), json_color(&theme, *variant, "bright-white"));

        if let Some(existing) = existing {
            *existing = new;
        } else {
            schemes.push(new.into());
        }
    }

    let json = serde_json::to_string_pretty(&json)?;

    let backup_path = settings_path.clone().with_file_name("~settings.json");
    _ = fs::remove_file(&backup_path);
    fs::rename(&settings_path, &backup_path)?;

    if let Err(err) = fs::write(&settings_path, json) {
        _ = fs::remove_file(&settings_path);
        fs::rename(&backup_path, &settings_path)?;
        return Err(err.into());
    }

    Ok(())
}

fn json_color(theme: &Theme, variant: ThemeVariant, semantic: impl Into<String>) -> serde_json::Value {
    json!(format!(
        "#{:X}",
        theme.get(&Indexer::Semantic(variant, semantic.into())).without_a().to_srgb()
    ))
}
