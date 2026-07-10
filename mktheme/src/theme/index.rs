use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use anyhow::{anyhow, Error as AnyError};
use lazy_static::lazy_static;

use crate::theme::{Primary, ThemeMode, ThemeTemperature, ThemeVariant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Indexer {
    Base(Sat, Temp, Lum),
    Themed(ThemeVariant, Sat, ThemeHue, Level),
    Primary(Sat, Primary, Lum),
    ThemedPrimary(ThemeVariant, Sat, Primary, Level),
    Semantic(ThemeVariant, String),
}

/// A hue from the base palette, which is unmodified by the theme variant and
/// proceeds from cold to hot.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Temp {
    #[default]
    Cold = 0,
    Cool = 1,
    Coolish = 2,
    Warmish = 3,
    Warm = 4,
    Hot = 5,
}

pub const TEMP_COUNT: usize = 6;

/// A luminance level from the base palette, which is unmodified by the theme
/// variant and proceeds from dim to bright.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lum {
    BrightHigh = 9,
    BrightMedium = 8,
    BrightLow = 7,
    #[default]
    MediumHigher = 6,
    MediumHigh = 5,
    MediumLow = 4,
    MediumLower = 3,
    DimHigh = 2,
    DimMedium = 1,
    DimLow = 0,
}

pub const LUM_COUNT: usize = 10;

/// A color hue from the themed palette, which will proceed from cold to hot in
/// a cool theme and from hot to cold in a warm theme.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThemeHue {
    #[default]
    Color1 = 0,
    Color2 = 1,
    Color3 = 2,
    Color4 = 3,
    Color5 = 4,
    Color6 = 5,
}

/// A saturation level. There are only three levels of saturation in the
/// palette, and they are not modified by the theme variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sat {
    Muted = 0,
    #[default]
    Base = 1,
    Intense = 2,
}

pub const SAT_COUNT: usize = 3;

/// A semantic level from the themed palette, which maps luminosity to semantic
/// roles. "Dark" themes will have low luminance values for background and high
/// luminance values for foreground, while "light" themes will have the
/// opposite.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    HighForeground = 9,
    #[default]
    Foreground = 8,
    LowForeground = 7,
    HigherMidground = 6,
    HighMidground = 5,
    LowMidground = 4,
    LowerMidground = 3,
    HighBackground = 2,
    Background = 1,
    LowBackground = 0,
}

