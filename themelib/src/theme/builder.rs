use std::{cmp::Ordering, f64::consts::TAU};

use glam::FloatExt;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{
    color::okhsl,
    theme::{BasePalette, Primary, PrimaryMap, Theme, ThemeVariant, LUM_COUNT, SAT_COUNT, TEMP_COUNT},
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

    /// Luminance range for the theme, as a fraction of 1.
    pub lum_range: (f64, f64),

    /// Power to apply to luminance when calculating color levels.
    pub lum_alpha: f64,

    /// Gamma to apply to luminance when calculating color levels.
    pub lum_gamma: f64,

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
        let lums: Vec<f64> = (0..LUM_COUNT)
            .map(|i| f64::lerp(self.lum_range.0, self.lum_range.1, i as f64 / (LUM_COUNT - 1) as f64) / 100.0)
            .collect::<Vec<_>>();

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

        for sat in 0..SAT_COUNT {
            for temp in 0..TEMP_COUNT {
                for lum in 0..LUM_COUNT {
                    let h = hues[temp].to_radians().rem_euclid(TAU);
                    let l = self.lum_fn(lums[lum]);
                    let s = sat_ranges[sat].0.lerp(sat_ranges[sat].1, l);

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
                .filter(|(i, _)| (1 << i) & used == 0)
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
            variants: self.variants,
        }
    }

    pub fn build_theme(&self) -> Theme {
        self.clone().into_theme()
    }

    pub fn lum_fn(&self, lum: f64) -> f64 {
        lum_fn(lum, self.lum_range, self.lum_alpha, self.lum_gamma)
    }
}

pub fn lum_fn(lum: f64, (low, high): (f64, f64), power: f64, gamma: f64) -> f64 {
    let t = lum.clamp(0.0, 1.0).powf(gamma);
    let s = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };

    let lum = if t < 0.5 {
        s.powf(power) * 0.5
    } else {
        1.0 - s.powf(power) * 0.5
    };

    f64::lerp(low / 100.0, high / 100.0, lum)
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
            cool_range: (270.0, 180.0),
            warm_range: (0.0, 90.0),
            offset: 0.0,
            lum_range: (5.0, 95.0),
            lum_alpha: 1.0,
            lum_gamma: 1.0,
            muted_sat_range: (0.0, 20.0),
            base_sat_range: (40.0, 60.0),
            intense_sat_range: (80.0, 100.0),
        }
    }
}
