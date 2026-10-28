mod builder;
mod colorref;
mod primary;
mod variant;

use std::{collections::HashMap, fs::File, io, path::PathBuf};

use anyhow::Result as AnyResult;
use semver::Version;

pub use builder::*;
pub use colorref::*;
pub use primary::*;
use serde::{Deserialize, Serialize};
pub use variant::*;

use crate::{
    color::{OkHsl, OkHsla},
    template::Template,
};

pub type BasePalette = [[[OkHsl; LUM_COUNT]; TEMP_COUNT]; SAT_COUNT];
pub type PrimaryMap = HashMap<Primary, Temp>;

const BACKGROUND_ALPHA: f64 = 2.0 / 3.0;
const SHADOW_ALPHA: f64 = 2.0 / 3.0;

pub struct Theme {
    /// Theme metadata.
    pub meta: Metadata,

    /// The code style configuration for the theme.
    pub code: CodeStyle,

    /// The hue range in degrees for the theme's cool colors.
    base_palette: BasePalette,

    /// Mapping of primary colors to their associated temperatures.
    primaries: PrimaryMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    /// The name of the theme.
    pub name: String,

    /// The variants to include in the theme.
    pub variants: Vec<ThemeVariant>,

    /// The variant to use if none is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,

    /// Path to theme icon.
    ///
    /// This may be a standard image format or an SVG file. If an SVG file is
    /// used, it will be treated as a [`Template`] and may include substitutions
    /// that will be replaced by the theme colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The author of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// An optional description of the theme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The version of the theme, following semantic versioning.
    pub version: Version,
}

impl Theme {
    /// Returns the default theme variant.
    pub fn default_variant(&self) -> &'_ ThemeVariant {
        if let Some(default) = &self
            .meta
            .default_variant
            .as_ref()
            .and_then(|d| self.meta.variants.iter().find(|v| &v.name == d))
        {
            return default;
        }