lazy_static! {
    pub static ref KNOWN_SEMANTICS: HashSet<String> = {
        let mut set = HashSet::new();
        set.insert("caption-foreground".to_string());
        set.insert("card-background-transparent".to_string());
        set.insert("card-background".to_string());
        set.insert("caution-background-transparent".to_string());
        set.insert("caution-background".to_string());
        set.insert("caution-control-active-background".to_string());
        set.insert("caution-control-active-foreground".to_string());
        set.insert("caution-control-background".to_string());
        set.insert("caution-control-disabled-background".to_string());
        set.insert("caution-control-disabled-foreground".to_string());
        set.insert("caution-control-foreground".to_string());
        set.insert("caution-control-hover-background".to_string());
        set.insert("caution-control-hover-foreground".to_string());
        set.insert("caution-foreground".to_string());
        set.insert("caution-shadow".to_string());
        set.insert("chrome-background".to_string());
        set.insert("chrome-foreground".to_string());
        set.insert("chrome-high".to_string());
        set.insert("chrome-low".to_string());
        set.insert("chrome-shadow".to_string());
        set.insert("chrome".to_string());
        set.insert("content-background-transparent".to_string());
        set.insert("content-background".to_string());
        set.insert("control-active-background".to_string());
        set.insert("control-active-foreground".to_string());
        set.insert("control-background".to_string());
        set.insert("control-disabled-background".to_string());
        set.insert("control-disabled-foreground".to_string());
        set.insert("control-foreground".to_string());
        set.insert("control-hover-background".to_string());
        set.insert("control-hover-foreground".to_string());
        set.insert("danger-background-transparent".to_string());
        set.insert("danger-background".to_string());
        set.insert("danger-control-active-background".to_string());
        set.insert("danger-control-active-foreground".to_string());
        set.insert("danger-control-background".to_string());
        set.insert("danger-control-disabled-background".to_string());
        set.insert("danger-control-disabled-foreground".to_string());
        set.insert("danger-control-foreground".to_string());
        set.insert("danger-control-hover-background".to_string());
        set.insert("danger-control-hover-foreground".to_string());
        set.insert("danger-foreground".to_string());
        set.insert("danger-shadow".to_string());
        set.insert("dim-foreground".to_string());
        set.insert("dim-selection-background".to_string());
        set.insert("dim-shadow".to_string());
        set.insert("input-active-background".to_string());
        set.insert("input-active-border".to_string());
        set.insert("input-background".to_string());
        set.insert("input-border".to_string());
        set.insert("input-hover-background".to_string());
        set.insert("input-hover-border".to_string());
        set.insert("intense-foreground".to_string());
        set.insert("intense-shadow".to_string());
        set.insert("label-active".to_string());
        set.insert("label-disabled".to_string());
        set.insert("label-hover".to_string());
        set.insert("label".to_string());
        set.insert("link-active".to_string());
        set.insert("link-disabled".to_string());
        set.insert("link-hover".to_string());
        set.insert("link".to_string());
        set.insert("ok-background-transparent".to_string());
        set.insert("ok-background".to_string());
        set.insert("ok-control-active-background".to_string());
        set.insert("ok-control-active-foreground".to_string());
        set.insert("ok-control-background".to_string());
        set.insert("ok-control-disabled-background".to_string());
        set.insert("ok-control-disabled-foreground".to_string());
        set.insert("ok-control-foreground".to_string());
        set.insert("ok-control-hover-background".to_string());
        set.insert("ok-control-hover-foreground".to_string());
        set.insert("ok-foreground".to_string());
        set.insert("ok-shadow".to_string());
        set.insert("page-background".to_string());
        set.insert("primary-foreground".to_string());
        set.insert("secondary-control-active-background".to_string());
        set.insert("secondary-control-active-foreground".to_string());
        set.insert("secondary-control-background".to_string());
        set.insert("secondary-control-disabled-background".to_string());
        set.insert("secondary-control-disabled-foreground".to_string());
        set.insert("secondary-control-foreground".to_string());
        set.insert("secondary-control-hover-background".to_string());
        set.insert("secondary-control-hover-foreground".to_string());
        set.insert("selection-background".to_string());
        set.insert("dark-black".to_string());
        set.insert("bright-black".to_string());
        set.insert("dark-white".to_string());
        set.insert("bright-white".to_string());
        set.insert("dark-red".to_string());
        set.insert("bright-red".to_string());
        set.insert("dark-green".to_string());
        set.insert("bright-green".to_string());
        set.insert("dark-yellow".to_string());
        set.insert("bright-yellow".to_string());
        set.insert("dark-blue".to_string());
        set.insert("bright-blue".to_string());
        set.insert("dark-magenta".to_string());
        set.insert("bright-magenta".to_string());
        set.insert("dark-cyan".to_string());
        set.insert("bright-cyan".to_string());
        set
    };
}

impl Indexer {
    pub fn from_str_with_variant(s: &str, variant: ThemeVariant) -> Result<Self, AnyError> {
        let parts = s.split('-').collect::<Vec<_>>();

        if let Ok(sat) = Sat::from_str(parts[0]) {
            if let Ok(temp) = Temp::from_str(parts[1]) {
                let rest = parts[2..].join("-");
                if let Ok(lum) = Lum::from_str(&rest) {
                    return Ok(Indexer::Base(sat, temp, lum));
                }
            } else if let Ok(primary) = Primary::from_str(parts[1]) {
                let rest = parts[2..].join("-");
                if let Ok(lum) = Lum::from_str(&rest) {
                    return Ok(Indexer::Primary(sat, primary, lum));
                } else if let Ok(level) = Level::from_str(&rest) {
                    return Ok(Indexer::ThemedPrimary(variant, sat, primary, level));
                }
            } else if let Ok(hue) = ThemeHue::from_str(parts[1]) {
                let rest = parts[2..].join("-");
                if let Ok(level) = Level::from_str(&rest) {
                    return Ok(Indexer::Themed(variant, sat, hue, level));
                }
            }
        }

        if KNOWN_SEMANTICS.contains(s) {
            Ok(Indexer::Semantic(variant, s.to_string()))
        } else {
            Err(anyhow!("{s} is not a valid indexer name"))
        }
    }
}

