mod builder;
mod index;
mod primary;
mod variant;

use std::collections::HashMap;

use semver::Version;

pub use builder::*;
pub use index::*;
pub use primary::*;
use serde::{Deserialize, Serialize};
pub use variant::*;

use crate::color::{OkHsl, OkHsla};

pub type BasePalette = [[[OkHsl; LUM_COUNT]; TEMP_COUNT]; SAT_COUNT];
pub type PrimaryMap = HashMap<Primary, Temp>;

const BACKGROUND_ALPHA: f64 = 2.0 / 3.0;
const SHADOW_ALPHA: f64 = 2.0 / 3.0;

pub struct Theme {
    /// Theme metadata.
    pub meta: Metadata,

    /// The hue range in degrees for the theme's cool colors.
    base_palette: BasePalette,

    /// Mapping of primary colors to their associated temperatures.
    primaries: PrimaryMap,

    /// The code style configuration for the theme.
    pub code: CodeStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    /// The name of the theme.
    pub name: String,

    /// The variants to include in the theme.
    pub variants: Vec<ThemeVariant>,

    /// The author of the theme.
    pub author: Option<String>,

    /// An optional description of the theme.
    pub description: Option<String>,

    /// The version of the theme, following semantic versioning.
    pub version: Version,
}

impl Theme {
    pub fn get(&self, index: &Indexer) -> OkHsla {
        match index {
            Indexer::Base(sat, temp, lum) => {
                self.base_palette[*sat as usize][*temp as usize][*lum as usize].with_a(1.0)
            }
            Indexer::Themed(variant, sat, hue, level) => {
                let temp = hue.to_temp(*variant);
                let lum = level.to_lum(*variant);
                self.base_palette[*sat as usize][temp as usize][lum as usize].with_a(1.0)
            }
            Indexer::Primary(sat, primary, lum) => {
                let temp = self.primaries[&primary];
                self.base_palette[*sat as usize][temp as usize][*lum as usize].with_a(1.0)
            }
            Indexer::ThemedPrimary(variant, sat, primary, level) => {
                let temp = self.primaries[&primary];
                let lum = level.to_lum(*variant);
                self.base_palette[*sat as usize][temp as usize][lum as usize].with_a(1.0)
            }
            Indexer::Semantic(variant, name) => {
                let mut ctrl_hue = ThemeHue::Color1;

                while ctrl_hue.to_temp(*variant) == self.primaries[&Primary::Red]
                    || ctrl_hue.to_temp(*variant) == self.primaries[&Primary::Yellow]
                    || ctrl_hue.to_temp(*variant) == self.primaries[&Primary::Green]
                {
                    ctrl_hue = ctrl_hue.next();
                }

                let mut secondary_hue = ctrl_hue.next();

                while secondary_hue.to_temp(*variant) == self.primaries[&Primary::Red]
                    || secondary_hue.to_temp(*variant) == self.primaries[&Primary::Blue]
                    || secondary_hue.to_temp(*variant) == self.primaries[&Primary::Green]
                    || secondary_hue == ctrl_hue
                {
                    secondary_hue = secondary_hue.next();
                }

                let accent_level = Level::HigherMidground;
                let ctrl_fg_level = Level::Foreground;
                let ctrl_bg_level = Level::LowMidground;
                let link_level = Level::HighMidground;
                let disabled_fg_level = Level::HighMidground;
                let disabled_bg_level = Level::HighBackground;
                let dsabled_link_level = Level::LowMidground;

                let (index, alpha) = match name.as_str() {
                    "primary-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                        1.0,
                    ),
                    "intense-foreground" => (
                        Indexer::Themed(*variant, Sat::Intense, ThemeHue::Color6, Level::HighForeground),
                        1.0,
                    ),
                    "dim-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, Level::LowMidground),
                        1.0,
                    ),
                    "content-background" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::Background),
                        1.0,
                    ),
                    "card-background" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground),
                        1.0,
                    ),
                    "page-background" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "selection-background" => (
                        Indexer::Themed(*variant, Sat::Base, secondary_hue, Level::LowerMidground),
                        1.0,
                    ),
                    "dim-selection-background" => (
                        Indexer::Themed(*variant, Sat::Base, secondary_hue, Level::HighBackground),
                        1.0,
                    ),
                    "content-background-transparent" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::Background),
                        BACKGROUND_ALPHA,
                    ),
                    "card-background-transparent" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "chrome-foreground" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                        1.0,
                    ),
                    "chrome-high" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                        1.0,
                    ),
                    "chrome" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::HighMidground),
                        1.0,
                    ),
                    "chrome-low" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::LowMidground),
                        1.0,
                    ),
                    "chrome-background" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::HighBackground),
                        1.0,
                    ),
                    "caption-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "chrome-shadow" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Lum::DimHigh.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),
                    "dim-shadow" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, Lum::DimHigh.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),
                    "intense-shadow" => (
                        Indexer::Themed(*variant, Sat::Intense, ThemeHue::Color6, Lum::DimLow.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),

                    "link" => (Indexer::Themed(*variant, Sat::Intense, ctrl_hue, link_level), 1.0),
                    "link-hover" => (
                        Indexer::Themed(*variant, Sat::Intense, ctrl_hue, link_level.brighter(*variant)),
                        1.0,
                    ),
                    "link-active" => (
                        Indexer::Themed(*variant, Sat::Intense, ctrl_hue, link_level.dimmer(*variant)),
                        1.0,
                    ),
                    "link-disabled" => (Indexer::Themed(*variant, Sat::Base, ctrl_hue, dsabled_link_level), 1.0),

                    "label" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                        1.0,
                    ),
                    "label-hover" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Muted,
                            ThemeHue::Color6,
                            Level::Foreground.brighter(*variant),
                        ),
                        1.0,
                    ),
                    "label-active" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Muted,
                            ThemeHue::Color6,
                            Level::Foreground.dimmer(*variant),
                        ),
                        1.0,
                    ),
                    "label-disabled" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, disabled_fg_level),
                        1.0,
                    ),

                    "control-foreground" => (Indexer::Themed(*variant, Sat::Muted, ctrl_hue, ctrl_fg_level), 1.0),
                    "control-background" => (Indexer::Themed(*variant, Sat::Base, ctrl_hue, ctrl_bg_level), 1.0),
                    "control-hover-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, ctrl_hue, ctrl_fg_level.brighter(*variant)),
                        1.0,
                    ),
                    "control-hover-background" => (
                        Indexer::Themed(*variant, Sat::Base, ctrl_hue, ctrl_bg_level.brighter(*variant)),
                        1.0,
                    ),
                    "control-active-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, ctrl_hue, ctrl_fg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "control-active-background" => (
                        Indexer::Themed(*variant, Sat::Base, ctrl_hue, ctrl_bg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "control-disabled-foreground" => {
                        (Indexer::Themed(*variant, Sat::Muted, ctrl_hue, disabled_fg_level), 1.0)
                    }
                    "control-disabled-background" => {
                        (Indexer::Themed(*variant, Sat::Muted, ctrl_hue, disabled_bg_level), 1.0)
                    }

                    "secondary-control-foreground" => {
                        (Indexer::Themed(*variant, Sat::Muted, secondary_hue, ctrl_fg_level), 1.0)
                    }
                    "secondary-control-background" => {
                        (Indexer::Themed(*variant, Sat::Base, secondary_hue, ctrl_bg_level), 1.0)
                    }
                    "secondary-control-hover-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, secondary_hue, ctrl_fg_level.brighter(*variant)),
                        1.0,
                    ),
                    "secondary-control-hover-background" => (
                        Indexer::Themed(*variant, Sat::Base, secondary_hue, ctrl_bg_level.brighter(*variant)),
                        1.0,
                    ),
                    "secondary-control-active-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, secondary_hue, ctrl_fg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "secondary-control-active-background" => (
                        Indexer::Themed(*variant, Sat::Base, secondary_hue, ctrl_bg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "secondary-control-disabled-foreground" => (
                        Indexer::Themed(*variant, Sat::Muted, secondary_hue, disabled_fg_level),
                        1.0,
                    ),
                    "secondary-control-disabled-background" => (
                        Indexer::Themed(*variant, Sat::Muted, secondary_hue, disabled_bg_level),
                        1.0,
                    ),

                    "ok-control-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, ctrl_fg_level),
                        1.0,
                    ),
                    "ok-control-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, ctrl_bg_level),
                        1.0,
                    ),
                    "ok-control-hover-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, ctrl_fg_level.brighter(*variant)),
                        1.0,
                    ),
                    "ok-control-hover-background" => (
                        Indexer::ThemedPrimary(
                            *variant,
                            Sat::Intense,
                            Primary::Green,
                            ctrl_bg_level.brighter(*variant),
                        ),
                        1.0,
                    ),
                    "ok-control-active-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, ctrl_fg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "ok-control-active-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, ctrl_bg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "ok-control-disabled-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Green, disabled_fg_level),
                        1.0,
                    ),
                    "ok-control-disabled-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Green, disabled_bg_level),
                        1.0,
                    ),

                    "caution-control-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, ctrl_fg_level),
                        1.0,
                    ),
                    "caution-control-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Yellow, ctrl_bg_level),
                        1.0,
                    ),
                    "caution-control-hover-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, ctrl_fg_level.brighter(*variant)),
                        1.0,
                    ),
                    "caution-control-hover-background" => (
                        Indexer::ThemedPrimary(
                            *variant,
                            Sat::Intense,
                            Primary::Yellow,
                            ctrl_bg_level.brighter(*variant),
                        ),
                        1.0,
                    ),
                    "caution-control-active-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, ctrl_fg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "caution-control-active-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, ctrl_bg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "caution-control-disabled-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Yellow, disabled_fg_level),
                        1.0,
                    ),
                    "caution-control-disabled-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Yellow, disabled_bg_level),
                        1.0,
                    ),

                    "danger-control-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, ctrl_fg_level),
                        1.0,
                    ),
                    "danger-control-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, ctrl_bg_level),
                        1.0,
                    ),
                    "danger-control-hover-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, ctrl_fg_level.brighter(*variant)),
                        1.0,
                    ),
                    "danger-control-hover-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, ctrl_bg_level.brighter(*variant)),
                        1.0,
                    ),
                    "danger-control-active-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, ctrl_fg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "danger-control-active-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, ctrl_bg_level.dimmer(*variant)),
                        1.0,
                    ),
                    "danger-control-disabled-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Red, disabled_fg_level),
                        1.0,
                    ),
                    "danger-control-disabled-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Red, disabled_bg_level),
                        1.0,
                    ),

                    "ok-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, accent_level),
                        1.0,
                    ),
                    "ok-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Level::HighBackground),
                        1.0,
                    ),
                    "ok-background-transparent" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "ok-shadow" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Lum::DimHigh.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),

                    "caution-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Yellow, accent_level),
                        1.0,
                    ),
                    "caution-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Level::HighBackground),
                        1.0,
                    ),
                    "caution-background-transparent" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "caution-shadow" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Lum::DimHigh.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),

                    "danger-foreground" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, accent_level),
                        1.0,
                    ),
                    "danger-background" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, Level::HighBackground),
                        1.0,
                    ),
                    "danger-background-transparent" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, Level::HighBackground),
                        BACKGROUND_ALPHA,
                    ),
                    "danger-shadow" => (
                        Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, Lum::DimHigh.to_level(*variant)),
                        SHADOW_ALPHA,
                    ),

                    "input-background" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::Background),
                        1.0,
                    ),
                    "input-border" => (
                        Indexer::Themed(*variant, Sat::Base, ThemeHue::Color1, Level::LowMidground),
                        1.0,
                    ),
                    "input-hover-background" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Base,
                            ThemeHue::Color1,
                            Level::Background.brighter(*variant),
                        ),
                        1.0,
                    ),
                    "input-hover-border" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Base,
                            ThemeHue::Color1,
                            Level::LowMidground.brighter(*variant),
                        ),
                        1.0,
                    ),
                    "input-active-background" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Base,
                            ThemeHue::Color1,
                            Level::Background.dimmer(*variant),
                        ),
                        1.0,
                    ),
                    "input-active-border" => (
                        Indexer::Themed(
                            *variant,
                            Sat::Intense,
                            ThemeHue::Color1,
                            Level::LowMidground.dimmer(*variant),
                        ),
                        1.0,
                    ),

                    "dark-black" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                        1.0,
                    ),
                    "bright-black" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowerMidground),
                        1.0,
                    ),
                    "dark-white" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color5, Level::HigherMidground),
                        1.0,
                    ),
                    "bright-white" => (
                        Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color5, Level::HighForeground),
                        1.0,
                    ),
                    "dark-red" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, Level::HighMidground),
                        1.0,
                    ),
                    "bright-red" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, Level::LowForeground),
                        1.0,
                    ),
                    "dark-green" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, Level::HighMidground),
                        1.0,
                    ),
                    "bright-green" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, Level::LowForeground),
                        1.0,
                    ),
                    "dark-yellow" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Yellow, Level::HighMidground),
                        1.0,
                    ),
                    "bright-yellow" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Yellow, Level::LowForeground),
                        1.0,
                    ),
                    "dark-blue" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                        1.0,
                    ),
                    "bright-blue" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Blue, Level::LowForeground),
                        1.0,
                    ),
                    "dark-magenta" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Magenta, Level::HighMidground),
                        1.0,
                    ),
                    "bright-magenta" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Magenta, Level::LowForeground),
                        1.0,
                    ),
                    "dark-cyan" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Cyan, Level::HighMidground),
                        1.0,
                    ),
                    "bright-cyan" => (
                        Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Cyan, Level::LowForeground),
                        1.0,
                    ),
                    "comment" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, Level::HigherMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "comment-symbol" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Green, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "keyword" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Cyan, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "control-keyword" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Blue, Level::HighMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "directive" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Blue, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color6, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "operator" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color5, Level::HigherMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "variable" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "argument" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Muted, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::Themed(*variant, Sat::Base, ThemeHue::Color5, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "literal" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "unit" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Red, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "string" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Magenta, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "string-delimiter" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Magenta, Level::LowMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Red, Level::LowMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "escape" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "function" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Level::Foreground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "type" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Green, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Cyan, Level::LowForeground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "namespace" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Magenta, Level::HigherMidground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::Themed(*variant, Sat::Muted, ThemeHue::Color1, Level::LowForeground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "macro" => match self.code.vocab {
                        ColorVocabulary::JonStyle => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Yellow, Level::LowForeground),
                            1.0,
                        ),
                        ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Base, Primary::Magenta, Level::HighMidground),
                            1.0,
                        ),
                        _ => todo!(),
                    },
                    "markup" => match self.code.vocab {
                        ColorVocabulary::JonStyle | ColorVocabulary::VisualStudio => (
                            Indexer::ThemedPrimary(*variant, Sat::Intense, Primary::Blue, Level::LowForeground),
                            1.0,
                        ),
                        _ => todo!(),
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

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            author: None,
            version: Version::new(0, 1, 0),
            description: None,
            variants: vec![
                ThemeVariant::DAWN,
                ThemeVariant::DUSK,
                ThemeVariant::NOON,
                ThemeVariant::NIGHT,
            ],
        }
    }
}
