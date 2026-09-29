use std::{array, cmp::Ordering, f64::consts::TAU};

use glam::FloatExt;
use serde::{Deserialize, Serialize};

use crate::{
    color::okhsl,
    theme::{BasePalette, CodeStyle, Metadata, Primary, PrimaryMap, Sat, Theme, LUM_COUNT, SAT_COUNT, TEMP_COUNT},
};

/// A collection of parameters for generating a theme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeBuilder {
    /// Metadata for the theme.
    pub meta: Metadata,

    /// The hue configuration for the theme.
    pub hue: HueBuilder,

    /// The luminance configuration for the theme.
    pub lum: LumBuilder,

    /// The saturation configuration for the theme.
    pub sat: SatBuilder,

    /// The code style configuration for the theme.
    pub code: CodeStyle,
}

/// Different hue configuration options for the theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HueBuilder {
    /// Analogous hue configuration, where all six colors are spread around a
    /// primary hue.
    Analogous {
        /// Primary hue, in degrees, around which others are spread.
        primary: f64,

        /// Angle in degrees of the total spread around the primary hue.
        spread: f64,
    },

    /// Complementary hue configuration, where three colors are spread around a
    /// primary hue and the other three colors are spread around a hue
    /// complementary to the primary.
    Complementary {
        /// Primary hue, in degrees, around which three colors are spread.
        primary: f64,

        /// Angle in degrees of the total spread around the primary hue and its
        /// complement.
        spread: f64,

        /// Angle in degrees to offset the complementary hue from the primary
        /// hue.
        offset: f64,

        /// If set, the complementary hues will spread in the opposite direction
        /// to the primary.
        reverse: bool,
    },

    /// Triadic hue configuration, where a primary is selected and the other two
    /// hues are selected at 120 degrees from the primary. The remaining three
    /// hues are slightly offset from the triadic positions.
    Triadic {
        /// Primary hue, in degrees, around which the triadic configuration is
        /// based.
        primary: f64,

        /// Angle in degrees at which each secondary hue is offset from the
        /// triadic hue.
        spread: f64,
    },

    /// Custom hue configuration, where each of the six hues is specified
    /// individually.
    ///
    /// The temperature names are used in palette semantics, but are not strict
    /// indicators of actual color temperature in this mode: any hue can be set
    /// for a particular temperature name.
    Custom {
        /// Cold hue, in degrees.
        cold: f64,

        /// Cool hue, in degrees.
        cool: f64,

        /// Medium-cool hue, in degrees.
        coolish: f64,

        /// Medium-warm hue, in degrees.
        warmish: f64,

        /// Warm hue, in degrees.
        warm: f64,

        /// Hot hue, in degrees.
        hot: f64,

        /// Amount to offset all hues by in degrees.
        offset: f64,
    },
}

/// Luminance configuration options for the theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LumBuilder {
    /// Luminance range for the theme, as a fraction of 1.
    pub range: (f64, f64),

    /// Power to apply to luminance when calculating color levels.
    pub alpha: f64,

    /// Gamma to apply to luminance when calculating color levels.
    pub gamma: f64,
}

/// Saturation configuration options for the theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SatBuilder {
    /// Saturation range for muted colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub muted_range: (f64, f64),

    /// Saturation range for base colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub base_range: (f64, f64),

    /// Saturation range for intense colors as a fraction of 1.
    ///
    /// The lower bound of this range is used for the darkest colors, and the
    /// upper bound is used for the lightest colors.
    pub intense_range: (f64, f64),
}

impl ThemeBuilder {
    pub fn into_theme(self) -> Theme {
        let lums = self.lum.into_lums();

        let hues = self.hue.build_hues();

        let mut base_palette: BasePalette = Default::default();

        for sat in 0..SAT_COUNT {
            for temp in 0..TEMP_COUNT {
                for lum in 0..LUM_COUNT {
                    let h = hues[temp].to_radians().rem_euclid(TAU);
                    let l = self.lum_fn(lums[lum]);
                    let s = self.sat.get(sat.into(), l);

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
            meta: self.meta.clone(),
            base_palette,
            primaries,
        }
    }

    pub fn build_theme(&self) -> Theme {
        self.clone().into_theme()
    }

    pub fn lum_fn(&self, lum: f64) -> f64 {
        lum_fn(lum, self.lum.range, self.lum.alpha, self.lum.gamma)
    }
}

pub fn lum_fn(lum: f64, (low, high): (f64, f64), alpha: f64, gamma: f64) -> f64 {
    let t = lum.clamp(0.0, 1.0).powf(gamma);
    let s = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };

    let lum = if t < 0.5 {
        s.powf(alpha) * 0.5
    } else {
        1.0 - s.powf(alpha) * 0.5
    };

    f64::lerp(low / 100.0, high / 100.0, lum)
}

impl LumBuilder {
    pub fn into_lums(&self) -> [f64; LUM_COUNT] {
        array::from_fn(|i| f64::lerp(self.range.0, self.range.1, i as f64 / (LUM_COUNT - 1) as f64) / 100.0)
    }
}

impl HueBuilder {
    pub fn default_analogous() -> Self {
        Self::Analogous {
            primary: 0.0,
            spread: 45.0,
        }
    }