impl Temp {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Temp::Cold,
            Temp::Cool,
            Temp::Coolish,
            Temp::Warmish,
            Temp::Warm,
            Temp::Hot,
        ]
        .into_iter()
    }

    pub fn to_theme_hue(&self, variant: ThemeVariant) -> ThemeHue {
        match variant.temperature {
            ThemeTemperature::Cool => match self {
                Temp::Cold => ThemeHue::Color1,
                Temp::Cool => ThemeHue::Color2,
                Temp::Coolish => ThemeHue::Color3,
                Temp::Warmish => ThemeHue::Color4,
                Temp::Warm => ThemeHue::Color5,
                Temp::Hot => ThemeHue::Color6,
            },
            ThemeTemperature::Warm => match self {
                Temp::Cold => ThemeHue::Color6,
                Temp::Cool => ThemeHue::Color5,
                Temp::Coolish => ThemeHue::Color4,
                Temp::Warmish => ThemeHue::Color3,
                Temp::Warm => ThemeHue::Color2,
                Temp::Hot => ThemeHue::Color1,
            },
        }
    }
}

impl ThemeHue {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            ThemeHue::Color1,
            ThemeHue::Color2,
            ThemeHue::Color3,
            ThemeHue::Color4,
            ThemeHue::Color5,
            ThemeHue::Color6,
        ]
        .into_iter()
    }

    pub fn to_temp(&self, variant: ThemeVariant) -> Temp {
        match variant.temperature {
            ThemeTemperature::Cool => match self {
                ThemeHue::Color1 => Temp::Cold,
                ThemeHue::Color2 => Temp::Cool,
                ThemeHue::Color3 => Temp::Coolish,
                ThemeHue::Color4 => Temp::Warmish,
                ThemeHue::Color5 => Temp::Warm,
                ThemeHue::Color6 => Temp::Hot,
            },
            ThemeTemperature::Warm => match self {
                ThemeHue::Color1 => Temp::Hot,
                ThemeHue::Color2 => Temp::Warm,
                ThemeHue::Color3 => Temp::Warmish,
                ThemeHue::Color4 => Temp::Coolish,
                ThemeHue::Color5 => Temp::Cool,
                ThemeHue::Color6 => Temp::Cold,
            },
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeHue::Color1 => ThemeHue::Color2,
            ThemeHue::Color2 => ThemeHue::Color3,
            ThemeHue::Color3 => ThemeHue::Color4,
            ThemeHue::Color4 => ThemeHue::Color5,
            ThemeHue::Color5 => ThemeHue::Color6,
            ThemeHue::Color6 => ThemeHue::Color1,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            ThemeHue::Color1 => ThemeHue::Color6,
            ThemeHue::Color2 => ThemeHue::Color1,
            ThemeHue::Color3 => ThemeHue::Color2,
            ThemeHue::Color4 => ThemeHue::Color3,
            ThemeHue::Color5 => ThemeHue::Color4,
            ThemeHue::Color6 => ThemeHue::Color5,
        }
    }
}