        &self.meta.variants[0]
    }

    /// Returns the icon for the given theme variant.
    ///
    /// The result may be either a path to an image file or an SVG string. A
    /// default icon will be used if the metadata does not specify one.
    pub fn icon(&self, root: impl Into<PathBuf>, variant: &ThemeVariant) -> AnyResult<Icon> {
        if let Some(icon) = &self.meta.icon {
            let path = root.into().join(icon);

            if path.extension().map_or(false, |ext| ext == "svg") {
                let svg = File::open(&path)?;
                let svg = Template::new(io::read_to_string(svg)?);
                Ok(Icon::Svg(svg.render_to_string(&self, variant).0))
            } else {
                Ok(Icon::Path(path))
            }
        } else {
            let svg = Template::new(include_str!("../../templates/icon.svg"));
            Ok(Icon::Svg(svg.render_to_string(&self, variant).0))
        }
    }

    #[cfg(feature = "image")]
    pub fn icon_image(&self, root: impl Into<PathBuf>, variant: &ThemeVariant) -> AnyResult<image::DynamicImage> {
        match self.icon(root, variant)? {
            Icon::Path(path) => Ok(image::open(path)?),
            Icon::Svg(svg) => {
                let mut data = vec![0u8; 256 * 256 * 4];
                let mut pixmap = tiny_skia::PixmapMut::from_bytes(&mut data, 256, 256)
                    .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

                resvg::render(
                    &usvg::Tree::from_str(&svg, &usvg::Options::default())?,
                    tiny_skia::Transform::identity(),
                    &mut pixmap,
                );
                Ok(image::DynamicImage::ImageRgba8(
                    image::RgbaImage::from_raw(256, 256, data)
                        .ok_or_else(|| anyhow::anyhow!("Failed to create image from pixmap"))?,
                ))
            }
        }
    }

    /// Gets the color corresponding to the given indexer.
    pub fn get<'v, 's>(&self, index: impl Into<ColorRef<'v, 's>>) -> OkHsla {
        match index.into() {
            ColorRef::Base(sat, temp, lum) => self.base_palette[sat as usize][temp as usize][lum as usize].with_a(1.0),
            ColorRef::Themed(variant, sat, hue, level) => {
                let temp = hue.to_temp(variant);
                let lum = level.to_lum(variant);
                self.base_palette[sat as usize][temp as usize][lum as usize].with_a(1.0)
            }
            ColorRef::Primary(sat, primary, lum) => {
                let temp = self.primaries[&primary];
                self.base_palette[sat as usize][temp as usize][lum as usize].with_a(1.0)
            }
            ColorRef::ThemedPrimary(variant, sat, primary, level) => {
                let temp = self.primaries[&primary];
                let lum = level.to_lum(variant);
                self.base_palette[sat as usize][temp as usize][lum as usize].with_a(1.0)
            }
            ColorRef::Semantic(variant, name) => {
                let red = self.primaries[&Primary::Red].to_theme_hue(variant);
                let green = self.primaries[&Primary::Green].to_theme_hue(variant);
                let blue = self.primaries[&Primary::Blue].to_theme_hue(variant);
                let cyan = self.primaries[&Primary::Cyan].to_theme_hue(variant);
                let magenta = self.primaries[&Primary::Magenta].to_theme_hue(variant);
                let yellow = self.primaries[&Primary::Yellow].to_theme_hue(variant);

                let ctrl_hue = ThemeHue::Color1;

                let danger_hue = if ctrl_hue != red { red } else { magenta };

                let caution_hue = if ctrl_hue != yellow {
                    yellow
                } else if danger_hue != magenta {
                    magenta
                } else {
                    cyan
                };

                let ok_hue = if ctrl_hue != green { green } else { blue };

                let mut accent_hue = ThemeHue::Color6;
                while accent_hue == danger_hue
                    || accent_hue == caution_hue
                    || accent_hue == ok_hue
                    || accent_hue == ctrl_hue
                {
                    accent_hue = accent_hue.next();
                }

                let accent_level = Level::HigherMidground;
                let ctrl_fg_level = Level::Foreground;
                let ctrl_bg_level = Level::LowMidground;
                let link_level = Level::HighMidground;
                let disabled_fg_level = Level::HighMidground;
                let disabled_bg_level = Level::HighBackground;
                let disabled_link_level = Level::LowMidground;

                let (index, alpha) = match name {
                    /*
                        General UI semantics
                    */
                    "primary-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                        1.0,
                    ),
                    "intense-foreground" => (
                        ColorRef::Themed(variant, Sat::Intense, ThemeHue::Color6, Level::HighForeground),
                        1.0,
                    ),
                    "dim-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::LowMidground),
                        1.0,
                    ),
                    "content-background" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::Background),
                        1.0,
                    ),
                    "card-background" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground),
                        1.0,
                    ),
                    "page-background" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "selection-background" => (
                        ColorRef::Themed(variant, Sat::Base, accent_hue, Level::LowMidground),
                        1.0,
                    ),
                    "dim-selection-background" => (
                        ColorRef::Themed(variant, Sat::Base, accent_hue, Level::HighBackground),
                        1.0,
                    ),
                    "content-background-transparent" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::Background),
                        BACKGROUND_ALPHA,
                    ),
                    "card-background-transparent" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "chrome-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                        1.0,
                    ),
                    "chrome-high" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                        1.0,
                    ),
                    "chrome" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Level::HighMidground),
                        1.0,
                    ),
                    "chrome-low" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowMidground),
                        1.0,
                    ),
                    "chrome-background" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Level::HighBackground),
                        1.0,
                    ),
                    "caption-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "chrome-shadow" => (
                        ColorRef::Themed(variant, Sat::Base, ThemeHue::Color1, Lum::DimHigh.to_level(variant)),
                        SHADOW_ALPHA,
                    ),
                    "dim-shadow" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Lum::DimHigh.to_level(variant)),
                        SHADOW_ALPHA,
                    ),
                    "intense-shadow" => (
                        ColorRef::Themed(variant, Sat::Intense, ThemeHue::Color6, Lum::DimLow.to_level(variant)),
                        SHADOW_ALPHA,
                    ),

                    /*
                        Hyperlinks
                    */
                    "link" => (ColorRef::Themed(variant, Sat::Intense, ctrl_hue, link_level), 1.0),
                    "link-hover" => (
                        ColorRef::Themed(variant, Sat::Intense, ctrl_hue, link_level.brighter(variant)),
                        1.0,
                    ),
                    "link-active" => (
                        ColorRef::Themed(variant, Sat::Intense, ctrl_hue, link_level.dimmer(variant)),
                        1.0,
                    ),
                    "link-disabled" => (ColorRef::Themed(variant, Sat::Base, ctrl_hue, disabled_link_level), 1.0),

                    /*
                        Labels
                    */
                    "label" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                        1.0,
                    ),
                    "label-hover" => (
                        ColorRef::Themed(
                            variant,
                            Sat::Muted,
                            ThemeHue::Color6,
                            Level::Foreground.brighter(variant),
                        ),
                        1.0,
                    ),
                    "label-active" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground.dimmer(variant)),
                        1.0,
                    ),
                    "label-disabled" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, disabled_fg_level),
                        1.0,
                    ),

                    /*
                        Primary controls
                    */
                    "control-foreground" => (ColorRef::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level), 1.0),
                    "control-background" => (ColorRef::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level), 1.0),
                    "control-hover-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level.brighter(variant)),
                        1.0,
                    ),
                    "control-hover-background" => (
                        ColorRef::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level.brighter(variant)),
                        1.0,
                    ),
                    "control-active-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level.dimmer(variant)),
                        1.0,
                    ),
                    "control-active-background" => (
                        ColorRef::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level.dimmer(variant)),
                        1.0,
                    ),
                    "control-disabled-foreground" => {
                        (ColorRef::Themed(variant, Sat::Muted, ctrl_hue, disabled_fg_level), 1.0)
                    }
                    "control-disabled-background" => {
                        (ColorRef::Themed(variant, Sat::Muted, ctrl_hue, disabled_bg_level), 1.0)
                    }

                    /*
                        Accent controls
                    */
                    "accent-control-foreground" => {
                        (ColorRef::Themed(variant, Sat::Base, accent_hue, ctrl_fg_level), 1.0)
                    }
                    "accent-control-background" => {
                        (ColorRef::Themed(variant, Sat::Intense, accent_hue, ctrl_bg_level), 1.0)
                    }
                    "accent-control-hover-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, accent_hue, ctrl_fg_level.brighter(variant)),
                        1.0,
                    ),
                    "accent-control-hover-background" => (
                        ColorRef::Themed(variant, Sat::Intense, accent_hue, ctrl_bg_level.brighter(variant)),
                        1.0,
                    ),
                    "accent-control-active-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, accent_hue, ctrl_fg_level.dimmer(variant)),
                        1.0,
                    ),
                    "accent-control-active-background" => (
                        ColorRef::Themed(variant, Sat::Intense, accent_hue, ctrl_bg_level.dimmer(variant)),
                        1.0,
                    ),
                    "accent-control-disabled-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, accent_hue, disabled_fg_level),
                        1.0,
                    ),
                    "accent-control-disabled-background" => (
                        ColorRef::Themed(variant, Sat::Muted, accent_hue, disabled_bg_level),
                        1.0,
                    ),

                    /*
                        Affirmative/success colors
                    */
                    "ok-control-foreground" => (ColorRef::Themed(variant, Sat::Base, ok_hue, ctrl_fg_level), 1.0),
                    "ok-control-background" => (ColorRef::Themed(variant, Sat::Intense, ok_hue, ctrl_bg_level), 1.0),
                    "ok-control-hover-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, ok_hue, ctrl_fg_level.brighter(variant)),
                        1.0,
                    ),
                    "ok-control-hover-background" => (
                        ColorRef::Themed(variant, Sat::Intense, ok_hue, ctrl_bg_level.brighter(variant)),
                        1.0,
                    ),
                    "ok-control-active-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, ok_hue, ctrl_fg_level.dimmer(variant)),
                        1.0,
                    ),
                    "ok-control-active-background" => (
                        ColorRef::Themed(variant, Sat::Base, ok_hue, ctrl_bg_level.dimmer(variant)),
                        1.0,
                    ),
                    "ok-control-disabled-foreground" => {
                        (ColorRef::Themed(variant, Sat::Muted, ok_hue, disabled_fg_level), 1.0)
                    }
                    "ok-control-disabled-background" => {
                        (ColorRef::Themed(variant, Sat::Muted, ok_hue, disabled_bg_level), 1.0)
                    }

                    "ok-foreground" => (ColorRef::Themed(variant, Sat::Intense, ok_hue, accent_level), 1.0),
                    "ok-background" => (ColorRef::Themed(variant, Sat::Base, ok_hue, Level::HighBackground), 1.0),
                    "ok-background-transparent" => (
                        ColorRef::Themed(variant, Sat::Base, ok_hue, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "ok-shadow" => (
                        ColorRef::Themed(variant, Sat::Base, ok_hue, Lum::DimHigh.to_level(variant)),
                        SHADOW_ALPHA,
                    ),

                    /*
                        Cautionary/warning colors
                    */
                    "caution-control-foreground" => {
                        (ColorRef::Themed(variant, Sat::Base, caution_hue, ctrl_fg_level), 1.0)
                    }
                    "caution-control-background" => {
                        (ColorRef::Themed(variant, Sat::Intense, caution_hue, ctrl_bg_level), 1.0)
                    }
                    "caution-control-hover-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, ctrl_fg_level.brighter(variant)),
                        1.0,
                    ),
                    "caution-control-hover-background" => (
                        ColorRef::Themed(variant, Sat::Intense, caution_hue, ctrl_bg_level.brighter(variant)),
                        1.0,
                    ),
                    "caution-control-active-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, ctrl_fg_level.dimmer(variant)),
                        1.0,
                    ),
                    "caution-control-active-background" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, ctrl_bg_level.dimmer(variant)),
                        1.0,
                    ),
                    "caution-control-disabled-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, caution_hue, disabled_fg_level),
                        1.0,
                    ),
                    "caution-control-disabled-background" => (
                        ColorRef::Themed(variant, Sat::Muted, caution_hue, disabled_bg_level),
                        1.0,
                    ),
                    "caution-foreground" => (ColorRef::Themed(variant, Sat::Intense, caution_hue, accent_level), 1.0),
                    "caution-background" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, Level::HighBackground),
                        1.0,
                    ),
                    "caution-background-transparent" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "caution-shadow" => (
                        ColorRef::Themed(variant, Sat::Base, caution_hue, Lum::DimHigh.to_level(variant)),
                        SHADOW_ALPHA,
                    ),

                    /*
                        Danger/error colors
                    */
                    "danger-control-foreground" => {
                        (ColorRef::Themed(variant, Sat::Base, danger_hue, ctrl_fg_level), 1.0)
                    }
                    "danger-control-background" => {
                        (ColorRef::Themed(variant, Sat::Intense, danger_hue, ctrl_bg_level), 1.0)
                    }
                    "danger-control-hover-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, ctrl_fg_level.brighter(variant)),
                        1.0,
                    ),
                    "danger-control-hover-background" => (
                        ColorRef::Themed(variant, Sat::Intense, danger_hue, ctrl_bg_level.brighter(variant)),
                        1.0,
                    ),
                    "danger-control-active-foreground" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, ctrl_fg_level.dimmer(variant)),
                        1.0,
                    ),
                    "danger-control-active-background" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, ctrl_bg_level.dimmer(variant)),
                        1.0,
                    ),
                    "danger-control-disabled-foreground" => (
                        ColorRef::Themed(variant, Sat::Muted, danger_hue, disabled_fg_level),
                        1.0,
                    ),
                    "danger-control-disabled-background" => (
                        ColorRef::Themed(variant, Sat::Muted, danger_hue, disabled_bg_level),
                        1.0,
                    ),
                    "danger-foreground" => (ColorRef::Themed(variant, Sat::Intense, danger_hue, accent_level), 1.0),
                    "danger-background" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, Level::HighBackground),
                        1.0,
                    ),
                    "danger-background-transparent" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "danger-shadow" => (
                        ColorRef::Themed(variant, Sat::Base, danger_hue, Lum::DimHigh.to_level(variant)),
                        SHADOW_ALPHA,
                    ),

                    /*
                        Input and text fields
                    */
                    "input-background" => (ColorRef::Themed(variant, Sat::Base, ctrl_hue, Level::Background), 1.0),
                    "input-border" => (ColorRef::Themed(variant, Sat::Base, ctrl_hue, Level::LowMidground), 1.0),
                    "input-hover-background" => (
                        ColorRef::Themed(variant, Sat::Base, ctrl_hue, Level::Background.brighter(variant)),
                        1.0,
                    ),
                    "input-hover-border" => (
                        ColorRef::Themed(variant, Sat::Base, ctrl_hue, Level::LowMidground.brighter(variant)),
                        1.0,
                    ),
                    "input-active-background" => (
                        ColorRef::Themed(variant, Sat::Base, ctrl_hue, Level::Background.dimmer(variant)),
                        1.0,
                    ),
                    "input-active-border" => (
                        ColorRef::Themed(variant, Sat::Intense, ctrl_hue, Level::LowMidground.dimmer(variant)),
                        1.0,
                    ),

                    /*
                        Terminal colors
                    */
                    "dark-black" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "bright-black" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowerMidground),
                        1.0,
                    ),
                    "dark-white" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HigherMidground),
                        1.0,
                    ),
                    "bright-white" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HighForeground),
                        1.0,
                    ),
                    "dark-red" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HighMidground),
                        1.0,
                    ),
                    "bright-red" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::LowForeground),
                        1.0,
                    ),
                    "dark-green" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::HighMidground),
                        1.0,
                    ),
                    "bright-green" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::LowForeground),
                        1.0,
                    ),
                    "dark-yellow" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, Level::HighMidground),
                        1.0,
                    ),
                    "bright-yellow" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, Level::LowForeground),
                        1.0,
                    ),
                    "dark-blue" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                        1.0,
                    ),
                    "bright-blue" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::LowForeground),
                        1.0,
                    ),
                    "dark-magenta" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::HighMidground),
                        1.0,
                    ),
                    "bright-magenta" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::LowForeground),
                        1.0,
                    ),
                    "dark-cyan" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::HighMidground),
                        1.0,
                    ),
                    "bright-cyan" => (
                        ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::LowForeground),
                        1.0,
                    ),

                    /*
                        Code semantics
                    */
                    "comment" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai | ColorVocabulary::Primer => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HigherMidground),
                            1.0,
                        ),
                    },
                    "comment-symbol" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai | ColorVocabulary::Primer => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "keyword" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "control-keyword" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "directive" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Muted, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "operator" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "punctuation" => (
                        ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::LowForeground),
                        1.0,
                    ),
                    "variable" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai | ColorVocabulary::Primer => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::LowForeground),
                            1.0,
                        ),
                    },
                    "argument" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Muted, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai | ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                    },
                    "literal" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                    },
                    "unit" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HigherMidground),
                            1.0,
                        ),
                    },
                    "string" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::Foreground),
                            1.0,
                        ),
                    },
                    "string-delimiter" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "escape" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::LowerMidground),
                            1.0,
                        ),
                    },
                    "function" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::Foreground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Cyan, Level::Foreground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Magenta, Level::HigherMidground),
                            1.0,
                        ),
                    },
                    "type" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HigherMidground),
                            1.0,
                        ),
                    },
                    "namespace" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio | ColorVocabulary::Monokai => (
                            ColorRef::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::Foreground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "macro" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                    },
                    "markup" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Vim => (
                            ColorRef::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::Monokai => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Red, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::Primer => (
                            ColorRef::ThemedPrimary(variant, Sat::Intense, Primary::Green, Level::HigherMidground),
                            1.0,
                        ),
                    },
                    _ => panic!("Unknown semantic color name: {}", name),
                };

                let mut color = self.get(&index);
                color.a = alpha;
                color
            }
        }
    }
}

pub enum Icon {
    Path(PathBuf),
    Svg(String),
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            author: None,
            version: Version::new(0, 1, 0),
            description: None,
            icon: None,
            variants: vec![
                ThemeVariant::dawn(),
                ThemeVariant::dusk(),
                ThemeVariant::noon(),
                ThemeVariant::night(),
            ],
            default_variant: Some("Night".to_string()),
        }
    }
}