    pub fn default_complementary() -> Self {
        Self::Complementary {
            primary: 0.0,
            spread: 30.0,
            offset: 0.0,
            reverse: true,
        }
    }

    pub fn default_triadic() -> Self {
        Self::Triadic {
            primary: 0.0,
            spread: 20.0,
        }
    }

    pub fn default_custom() -> Self {
        Self::Custom {
            cold: 120.0,
            cool: 180.0,
            coolish: 2400.0,
            warmish: 120.0,
            warm: 300.0,
            hot: 60.0,
            offset: 0.0,
        }
    }

    pub fn into_hues(self) -> [f64; TEMP_COUNT] {
        self.build_hues()
    }

    pub fn build_hues(&self) -> [f64; TEMP_COUNT] {
        match self {
            Self::Analogous { primary, spread } => {
                let increment = spread / (TEMP_COUNT / 2) as f64;
                let color1 = (primary - 2.0 * increment) % 360.0;
                let color2 = (primary - increment) % 360.0;
                let color3 = (primary + increment) % 360.0;
                let color4 = (primary + 2.0 * increment) % 360.0;
                let color5 = (primary + 3.0 * increment) % 360.0;

                let mut hues = [color1, color2, *primary, color3, color4, color5];
                if Self::temp(color1) < Self::temp(color5) {
                    hues.reverse();
                }

                hues
            }

            Self::Complementary {
                primary,
                spread,
                offset,
                reverse,
            } => {
                let secondary = (primary + 180.0 + offset) % 360.0;

                let (primary, secondary) = if Self::temp(*primary) > Self::temp(secondary) {
                    (secondary, *primary)
                } else {
                    (*primary, secondary)
                };

                let increment = spread / (TEMP_COUNT / 2) as f64;

                let cold = primary;
                let cool = cold + increment;
                let coolish = cool + increment;

                let hot = secondary;
                let warm = hot + increment;
                let warmish = warm + increment;

                let (hot, warmish) = if *reverse { (warmish, hot) } else { (hot, warmish) };

                [cold, cool, coolish, warmish, warm, hot]
            }

            Self::Triadic { primary, spread } => {
                let secondary = (primary + 120.0) % 360.0;
                let tertiary = (primary + 240.0) % 360.0;

                let mut triad = [*primary, secondary, tertiary];
                triad.sort_by(|a, b| Self::temp(*a).partial_cmp(&Self::temp(*b)).unwrap());

                [
                    triad[0],
                    triad[0] + spread,
                    triad[1],
                    triad[1] + spread,
                    triad[2],
                    triad[2] + spread,
                ]
            }

            Self::Custom {
                cold,
                cool,
                coolish,
                warmish,
                warm,
                hot,
                offset,
            } => [
                *cold + *offset,
                *cool + *offset,
                *coolish + *offset,
                *warmish + *offset,
                *warm + *offset,
                *hot + *offset,
            ],
        }
    }

    pub fn primary_index(&self) -> usize {
        match self {
            Self::Analogous { primary, .. } | Self::Complementary { primary, .. } | Self::Triadic { primary, .. } => {
                self.build_hues().iter().position(|&h| h == *primary).unwrap_or(0)
            }
            _ => 0,
        }
    }

    fn temp(hue: f64) -> f64 {
        let hue = hue % 360.0;
        if hue > 180.0 {
            (hue - 180.0) / 90.0 - 1.0
        } else {
            -hue / 90.0 + 1.0
        }
    }
}

impl SatBuilder {
    pub fn get(&self, sat: Sat, lum: f64) -> f64 {
        match sat {
            Sat::Muted => f64::lerp(self.muted_range.0, self.muted_range.1, lum) / 100.0,
            Sat::Base => f64::lerp(self.base_range.0, self.base_range.1, lum) / 100.0,
            Sat::Intense => f64::lerp(self.intense_range.0, self.intense_range.1, lum) / 100.0,
        }
    }
}

impl Default for HueBuilder {
    fn default() -> Self {
        Self::default_complementary()
    }
}

impl Default for LumBuilder {
    fn default() -> Self {
        Self {
            range: (5.0, 95.0),
            alpha: 1.0,
            gamma: 1.0,
        }
    }
}

impl Default for SatBuilder {
    fn default() -> Self {
        Self {
            muted_range: (0.0, 20.0),
            base_range: (40.0, 60.0),
            intense_range: (80.0, 100.0),
        }
    }
}
