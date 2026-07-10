use std::{cmp::Ordering, f64::consts::TAU};

use glam::FloatExt;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{
    color::okhsl,
    theme::{BasePalette, Lum, Primary, PrimaryMap, Sat, Temp, Theme, ThemeVariant, LUM_COUNT, SAT_COUNT, TEMP_COUNT},
};

/// A collection of parameters for generating a theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeBuilder {
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

    /// The hue range in degrees for the theme's cool colors.
    pub cool_range: (f64, f64),

    /// The hue range in degrees for the theme's warm colors.
    pub warm_range: (f64, f64),

    /// Amount to offset all hues by in degrees.
    pub offset: f64,

    /// Luminance range for dark colors, as a fraction of 1.
    ///
    /// There are three luminance levels for the dark range: "DarkLow",
    /// "DarkMedium", and "DarkHigh".
    pub dark_range: (f64, f64),

    /// Luminance range for mid colors, as a fraction of 1.
    ///
    /// There are four luminance levels for the mid range: "MediumLower",
    /// "MediumLow", "MediumHigh", and "MediumHigher".
    pub mid_range: (f64, f64),

    /// Luminance range for bright colors, as a fraction of 1.
    ///
    /// There are three luminance levels for the bright range: "BrightLow",
    /// "BrightMedium", and "BrightHigh".
    pub bright_range: (f64, f64),

    /// Power to apply to luminance when calculating color levels.
    pub luminance_power: f64,

    /// Gamma to apply to luminance when calculating color levels.
    pub luminance_gamma: f64,

    /// Saturation range for muted colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub muted_sat_range: (f64, f64),

    /// Saturation range for base colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub base_sat_range: (f64, f64),

    /// Saturation range for intense colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub intense_sat_range: (f64, f64),
}

impl ThemeBuilder {
    pub fn into_theme(self) -> Theme {
        let lums: [f64; LUM_COUNT] = [
            self.dark_range.0 / 100.0,
            (self.dark_range.0 + self.dark_range.1) / 200.0,
            self.dark_range.1 / 100.0,
            self.mid_range.0 / 100.0,
            (self.mid_range.0 + self.mid_range.1) * (1.0 / 300.0),
            (self.mid_range.0 + self.mid_range.1) * (2.0 / 300.0),
            self.mid_range.1 / 100.0,
            self.bright_range.0 / 100.0,
            (self.bright_range.0 + self.bright_range.1) / 200.0,
            self.bright_range.1 / 100.0,
        ];

        let hues: [f64; TEMP_COUNT] = [
            self.offset + self.cool_range.0,
            self.offset + (self.cool_range.0 + self.cool_range.1) / 2.0,
            self.offset + self.cool_range.1,
            self.offset + self.warm_range.0,
            self.offset + (self.warm_range.0 + self.warm_range.1) / 2.0,
            self.offset + self.warm_range.1,
        ];

        let sat_ranges: [(f64, f64); SAT_COUNT] = [
            (self.muted_sat_range.0 / 100.0, self.muted_sat_range.1 / 100.0),
            (self.base_sat_range.0 / 100.0, self.base_sat_range.1 / 100.0),
            (self.intense_sat_range.0 / 100.0, self.intense_sat_range.1 / 100.0),
        ];

        let mut base_palette: BasePalette = Default::default();

        let lum_fn = |lum: f64| {
            let t = lum.clamp(0.0, 1.0).powf(self.luminance_gamma);
            let s = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };

            if t < 0.5 {
                s.powf(self.luminance_power) * 0.5
            } else {
                1.0 - s.powf(self.luminance_power) * 0.5
            }
        };

        for sat in 0..SAT_COUNT {
            for temp in 0..TEMP_COUNT {
                for lum in 0..LUM_COUNT {
                    let h = hues[temp].to_radians().rem_euclid(TAU);
                    let l = lum_fn(lums[lum]);
                    let s = sat_ranges[sat].0.lerp(sat_ranges[sat].1, l);

                    println!(
                        "{}-{}-{}: hsl({}°, {}%, {}%) -> #{:X}",
                        Into::<Sat>::into(sat),
                        Into::<Temp>::into(temp),
                        Into::<Lum>::into(lum),
                        (h.to_degrees()).round(),
                        (s * 100.0).round(),
                        (l * 100.0).round(),
                        okhsl(h, s, l).to_srgb()
                    );

                    base_palette[sat][temp][lum] = okhsl(h, s, l)
                }
            }
        }

        // Find the closest hue in the base palette for each primary color,
        // without reusing hues for multiple primaries.
        let mut used = 0;
        let mut primaries: PrimaryMap = Default::default();

        for primary in Primary::iter() {
            let (index, _) = hues
                .iter()
                .enumerate()
                .filter(|(i, _)| i & used == 0)
                .min_by(|(_, x), (_, y)| {
                    primary
                        .distance(**x)
                        .partial_cmp(&primary.distance(**y))
                        .unwrap_or(Ordering::Equal)
                })
                .unwrap();

            used |= 1 << index;
            primaries.insert(primary, index.into());
        }

        Theme {
            name: self.name,
            author: self.author,
            version: self.version,
            description: self.description,
            base_palette,
            primaries,
        }
    }

    pub fn build_theme(&self) -> Theme {
        self.clone().into_theme()
    }
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            author: None,
            version: Version::new(0, 1, 0),
            variants: vec![
                ThemeVariant::DAWN,
                ThemeVariant::DUSK,
                ThemeVariant::NOON,
                ThemeVariant::NIGHT,
            ],
            description: None,
            cool_range: (150.0, 270.0),
            warm_range: (-50.0, 70.0),
            offset: 0.0,
            dark_range: (0.1, 0.2),
            mid_range: (0.4, 0.6),
            bright_range: (0.9, 1.0),
            luminance_power: 1.2,
            luminance_gamma: 1.1,
            muted_sat_range: (0.1, 0.25),
            base_sat_range: (0.55, 0.65),
            intense_sat_range: (0.8, 0.9),
        }
    }
}