impl Lum {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Lum::BrightHigh,
            Lum::BrightMedium,
            Lum::BrightLow,
            Lum::MediumHigher,
            Lum::MediumHigh,
            Lum::MediumLow,
            Lum::MediumLower,
            Lum::DimHigh,
            Lum::DimMedium,
            Lum::DimLow,
        ]
        .into_iter()
    }

    pub fn to_level(&self, variant: ThemeVariant) -> Level {
        match variant.mode {
            ThemeMode::Dark => match self {
                Lum::BrightHigh => Level::HighForeground,
                Lum::BrightMedium => Level::Foreground,
                Lum::BrightLow => Level::LowForeground,
                Lum::MediumHigher => Level::HigherMidground,
                Lum::MediumHigh => Level::HighMidground,
                Lum::MediumLow => Level::LowMidground,
                Lum::MediumLower => Level::LowerMidground,
                Lum::DimHigh => Level::HighBackground,
                Lum::DimMedium => Level::Background,
                Lum::DimLow => Level::LowBackground,
            },
            ThemeMode::Light => match self {
                Lum::BrightHigh => Level::LowBackground,
                Lum::BrightMedium => Level::Background,
                Lum::BrightLow => Level::HighBackground,
                Lum::MediumHigher => Level::LowerMidground,
                Lum::MediumHigh => Level::LowMidground,
                Lum::MediumLow => Level::HighMidground,
                Lum::MediumLower => Level::HigherMidground,
                Lum::DimHigh => Level::LowForeground,
                Lum::DimMedium => Level::Foreground,
                Lum::DimLow => Level::HighForeground,
            },
        }
    }

    /// Returns the next brighter luminance, or the same luminance if it's
    /// already the brightest.
    pub fn brighter(&self) -> Self {
        match self {
            Lum::BrightHigh => Lum::BrightHigh,
            Lum::BrightMedium => Lum::BrightHigh,
            Lum::BrightLow => Lum::BrightMedium,
            Lum::MediumHigher => Lum::BrightLow,
            Lum::MediumHigh => Lum::MediumHigher,
            Lum::MediumLow => Lum::MediumHigh,
            Lum::MediumLower => Lum::MediumLow,
            Lum::DimHigh => Lum::MediumLower,
            Lum::DimMedium => Lum::DimHigh,
            Lum::DimLow => Lum::DimMedium,
        }
    }

    /// Returns the next dimmer luminance, or the same luminance if it's already
    /// the dimmest.
    pub fn dimmer(&self) -> Self {
        match self {
            Lum::BrightHigh => Lum::BrightMedium,
            Lum::BrightMedium => Lum::BrightLow,
            Lum::BrightLow => Lum::MediumHigher,
            Lum::MediumHigher => Lum::MediumHigh,
            Lum::MediumHigh => Lum::MediumLow,
            Lum::MediumLow => Lum::MediumLower,
            Lum::MediumLower => Lum::DimHigh,
            Lum::DimHigh => Lum::DimMedium,
            Lum::DimMedium => Lum::DimLow,
            Lum::DimLow => Lum::DimLow,
        }
    }
}

impl Level {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Level::HighForeground,
            Level::Foreground,
            Level::LowForeground,
            Level::HigherMidground,
            Level::HighMidground,
            Level::LowMidground,
            Level::LowerMidground,
            Level::HighBackground,
            Level::Background,
            Level::LowBackground,
        ]
        .into_iter()
    }

    pub fn to_lum(&self, variant: ThemeVariant) -> Lum {
        match variant.mode {
            ThemeMode::Dark => match self {
                Level::HighForeground => Lum::BrightHigh,
                Level::Foreground => Lum::BrightMedium,
                Level::LowForeground => Lum::BrightLow,
                Level::HigherMidground => Lum::MediumHigher,
                Level::HighMidground => Lum::MediumHigh,
                Level::LowMidground => Lum::MediumLow,
                Level::LowerMidground => Lum::MediumLower,
                Level::HighBackground => Lum::DimHigh,
                Level::Background => Lum::DimMedium,
                Level::LowBackground => Lum::DimLow,
            },
            ThemeMode::Light => match self {
                Level::HighForeground => Lum::DimLow,
                Level::Foreground => Lum::DimMedium,
                Level::LowForeground => Lum::DimHigh,
                Level::HigherMidground => Lum::MediumLower,
                Level::HighMidground => Lum::MediumLow,
                Level::LowMidground => Lum::MediumHigh,
                Level::LowerMidground => Lum::MediumHigher,
                Level::HighBackground => Lum::BrightLow,
                Level::Background => Lum::BrightMedium,
                Level::LowBackground => Lum::BrightHigh,
            },
        }
    }

    /// Returns the next brighter level, or the same level if it's already the
    /// brightest. This is absolute brightness, which may ne "lower" or "higher"
    /// depending on the theme mode. This is used for things like hover states.
    pub fn brighter(&self, variant: ThemeVariant) -> Self {
        self.to_lum(variant).brighter().to_level(variant)
    }

    /// Returns the next dimmer level, or the same level if it's already the
    /// dimmest. This is absolute brightness, which may ne "lower" or "higher"
    /// depending on the theme mode. This is used for things like hover states.
    pub fn dimmer(&self, variant: ThemeVariant) -> Self {
        self.to_lum(variant).dimmer().to_level(variant)
    }

    /// Returns the next higher level, or the same level if it's already the
    /// highest.
    pub fn higher(&self) -> Self {
        match self {
            Level::HighForeground => Level::HighForeground,
            Level::Foreground => Level::HighForeground,
            Level::LowForeground => Level::Foreground,
            Level::HigherMidground => Level::HigherMidground,
            Level::HighMidground => Level::HigherMidground,
            Level::LowMidground => Level::HighMidground,
            Level::LowerMidground => Level::LowMidground,
            Level::HighBackground => Level::HighBackground,
            Level::Background => Level::HighBackground,
            Level::LowBackground => Level::Background,
        }
    }

    /// Returns the next lower level, or the same level if it's already the
    /// lowest.
    pub fn lower(&self) -> Self {
        match self {
            Level::HighForeground => Level::Foreground,
            Level::Foreground => Level::LowForeground,
            Level::LowForeground => Level::LowForeground,
            Level::HigherMidground => Level::HighMidground,
            Level::HighMidground => Level::LowMidground,
            Level::LowMidground => Level::LowerMidground,
            Level::LowerMidground => Level::LowerMidground,
            Level::HighBackground => Level::Background,
            Level::Background => Level::LowBackground,
            Level::LowBackground => Level::LowBackground,
        }
    }
}

