use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThemeVariant {
    pub name: String,
    pub mode: ThemeMode,
    pub temp: ThemeTemperature,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeTemperature {
    #[default]
    Cool,
    Warm,
}

impl ThemeVariant {
    pub fn new(name: impl Into<String>, mode: ThemeMode, temp: ThemeTemperature) -> Self {
        Self {
            name: name.into(),
            mode,
            temp,
        }
    }

    pub fn dawn() -> Self {
        Self::new("Dawn", ThemeMode::Light, ThemeTemperature::Warm)
    }

    pub fn dusk() -> Self {
        Self::new("Dusk", ThemeMode::Dark, ThemeTemperature::Warm)
    }

    pub fn noon() -> Self {
        Self::new("Noon", ThemeMode::Light, ThemeTemperature::Cool)
    }

    pub fn night() -> Self {
        Self::new("Night", ThemeMode::Dark, ThemeTemperature::Cool)
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
        write!(f, "{}", self.name)
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
