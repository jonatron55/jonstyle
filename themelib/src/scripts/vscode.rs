use std::{
    borrow::Cow,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result as AnyResult, bail};
use serde_json::json;

use crate::template::Template;
use crate::theme::{Theme, ThemeMode};

pub fn make_vscode_theme(
    theme: &Theme,
    license: Option<&Path>,
    license_id: Option<&str>,
    repository: Option<&str>,
    output_root: &Path,
) -> AnyResult<()> {
    println!("Generating VS Code theme extension in {}", output_root.display());

    fs::create_dir_all(&output_root)?;
    fs::create_dir_all(output_root.join("themes"))?;
    fs::create_dir_all(output_root.join(".vscode"))?;

    if let Some(license_path) = license {
        let license_text = fs::read_to_string(license_path)?;
        let mut license_file = File::create(output_root.join("LICENSE"))?;
        license_file.write_all(license_text.as_bytes())?;
    }

    let license_id = if let Some(license_id) = license_id {
        Some(license_id.to_string())
    } else if let Some(license_path) = license {
        println!("Detecting license ID from {}...", license_path.display());
        match shell_exec_env(
            format!(
                "osslili --evidence-detail=minimal --output-format=kissbom {}",
                license_path.to_str().unwrap()
            ),
            PathBuf::from("."),
            &[("PYTHONIOENCODING", "UTF-8")],
        ) {
            Ok(output) => {
                let start = output.find('{').unwrap_or(0);
                let output = &output[start..];
                let json = serde_json::from_str::<serde_json::Value>(&output)?;
                let id = json
                    .get("packages")
                    .and_then(|packages| packages.get(0))
                    .and_then(|pkg| pkg.get("license"))
                    .map(|val| val.as_str().map(|s| s.to_owned()))
                    .flatten();
                if let Some(id) = &id {
                    println!("Detected license ID: {id}");
                } else {
                    println!("License not recognized");
                }
                id
            }
            Err(err) => {
                eprintln!("Warning: Failed to detect license ID: {}", err);
                None
            }
        }
    } else {
        None
    };

    let description = theme.meta.description.as_ref().map_or_else(
        || Cow::Owned(format!("{} theme collection for Visual Studio Code.", theme.meta.name)),
        |desc| Cow::Borrowed(desc),
    );

    let package_name = theme
        .meta
        .name
        .chars()
        .filter_map(|ch| {
            if ch.is_whitespace() {
                Some('-')
            } else if ch.is_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<String>();

    let publisher_name = theme
        .meta
        .author
        .as_ref()
        .map(|s| {
            s.chars()
                .filter_map(|ch| {
                    if ch.is_whitespace() {
                        Some('-')
                    } else if ch.is_alphanumeric() {
                        Some(ch.to_ascii_lowercase())
                    } else {
                        None
                    }
                })
                .collect::<String>()
        })
        .unwrap_or_else(|| "unknown".to_string());
    {
        let readme = File::create(output_root.join("README.md"))?;
        let mut readme = BufWriter::new(readme);

        writeln!(readme, "# {} Themes #", theme.meta.name)?;
        writeln!(readme)?;
        writeln!(readme, "{description}")?;
        writeln!(readme)?;
    }

    {
        let changelog = File::create(output_root.join("CHANGELOG.md"))?;
        let mut changelog = BufWriter::new(changelog);

        writeln!(changelog, "# Change Log #")?;
        writeln!(changelog)?;
        writeln!(changelog, "Changes to {}.", theme.meta.name)?;
        writeln!(changelog)?;
        writeln!(changelog, "## [Unreleased] ##")?;
        writeln!(changelog)?;
    }

    {
        let vscodeignore = File::create(output_root.join(".vscodeignore"))?;
        let mut vscodeignore = BufWriter::new(vscodeignore);

        writeln!(vscodeignore, ".vscode")?;
        writeln!(vscodeignore, ".vscode-test")?;
        writeln!(vscodeignore, ".gitignore")?;
    }

    {
        let mut package = File::create(output_root.join("package.json"))?;

        let json = json!({
            "name": package_name,
            "displayName": theme.meta.name,
            "description": description,
            "version": theme.meta.version.to_string(),
            "publisher": publisher_name,
            "engines": {
                "vscode": "^1.88.0"
            },
            "repository": repository,
            "license": license_id,
            "categories": ["Themes"],
            "contributes": {
                "themes": theme.meta.variants.iter().map(|variant| {
                    let variant_name = format!("{variant}").to_lowercase();
                    let theme_name = format!("{} {}", theme.meta.name, variant);
                    let theme_file = format!("themes/{package_name}-{variant_name}-color-theme.json");
                    json!({
                        "label": theme_name,
                        "uiTheme": match variant.mode {
                            ThemeMode::Light => "vs",
                            ThemeMode::Dark => "vs-dark",
                        },
                    "path": theme_file,
                    })
                }).collect::<Vec<_>>(),
            }
        });

        serde_json::to_writer_pretty(&mut package, &json)?;
    }

    let template = Template::new(include_str!("../../../templates/vscode-theme.json"));

    for variant in &theme.meta.variants {
        let variant_name = format!("{variant}").to_lowercase();
        let theme_file = output_root.join(format!("themes/{package_name}-{variant_name}-color-theme.json"));

        let mut file = BufWriter::new(File::create(theme_file)?);
        template.render(theme, *variant, &mut file)?;
    }

    {
        let mut launch = File::create(output_root.join(".vscode/launch.json"))?;
        let json = json!({
            "version": "0.2.0",
            "configurations": [
                {
                    "name": "Run Extension",
                    "type": "extensionHost",
                    "request": "launch",
                    "args": [
                        "--extensionDevelopmentPath=${workspaceFolder}",
                        "--newWindow",
                    ]
                }
            ]
        });

        serde_json::to_writer_pretty(&mut launch, &json)?;
    }

    Ok(())
}

pub fn package_vscode_theme(output_root: &Path, has_license: bool, has_repository: bool) -> AnyResult<()> {
    println!("Packaging VS Code theme extension in {}", output_root.display());

    if !has_license {
        eprintln!("Warning: No license specified. Use --license to specify a license.");
    }

    if !has_repository {
        eprintln!("Warning: No repository specified. Use --repository to specify a repository.");
    }

    let cmd = match (has_license, has_repository) {
        (true, true) => "vsce package",
        (true, false) => "vsce package --allow-missing-repository",
        (false, true) => "vsce package --skip-license",
        (false, false) => "vsce package --skip-license --allow-missing-repository",
    };

    let output = shell_exec(cmd, output_root)?;
    println!("{output}");

    Ok(())
}

pub fn install_vscode_theme(vsix_path: &Path) -> AnyResult<()> {
    let output = Command::new("code").arg("--install-extension").arg(vsix_path).output()?;

    if !output.status.success() {
        eprintln!("Error installing VS Code theme:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("Failed to install VS Code theme");
    }

    Ok(())
}

fn shell_exec(cmd: impl AsRef<str>, cwd: impl AsRef<Path>) -> AnyResult<String> {
    shell_exec_env(cmd, cwd, &[] as &[(&str, &str)])
}

#[cfg(target_os = "windows")]
fn shell_exec_env(
    cmd: impl AsRef<str>,
    cwd: impl AsRef<Path>,
    env: &[(impl AsRef<str>, impl AsRef<str>)],
) -> AnyResult<String> {
    let output = Command::new("pwsh")
        .args(&["-C", cmd.as_ref()])
        .current_dir(cwd.as_ref())
        .envs(env.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
        .output()?;
    if !output.status.success() {
        bail!(
            "Command '{}' failed with exit code {}:\n{}\n{}",
            cmd.as_ref(),
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(not(target_os = "windows"))]
fn shell_exec_env(
    cmd: impl AsRef<str>,
    cwd: impl AsRef<Path>,
    env: &[(impl AsRef<str>, impl AsRef<str>)],
) -> AnyResult<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd.as_ref())
        .current_dir(cwd.as_ref())
        .envs(env.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
        .output()?;
    if !output.status.success() {
        bail!(
            "Command '{}' failed with exit code {}:\n{}\n{}",
            cmd.as_ref(),
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