impl Display for Indexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Indexer::Base(sat, temp, lum) => write!(f, "{sat}-{temp}-{lum}"),
            Indexer::Themed(_, sat, hue, level) => write!(f, "{sat}-{hue}-{level}",),
            Indexer::Primary(sat, primary, lum) => write!(f, "{sat}-{primary}-{lum}"),
            Indexer::ThemedPrimary(_, sat, primary, level) => {
                write!(f, "{sat}-{primary}-{level}")
            }
            Indexer::Semantic(_, role) => write!(f, "{role}"),
        }
    }
}

impl Display for Temp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Temp::Cold => write!(f, "cold"),
            Temp::Cool => write!(f, "cool"),
            Temp::Coolish => write!(f, "coolish"),
            Temp::Warmish => write!(f, "warmish"),
            Temp::Warm => write!(f, "warm"),
            Temp::Hot => write!(f, "hot"),
        }
    }
}

impl FromStr for Temp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cold" => Ok(Temp::Cold),
            "cool" => Ok(Temp::Cool),
            "coolish" => Ok(Temp::Coolish),
            "warmish" => Ok(Temp::Warmish),
            "warm" => Ok(Temp::Warm),
            "hot" => Ok(Temp::Hot),
            _ => Err(()),
        }
    }
}

impl Display for Lum {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Lum::BrightHigh => write!(f, "bright-high"),
            Lum::BrightMedium => write!(f, "bright-medium"),
            Lum::BrightLow => write!(f, "bright-low"),
            Lum::MediumHigher => write!(f, "medium-higher"),
            Lum::MediumHigh => write!(f, "medium-high"),
            Lum::MediumLow => write!(f, "medium-low"),
            Lum::MediumLower => write!(f, "medium-lower"),
            Lum::DimHigh => write!(f, "dim-high"),
            Lum::DimMedium => write!(f, "dim-medium"),
            Lum::DimLow => write!(f, "dim-low"),
        }
    }
}

impl FromStr for Lum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bright-high" => Ok(Lum::BrightHigh),
            "bright-medium" => Ok(Lum::BrightMedium),
            "bright-low" => Ok(Lum::BrightLow),
            "medium-higher" => Ok(Lum::MediumHigher),
            "medium-high" => Ok(Lum::MediumHigh),
            "medium-low" => Ok(Lum::MediumLow),
            "medium-lower" => Ok(Lum::MediumLower),
            "dim-high" => Ok(Lum::DimHigh),
            "dim-medium" => Ok(Lum::DimMedium),
            "dim-low" => Ok(Lum::DimLow),
            _ => Err(()),
        }
    }
}

impl Display for ThemeHue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ThemeHue::Color1 => write!(f, "color-1"),
            ThemeHue::Color2 => write!(f, "color-2"),
            ThemeHue::Color3 => write!(f, "color-3"),
            ThemeHue::Color4 => write!(f, "color-4"),
            ThemeHue::Color5 => write!(f, "color-5"),
            ThemeHue::Color6 => write!(f, "color-6"),
        }
    }
}

