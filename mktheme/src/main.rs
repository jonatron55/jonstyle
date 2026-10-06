use std::{
    env, fs,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use anyhow::{bail, Result as AnyResult};
use clap::{Parser, Subcommand};
use crossterm::{
    queue,
    style::{Color as TermColor, ContentStyle, Print, PrintStyledContent},
};
use semver::Version;
use themelib::{
    sampler::{self, HueMode},
    scripts::{self, pal::make_pal},
    template::{fmt_string, Template},
    theme::{Indexer, Lum, Metadata, Sat, Temp, Theme, ThemeBuilder, ThemeVariant},
};

/// Generates six-colour themes from a given configuration file and applies them
/// to a given template.
#[derive(Parser)]
struct Args {
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
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        /// Template file to apply the theme to.
        #[arg()]
        template: PathBuf,

        /// Output path to write the generated theme variants to.
        ///
        /// If not provided, the generated files will be written to the current
        /// working directory.
        #[arg(short, long, alias = "out")]
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

    /// Generates a theme from a given image file.
    ///
    /// This command extracts dominant hues from the given image file and
    /// generates a theme configuration based on the mode provided.
    #[clap(alias = "from-img")]
    FromImage {
        /// Image file to extract colors from.
        #[arg()]
        image: PathBuf,

        /// Output path to write the generated theme configuration to.
        #[arg(short, long, alias = "out")]
        output: Option<PathBuf>,

        /// Mode to use for generating the theme from the image.
        #[arg(short, long, value_enum, default_value_t = HueMode::Complementary)]
        mode: HueMode,

        /// Spread argument for the hue mode, in degrees.
        ///
        /// Does not apply to the "custom" mode.
        #[arg(short, long)]
        spread: Option<f64>,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,

        /// The name of the theme.
        #[arg(short, long)]
        name: Option<String>,

        /// The author of the theme.
        #[arg(short, long)]
        author: Option<String>,

        /// An optional description of the theme.
        #[arg(short, long)]
        description: Option<String>,

        /// The version of the theme, following semantic versioning.
        #[arg(short, long)]
        version: Option<Version>,

        /// Output in JSON instead of TOML.
        #[arg(short, long)]
        json: bool,
    },

    /// Generates a Visual Studio Code theme extension from the given
    /// configuration.
    #[clap(alias = "vscode")]
    VsCode {
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        /// Output path to write the generated VS Code theme extension to.
        #[arg(short, long, alias = "out")]
        output: PathBuf,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,

        /// Whether to package the generated theme extension as a .vsix file
        /// after generating it.
        ///
        /// This requires the 'vsce' command-line tool to be installed and
        /// available in the system PATH.
        #[arg(short, long, alias = "pack")]
        package: bool,

        /// Whether to install the generated theme extension to the local VS
        /// Code extensions directory.
        #[arg(short, long)]
        install: bool,

        /// License text to include in the generated VS Code theme extension.
        #[arg(short, long, alias = "lic")]
        license: Option<PathBuf>,

        /// SPDX license identifier to include in the generated VS Code theme
        /// extension.
        ///
        /// If not provided, the license ID will be automatically detected from
        /// the file provided by '--license' if 'osslili' is available.
        #[arg(short = 'L', long, alias = "spdx")]
        license_id: Option<String>,

        /// Repository URL to include in the generated VS Code theme extension's
        /// package.json file.
        #[arg(short, long, alias = "repo")]
        repository: Option<String>,
    },

    /// Generates a '.pal' (RIFF palette) file containing the theme's colors.
    Pal {
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        /// Output path to write the generated .pal file to.
        #[arg(short, long, alias = "out")]
        output: Option<PathBuf>,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,
    },

    /// Generates an '.ase' (Adobe Swatch Exchange) file containing the theme's
    /// colors.
    Ase {
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        /// Output path to write the generated '.ase' file to.
        #[arg(short, long, alias = "out")]
        output: PathBuf,

        /// Whether to overwrite existing files at the output path.
        #[arg(short, long)]
        force: bool,
    },

    /// Lists all theme colors for the given variant.
    List {
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        #[arg(short, long, alias = "var")]
        variant: ThemeVariant,
    },

    /// Add the given theme to Windows Terminal.
    #[cfg(windows)]
    Wt {
        /// Theme configuration file.
        ///
        /// This should be a TOML file containing a ThemeBuilder struct.
        #[arg()]
        config: PathBuf,

        /// Whether to overwrite existing entries in the Windows Terminal configuration.
        #[arg(short, long)]
        force: bool,
    },
}

fn load_config(path: &Path) -> AnyResult<Theme> {
    let config = fs::read_to_string(&path)?;
    match path.extension().and_then(|os_str| os_str.to_str()) {
        Some("toml") | Some("ini") => Ok(toml::from_str::<ThemeBuilder>(&config)?.into_theme()),
        Some("json") => Ok(serde_json::from_str::<ThemeBuilder>(&config)?.into_theme()),
        Some(other) => bail!("Config file must be a '.toml' or '.json' file ('.{other}' provided)"),
        None => bail!("Config file must be a '.toml' or '.json' file"),
    }
}

pub fn main() -> AnyResult<()> {
    let args = Args::parse();

    match args.command {
        Command::Apply {
            config,
            template: template_file,
            output,
            force,
            pattern,
        } => {
            let theme = load_config(&config)?;
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
        Command::FromImage {
            image,
            output,
            mode,
            spread,
            force,
            name,
            author,
            description,
            version,
            json,
        } => {
            let mut meta = Metadata::default();
            if let Some(name) = name {
                meta.name = name;
            } else if let Some(stem) = image.file_stem().and_then(|s| s.to_str()) {
                meta.name = fmt_string(stem, "T");
            }

            meta.author = author;
            meta.description = description;
            if let Some(version) = version {
                meta.version = version;
            }

            let output = if let Some(output) = output {
                if output.is_dir() {
                    output.join(PathBuf::from(fmt_string(&meta.name, "k")).with_added_extension("toml"))
                } else {
                    output
                }
            } else {
                PathBuf::from(fmt_string(&meta.name, "k")).with_added_extension("toml")
            };

            if output.exists() && !force {
                bail!("{}: Already exists (use --force to overwrite)", output.display());
            }

            println!("Analyzing: {}", image.display());
            let histogram = sampler::HueSampler::from_file(&image)?.histogram();
            histogram.print()?;

            let hue_builder = histogram.create_builder(mode, spread);

            let builder = ThemeBuilder {
                hue: hue_builder,
                meta,
                ..Default::default()
            };

            println!("Writing: {}", output.display());

            if json {
                fs::write(output, serde_json::to_string(&builder)?)?;
            } else {
                fs::write(output, toml::to_string(&builder)?)?;
            }

            let theme = builder.into_theme();

            let cold = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Cold, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();
            let cool = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Cool, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();
            let coolish = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Coolish, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();
            let warmish = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Warmish, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();
            let warm = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Warm, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();
            let hot = theme
                .get(&Indexer::Base(Sat::Intense, Temp::Hot, Lum::MediumHigh))
                .to_srgba()
                .to_bytes();

            let cold = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: cold[0],
                    g: cold[1],
                    b: cold[2],
                }),
                ..Default::default()
            };
            let cool = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: cool[0],
                    g: cool[1],
                    b: cool[2],
                }),
                ..Default::default()
            };
            let coolish = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: coolish[0],
                    g: coolish[1],
                    b: coolish[2],
                }),
                ..Default::default()
            };
            let warmish = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: warmish[0],
                    g: warmish[1],
                    b: warmish[2],
                }),
                ..Default::default()
            };
            let warm = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: warm[0],
                    g: warm[1],
                    b: warm[2],
                }),
                ..Default::default()
            };
            let hot = ContentStyle {
                foreground_color: Some(TermColor::Rgb {
                    r: hot[0],
                    g: hot[1],
                    b: hot[2],
                }),
                ..Default::default()
            };

            queue!(
                stdout(),
                Print("Theme created with hues: "),
                PrintStyledContent(cold.apply("███ ")),
                PrintStyledContent(cool.apply("███ ")),
                PrintStyledContent(coolish.apply("███ ")),
                PrintStyledContent(warmish.apply("███ ")),
                PrintStyledContent(warm.apply("███ ")),
                PrintStyledContent(hot.apply("███ ")),
                Print("\n"),
            )?;

            stdout().flush()?;
        }

        Command::VsCode {
            config,
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
                &load_config(&config)?,
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
        Command::Pal { config, output, force } => {
            make_pal(&load_config(&config)?, output.as_deref(), force)?;
        }
        Command::Ase {
            config,
            output: _,
            force: _,
        } => {
            let _ = load_config(&config)?;
            todo!()
        }
        Command::List { config, variant } => {
            let theme = load_config(&config)?;
            for indexer in Indexer::iter(variant) {
                let color = theme.get(&indexer).to_srgba();
                println!("{indexer}: #{color:X}");
            }
        }
        #[cfg(windows)]
        Command::Wt { config, force } => {
            let theme = load_config(&config)?;
            scripts::wt::make_wt_theme(&theme, force)?;
        }
    }

    Ok(())
}
