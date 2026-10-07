use std::{
    f64::consts::TAU,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Primary {
    #[default]
    Red = 0b001,
    Green = 0b010,
    Yellow = 0b011,
    Blue = 0b100,
    Magenta = 0b101,
    Cyan = 0b110,
}

impl Primary {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Magenta,
            Self::Cyan,
        ]
        .into_iter()
    }

    pub const fn base_hue(&self) -> f64 {
        // In OkHSL, the primary colors are slightly offset from the usual HSL
        // values. We offset by 20 degrees to better align.
        match self {
            Primary::Red => 20.0,
            Primary::Yellow => 80.0,
            Primary::Green => 140.0,
            Primary::Cyan => 200.0,
            Primary::Blue => 260.0,
            Primary::Magenta => 320.0,
        }
    }

    pub fn nearest(hue: f32) -> Self {
        // In OkHSL, the primary colors are slightly offset from the usual HSL
        // values. We offset by 20 degrees to better align.
        let hue = hue - 20.0;
        let hue = hue.rem_euclid(360.0);
        let index = ((hue / 60.0).round() as u8) % 6;

        match index {
            0 => Primary::Red,
            1 => Primary::Yellow,
            2 => Primary::Green,
            3 => Primary::Cyan,
            4 => Primary::Blue,
            5 => Primary::Magenta,
            _ => unreachable!(),
        }
    }

    pub fn distance(&self, hue: f64) -> f64 {
        let hue = hue.rem_euclid(TAU);
        let diff = (hue - self.base_hue().to_radians()).abs();
        diff.min(TAU - diff)
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Primary::Red => write!(f, "red"),
            Primary::Green => write!(f, "green"),
            Primary::Yellow => write!(f, "yellow"),
            Primary::Blue => write!(f, "blue"),
            Primary::Magenta => write!(f, "magenta"),
            Primary::Cyan => write!(f, "cyan"),
        }
    }
}

impl FromStr for Primary {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Primary::Red),
            "green" => Ok(Primary::Green),
            "yellow" => Ok(Primary::Yellow),
            "blue" => Ok(Primary::Blue),
            "magenta" => Ok(Primary::Magenta),
            "cyan" => Ok(Primary::Cyan),
            _ => Err(()),
        }
    }
}