impl FromStr for ThemeHue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "color-1" => Ok(ThemeHue::Color1),
            "color-2" => Ok(ThemeHue::Color2),
            "color-3" => Ok(ThemeHue::Color3),
            "color-4" => Ok(ThemeHue::Color4),
            "color-5" => Ok(ThemeHue::Color5),
            "color-6" => Ok(ThemeHue::Color6),
            _ => Err(()),
        }
    }
}

impl Display for Sat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Sat::Muted => write!(f, "muted"),
            Sat::Base => write!(f, "base"),
            Sat::Intense => write!(f, "intense"),
        }
    }
}

impl FromStr for Sat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "muted" => Ok(Sat::Muted),
            "base" => Ok(Sat::Base),
            "intense" => Ok(Sat::Intense),
            _ => Err(()),
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Level::HighForeground => write!(f, "high-foreground"),
            Level::Foreground => write!(f, "foreground"),
            Level::LowForeground => write!(f, "low-foreground"),
            Level::HigherMidground => write!(f, "higher-midground"),
            Level::HighMidground => write!(f, "high-midground"),
            Level::LowMidground => write!(f, "low-midground"),
            Level::LowerMidground => write!(f, "lower-midground"),
            Level::HighBackground => write!(f, "high-background"),
            Level::Background => write!(f, "background"),
            Level::LowBackground => write!(f, "low-background"),
        }
    }
}

impl FromStr for Level {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "high-foreground" => Ok(Level::HighForeground),
            "foreground" => Ok(Level::Foreground),
            "low-foreground" => Ok(Level::LowForeground),
            "higher-midground" => Ok(Level::HigherMidground),
            "high-midground" => Ok(Level::HighMidground),
            "low-midground" => Ok(Level::LowMidground),
            "lower-midground" => Ok(Level::LowerMidground),
            "high-background" => Ok(Level::HighBackground),
            "background" => Ok(Level::Background),
            "low-background" => Ok(Level::LowBackground),
            _ => Err(()),
        }
    }
}

impl From<usize> for Temp {
    fn from(value: usize) -> Self {
        match value {
            0 => Temp::Cold,
            1 => Temp::Cool,
            2 => Temp::Coolish,
            3 => Temp::Warmish,
            4 => Temp::Warm,
            5 => Temp::Hot,
            _ => panic!("Invalid value for Temp: {value}"),
        }
    }
}

impl From<usize> for Lum {
    fn from(value: usize) -> Self {
        match value {
            0 => Lum::DimLow,
            1 => Lum::DimMedium,
            2 => Lum::DimHigh,
            3 => Lum::MediumLower,
            4 => Lum::MediumLow,
            5 => Lum::MediumHigh,
            6 => Lum::MediumHigher,
            7 => Lum::BrightLow,
            8 => Lum::BrightMedium,
            9 => Lum::BrightHigh,
            _ => panic!("Invalid value for Lum: {value}"),
        }
    }
}

impl From<usize> for ThemeHue {
    fn from(value: usize) -> Self {
        match value {
            0 => ThemeHue::Color1,
            1 => ThemeHue::Color2,
            2 => ThemeHue::Color3,
            3 => ThemeHue::Color4,
            4 => ThemeHue::Color5,
            5 => ThemeHue::Color6,
            _ => panic!("Invalid value for ThemeHue: {value}"),
        }
    }
}

impl From<usize> for Sat {
    fn from(value: usize) -> Self {
        match value {
            0 => Sat::Muted,
            1 => Sat::Base,
            2 => Sat::Intense,
            _ => panic!("Invalid value for Sat: {value}"),
        }
    }
}

impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            0 => Level::LowBackground,
            1 => Level::Background,
            2 => Level::HighBackground,
            3 => Level::LowerMidground,
            4 => Level::LowMidground,
            5 => Level::HighMidground,
            6 => Level::HigherMidground,
            7 => Level::LowForeground,
            8 => Level::Foreground,
            9 => Level::HighForeground,
            _ => panic!("Invalid value for Level: {value}"),
        }
    }
}
