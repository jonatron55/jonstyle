use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, bail, Result as AnyResult};
use clap::{Parser, Subcommand};
use themelib::{
    scripts::{self, pal::make_pal},
    template::Template,
    theme::{Indexer, ThemeBuilder, ThemeVariant},
};

/// Generates six-colour themes from a given configuration file and applies them
/// to a given template.
#[derive(Parser)]
struct Args {
    /// Theme configuration file.
    ///
    /// This should be a TOML file containing a ThemeBuilder struct.
    #[arg()]
    config: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Applies a theme to a given template.
    ///
    /// This command generates theme variants from the given configuration and
    /// applies them to a given template file, writing one file per variant to
    /// an output directory.
    Apply {
        /// Template file to apply the theme to.
        #[arg()]
        template: PathBuf,

        /// Output path to write the generated theme variants to.
        ///
        /// If not provided, the generated files will be written to the current
        /// working directory.
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,

        /// Pattern to use for naming generated files, in the same format as is
        /// used in templates.
        ///
        /// The default will be a filename pattern "{{name:k}}-{{variant:k}}"
        /// with the same extension as the template file.
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Generates a Visual Studio Code theme extension from the given
    /// configuration.
    #[clap(alias = "vscode")]
    VsCode {
        /// Output path to write the generated VS Code theme extension to.
        #[arg(short, long)]
        output: PathBuf,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,

        /// Whether to package the generated theme extension as a .vsix file
        /// after generating it.
        ///
        /// This requires the 'vsce' command-line tool to be installed and
        /// available in the system PATH.
        #[arg(short, long)]
        package: bool,

        /// Whether to install the generated theme extension to the local VS
        /// Code extensions directory.
        #[arg(short, long)]
        install: bool,

        /// License text to include in the generated VS Code theme extension.
        #[arg(short, long)]
        license: Option<PathBuf>,

        /// SPDX license identifier to include in the generated VS Code theme
        /// extension.
        ///
        /// If not provided, the license ID will be automatically detected from
        /// the file provided by '--license' if 'osslili' is available.
        #[arg(short = 'L', long)]
        license_id: Option<String>,

        /// Repository URL to include in the generated VS Code theme extension's
        /// package.json file.
        #[arg(short, long)]
        repository: Option<String>,
    },

    /// Generates a '.pal' (RIFF palette) file containing the theme's colors.
    Pal {
        /// Output path to write the generated .pal file to.
        #[arg()]
        output: Option<PathBuf>,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,
    },

    /// Generates an '.ase' (Adobe Swatch Exchange) file containing the theme's
    /// colors.
    Ase {
        /// Output path to write the generated '.ase' file to.
        #[arg()]
        output: PathBuf,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,
    },

    /// Lists all theme colors for the given variant.
    List {
        #[arg(short, long)]
        variant: ThemeVariant,
    },

    /// Add the given theme to Windows Terminal.
    Wt {
        /// Whether to overwrite existing entries in the Windows Terminal configuration.
        #[arg(short, long)]
        force: bool,
    },
}

pub fn main() -> AnyResult<()> {
    let args = Args::parse();

    let config = fs::read_to_string(&args.config)?;
    let theme = match args.config.extension().and_then(|os_str| os_str.to_str()) {
        Some("toml") | Some("ini") => toml::from_str::<ThemeBuilder>(&config)?.into_theme(),
        Some("json") => serde_json::from_str::<ThemeBuilder>(&config)?.into_theme(),
        Some(other) => {
            return Err(anyhow!("Config file must be a '.toml' or '.json' file ('.{other}' provided)").into());
        }
        None => return Err(anyhow!("Config file must be a '.toml' or '.json' file").into()),
    };

    match args.command {
        Command::Apply {
            template: template_file,
            output,
            force,
            pattern,
        } => {
            let template = Template::new(fs::read_to_string(&template_file)?);
            let pattern = pattern.unwrap_or_else(|| {
                let extension = template_file.extension().and_then(|os_str| os_str.to_str()).unwrap_or("txt");
                format!("{{{{name:k}}}}-{{{{variant:k}}}}.{extension}")
            });

            let output_template = Template::new(pattern.clone());
            let output_path = output.unwrap_or_else(|| env::current_dir().unwrap());

            fs::create_dir_all(&output_path)?;

            for variant in ThemeVariant::iter() {
                let (output, errors) = output_template.render_to_string(&theme, variant);
                if !errors.is_empty() {
                    for error in errors {
                        eprintln!("{}: {error}", pattern);
                    }
                    continue;
                }

                let output = output_path.join(output);
                if output.exists() && !force {
                    eprintln!("{}: Already exists (use --force to overwrite)", output.display());
                    continue;
                }

                let mut file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&output)?;

                let errors = template.render(&theme, variant, &mut file)?;

                for error in errors {
                    eprintln!("{}: {error}", template_file.display());
                }
            }
        }
        Command::VsCode {
            output,
            force,
            package,
            install,
            license,
            license_id,
            repository,
        } => {
            if output.exists() && !force {
                bail!("{}: Already exists (use --force to overwrite)", output.display());
            }

            _ = fs::remove_dir_all(&output);

            scripts::vscode::make_vscode_theme(
                &theme,
                license.as_deref(),
                license_id.as_deref(),
                repository.as_deref(),
                &output,
            )?;

            if package {
                scripts::vscode::package_vscode_theme(&output, license.is_some(), repository.is_some())?;
            }

            if install {
                scripts::vscode::install_vscode_theme(&output)?;
            }
        }
        Command::Pal { output, force } => {
            make_pal(&theme, output.as_deref(), force)?;
        }
        Command::Ase { output: _, force: _ } => {
            todo!()
        }
        Command::List { variant } => {
            for indexer in Indexer::iter(variant) {
                let color = theme.get(&indexer).to_srgba();
                println!("{indexer}: #{color:X}");
            }
        }
        Command::Wt { force } => {
            scripts::wt::make_wt_theme(&theme, force)?;
        }
    }

    Ok(())
}
