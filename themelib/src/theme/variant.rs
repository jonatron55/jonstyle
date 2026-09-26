use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThemeVariant {
    pub mode: ThemeMode,
    pub temperature: ThemeTemperature,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeTemperature {
    #[default]
    Cool,
    Warm,
}

impl ThemeVariant {
    pub const DAWN: Self = Self::new(ThemeMode::Light, ThemeTemperature::Warm);
    pub const DUSK: Self = Self::new(ThemeMode::Dark, ThemeTemperature::Warm);
    pub const NOON: Self = Self::new(ThemeMode::Light, ThemeTemperature::Cool);
    pub const NIGHT: Self = Self::new(ThemeMode::Dark, ThemeTemperature::Cool);

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::DAWN, Self::DUSK, Self::NOON, Self::NIGHT].into_iter()
    }

    pub const fn new(mode: ThemeMode, temperature: ThemeTemperature) -> Self {
        Self { mode, temperature }
    }
}

impl ThemeMode {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Light, Self::Dark].into_iter()
    }
}

impl ThemeTemperature {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Cool, Self::Warm].into_iter()
    }
}

impl Display for ThemeVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (self.mode, self.temperature) {
            (ThemeMode::Light, ThemeTemperature::Warm) => write!(f, "Dawn"),
            (ThemeMode::Dark, ThemeTemperature::Warm) => write!(f, "Dusk"),
            (ThemeMode::Light, ThemeTemperature::Cool) => write!(f, "Noon"),
            (ThemeMode::Dark, ThemeTemperature::Cool) => write!(f, "Night"),
        }
    }
}

impl Display for ThemeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::Dark => write!(f, "Dark"),
        }
    }
}

impl Display for ThemeTemperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ThemeTemperature::Cool => write!(f, "Cool"),
            ThemeTemperature::Warm => write!(f, "Warm"),
        }
    }
}

impl FromStr for ThemeVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dawn" => Ok(Self::DAWN),
            "dusk" => Ok(Self::DUSK),
            "noon" => Ok(Self::NOON),
            "night" => Ok(Self::NIGHT),
            _ => Err(format!("Invalid theme variant: {s}")),
        }
    }
}

impl Serialize for ThemeVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ThemeVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Dawn" => Ok(Self::DAWN),
            "Dusk" => Ok(Self::DUSK),
            "Noon" => Ok(Self::NOON),
            "Night" => Ok(Self::NIGHT),
            _ => Err(DeError::custom(format!("invalid theme variant: {s}"))),
        }
    }
}
