mod builder;
mod index;
mod primary;
mod variant;

use std::{collections::HashMap, ops::Index};

use semver::Version;

pub use builder::*;
pub use index::*;
pub use primary::*;
pub use variant::*;

use crate::color::OkHsl;

pub type BasePalette = [[[OkHsl; LUM_COUNT]; TEMP_COUNT]; SAT_COUNT];
pub type PrimaryMap = HashMap<Primary, Temp>;

pub struct Theme {
    /// The name of the theme.
    pub name: String,

    /// The author of the theme.
    pub author: Option<String>,

    /// An optional description of the theme.
    pub description: Option<String>,

    /// The version of the theme, following semantic versioning.
    pub version: Version,

    /// The hue range in degrees for the theme's cool colors.
    base_palette: BasePalette,

    /// Mapping of primary colors to their associated temperatures.
    primaries: PrimaryMap,
}

impl Index<Indexer> for Theme {
    type Output = OkHsl;

    fn index(&self, index: Indexer) -> &Self::Output {
        let mut ctrl_hue = ThemeHue::Color1;

        while self.primaries[&Primary::Red] == self.primaries[&Primary::Green]
            || self.primaries[&Primary::Red] == self.primaries[&Primary::Blue]
            || self.primaries[&Primary::Green] == self.primaries[&Primary::Blue]
        {
            ctrl_hue = ctrl_hue.next();
        }

        let mut secondary_hue = ctrl_hue.next();

        while self.primaries[&Primary::Red] == self.primaries[&Primary::Green]
            || self.primaries[&Primary::Red] == self.primaries[&Primary::Blue]
            || self.primaries[&Primary::Green] == self.primaries[&Primary::Blue]
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

        match index {
            Indexer::Base(sat, temp, lum) => &self.base_palette[sat as usize][temp as usize][lum as usize],
            Indexer::Themed(variant, sat, hue, level) => {
                let temp = hue.to_temp(variant);
                let lum = level.to_lum(variant);
                &self.base_palette[sat as usize][temp as usize][lum as usize]
            }
            Indexer::Primary(sat, primary, lum) => {
                let temp = self.primaries[&primary];
                &self.base_palette[sat as usize][temp as usize][lum as usize]
            }
            Indexer::ThemedPrimary(variant, sat, primary, level) => {
                let temp = self.primaries[&primary];
                let lum = level.to_lum(variant);
                &self.base_palette[sat as usize][temp as usize][lum as usize]
            }
            Indexer::Semantic(variant, name) => {
                let indexer = match name.as_str() {
                    "primary-foreground" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                    "intense-foreground" => {
                        Indexer::Themed(variant, Sat::Intense, ThemeHue::Color6, Level::HighForeground)
                    }
                    "dim-foreground" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::LowMidground),
                    "content-background" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::Background),
                    "card-background" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground),
                    "page-background" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                    "selection-background" => Indexer::Themed(variant, Sat::Base, secondary_hue, Level::LowerMidground),
                    "dim-selection-background" => {
                        Indexer::Themed(variant, Sat::Base, secondary_hue, Level::HighBackground)
                    }
                    "content-background-transparent" => {
                        Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::Background)
                    }
                    "card-background-transparent" => {
                        Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::HighBackground)
                    }
                    "chrome-foreground" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                    "chrome-high" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowForeground),
                    "chrome" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::HighMidground),
                    "chrome-low" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowMidground),
                    "chrome-background" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::HighBackground),
                    "caption-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground)
                    }
                    "chrome-shadow" => {
                        Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Lum::DimHigh.to_level(variant))
                    }
                    "dim-shadow" => {
                        Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, Lum::DimHigh.to_level(variant))
                    }
                    "intense-shadow" => {
                        Indexer::Themed(variant, Sat::Intense, ThemeHue::Color6, Lum::DimLow.to_level(variant))
                    }

                    "link" => Indexer::Themed(variant, Sat::Intense, ctrl_hue, link_level),
                    "link-hover" => Indexer::Themed(variant, Sat::Intense, ctrl_hue, link_level.brighter(variant)),
                    "link-active" => Indexer::Themed(variant, Sat::Intense, ctrl_hue, link_level.dimmer(variant)),
                    "link-disabled" => Indexer::Themed(variant, Sat::Base, ctrl_hue, dsabled_link_level),

                    "label" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground),
                    "label-hover" => Indexer::Themed(
                        variant,
                        Sat::Muted,
                        ThemeHue::Color6,
                        Level::Foreground.brighter(variant),
                    ),
                    "label-active" => {
                        Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, Level::Foreground.dimmer(variant))
                    }
                    "label-disabled" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color6, disabled_fg_level),

                    "control-foreground" => Indexer::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level),
                    "control-background" => Indexer::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level),
                    "control-hover-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level.brighter(variant))
                    }
                    "control-hover-background" => {
                        Indexer::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level.brighter(variant))
                    }
                    "control-active-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, ctrl_hue, ctrl_fg_level.dimmer(variant))
                    }
                    "control-active-background" => {
                        Indexer::Themed(variant, Sat::Base, ctrl_hue, ctrl_bg_level.dimmer(variant))
                    }
                    "control-disabled-foreground" => Indexer::Themed(variant, Sat::Muted, ctrl_hue, disabled_fg_level),
                    "control-disabled-background" => Indexer::Themed(variant, Sat::Muted, ctrl_hue, disabled_bg_level),

                    "secondary-control-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, secondary_hue, ctrl_fg_level)
                    }
                    "secondary-control-background" => Indexer::Themed(variant, Sat::Base, secondary_hue, ctrl_bg_level),
                    "secondary-control-hover-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, secondary_hue, ctrl_fg_level.brighter(variant))
                    }
                    "secondary-control-hover-background" => {
                        Indexer::Themed(variant, Sat::Base, secondary_hue, ctrl_bg_level.brighter(variant))
                    }
                    "secondary-control-active-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, secondary_hue, ctrl_fg_level.dimmer(variant))
                    }
                    "secondary-control-active-background" => {
                        Indexer::Themed(variant, Sat::Base, secondary_hue, ctrl_bg_level.dimmer(variant))
                    }
                    "secondary-control-disabled-foreground" => {
                        Indexer::Themed(variant, Sat::Muted, secondary_hue, disabled_fg_level)
                    }
                    "secondary-control-disabled-background" => {
                        Indexer::Themed(variant, Sat::Muted, secondary_hue, disabled_bg_level)
                    }

                    "ok-control-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, ctrl_fg_level)
                    }
                    "ok-control-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Green, ctrl_bg_level)
                    }
                    "ok-control-hover-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, ctrl_fg_level.brighter(variant))
                    }
                    "ok-control-hover-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Green, ctrl_bg_level.brighter(variant))
                    }
                    "ok-control-active-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, ctrl_fg_level.dimmer(variant))
                    }
                    "ok-control-active-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, ctrl_bg_level.dimmer(variant))
                    }
                    "ok-control-disabled-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Green, disabled_fg_level)
                    }
                    "ok-control-disabled-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Green, disabled_bg_level)
                    }

                    "caution-control-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, ctrl_fg_level)
                    }
                    "caution-control-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, ctrl_bg_level)
                    }
                    "caution-control-hover-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, ctrl_fg_level.brighter(variant))
                    }
                    "caution-control-hover-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, ctrl_bg_level.brighter(variant))
                    }
                    "caution-control-active-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, ctrl_fg_level.dimmer(variant))
                    }
                    "caution-control-active-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, ctrl_bg_level.dimmer(variant))
                    }
                    "caution-control-disabled-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Yellow, disabled_fg_level)
                    }
                    "caution-control-disabled-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Yellow, disabled_bg_level)
                    }

                    "danger-control-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, ctrl_fg_level)
                    }
                    "danger-control-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Red, ctrl_bg_level)
                    }
                    "danger-control-hover-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, ctrl_fg_level.brighter(variant))
                    }
                    "danger-control-hover-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Red, ctrl_bg_level.brighter(variant))
                    }
                    "danger-control-active-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, ctrl_fg_level.dimmer(variant))
                    }
                    "danger-control-active-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, ctrl_bg_level.dimmer(variant))
                    }
                    "danger-control-disabled-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Red, disabled_fg_level)
                    }
                    "danger-control-disabled-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Muted, Primary::Red, disabled_bg_level)
                    }

                    "ok-foreground" => Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Green, accent_level),
                    "ok-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HighBackground)
                    }
                    "ok-background-transparent" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HighBackground)
                    }
                    "ok-shadow" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, Lum::DimHigh.to_level(variant))
                    }

                    "caution-foreground" => {
                        Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Yellow, accent_level)
                    }
                    "caution-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighBackground)
                    }
                    "caution-background-transparent" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighBackground)
                    }
                    "caution-shadow" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Lum::DimHigh.to_level(variant))
                    }

                    "danger-foreground" => Indexer::ThemedPrimary(variant, Sat::Intense, Primary::Red, accent_level),
                    "danger-background" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighBackground)
                    }
                    "danger-background-transparent" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighBackground)
                    }
                    "danger-shadow" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, Lum::DimHigh.to_level(variant))
                    }

                    "input-background" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::Background),
                    "input-border" => Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::LowMidground),
                    "input-hover-background" => Indexer::Themed(
                        variant,
                        Sat::Base,
                        ThemeHue::Color1,
                        Level::Background.brighter(variant),
                    ),
                    "input-hover-border" => Indexer::Themed(
                        variant,
                        Sat::Base,
                        ThemeHue::Color1,
                        Level::LowMidground.brighter(variant),
                    ),
                    "input-active-background" => {
                        Indexer::Themed(variant, Sat::Base, ThemeHue::Color1, Level::Background.dimmer(variant))
                    }
                    "input-active-border" => Indexer::Themed(
                        variant,
                        Sat::Intense,
                        ThemeHue::Color1,
                        Level::LowMidground.dimmer(variant),
                    ),

                    "dark-black" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowBackground),
                    "bright-black" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color1, Level::LowerMidground),
                    "dark-white" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HigherMidground),
                    "bright-white" => Indexer::Themed(variant, Sat::Muted, ThemeHue::Color5, Level::HighForeground),
                    "dark-red" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighBackground),
                    "bright-red" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Red, Level::HighMidground),
                    "dark-green" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HighBackground),
                    "bright-green" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Green, Level::HighMidground),
                    "dark-yellow" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighBackground),
                    "bright-yellow" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Yellow, Level::HighMidground)
                    }
                    "dark-blue" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::HighBackground),
                    "bright-blue" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Blue, Level::HighMidground),
                    "dark-magenta" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighBackground)
                    }
                    "bright-magenta" => {
                        Indexer::ThemedPrimary(variant, Sat::Base, Primary::Magenta, Level::HighMidground)
                    }
                    "dark-cyan" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::HighBackground),
                    "bright-cyan" => Indexer::ThemedPrimary(variant, Sat::Base, Primary::Cyan, Level::HighMidground),

                    _ => panic!("Unknown semantic color name: {}", name),
                };

                &self[indexer]
            }
        }
    }
}
