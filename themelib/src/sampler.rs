use std::borrow::Cow;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    f64::consts::PI,
    io::{Result as IoResult, Write},
};
#[cfg(feature = "image")]
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Seek},
    path::Path,
};

#[cfg(feature = "image")]
use anyhow::Result as AnyResult;
use clap::ValueEnum;
#[cfg(not(target_arch = "wasm32"))]
use crossterm::{
    queue,
    style::{Color as TermColor, ContentStyle, Print, PrintStyledContent},
    terminal,
};
use glam::FloatExt;
#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView, ImageReader};
use itertools::Itertools;

#[cfg(not(target_arch = "wasm32"))]
use crate::color::{okhsl, SRgba};
use crate::{
    color::{OkHsl, SRgb},
    theme::HueBuilder,
};

#[derive(Clone, Debug)]
pub struct Histogram {
    data: [f64; 360],
}

#[derive(Clone, Debug)]
pub struct HueSampler<'a> {
    pixels: Cow<'a, [SRgb]>,
    sat_cutoff: f64,
    sat_weight_power: f64,
    smooth_lambda: f64,
    smooth_steps: usize,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum HueMode {
    /// Analogous hue configuration, where all six colors are spread around a
    /// primary hue.
    Analogous,

    /// Complementary hue configuration, where three colors are spread around a
    /// primary hue and the other three colors are spread around a hue
    /// complementary to the primary.
    Complementary,

    /// Triadic hue configuration, where a primary is selected and the other two
    /// hues are selected at 120 degrees from the primary. The remaining three
    /// hues are slightly offset from the triadic positions.
    Triadic,

    /// Custom hue configuration, where each of the six hues is specified
    /// individually.
    Custom,
}

impl<'a> HueSampler<'a> {
    pub fn from_iter(pixels: impl Iterator<Item = SRgb>) -> Self {
        Self {
            pixels: Cow::Owned(pixels.collect::<Vec<_>>()),
            smooth_lambda: 0.25,
            smooth_steps: 24,
            sat_cutoff: 0.1,
            sat_weight_power: 2.0,
        }
    }

    pub fn from_slice(pixels: &'a [SRgb]) -> Self {
        Self {
            pixels: Cow::Borrowed(pixels),
            smooth_lambda: 0.25,
            smooth_steps: 24,
            sat_cutoff: 0.1,
            sat_weight_power: 2.0,
        }
    }

    #[cfg(feature = "image")]
    pub fn from_image(image: &DynamicImage) -> Self {
        let downsampled = image.thumbnail(256, 256);
        Self::from_iter(downsampled.pixels().map(|(_x, _y, pixel)| SRgba::from(pixel).without_a()))
    }

    #[cfg(feature = "image")]
    pub fn from_reader<R: BufRead + Seek>(r: &mut R) -> AnyResult<Self> {
        let image = ImageReader::new(r).with_guessed_format()?.decode()?;
        Ok(Self::from_image(&image))
    }

    #[cfg(feature = "image")]
    pub fn from_file(path: impl AsRef<Path>) -> AnyResult<Self> {
        let r = File::open(path)?;
        let mut r = BufReader::new(r);
        Ok(Self::from_reader(&mut r)?)
    }

    pub fn with_sat_weight(mut self, power: f64, cutoff: f64) -> Self {
        self.sat_weight_power = power;
        self.sat_cutoff = cutoff;
        self
    }

    pub fn with_sat_weight_power(mut self, sat_weight_power: f64) -> Self {
        self.sat_weight_power = sat_weight_power;
        self
    }

    pub fn with_smoothing(mut self, lambda: f64, steps: usize) -> Self {
        self.smooth_lambda = lambda;
        self.smooth_steps = steps;
        self
    }

    pub fn histogram(&self) -> Histogram {
        let mut histogram = self.sample();
        histogram.smooth(self.smooth_lambda, self.smooth_steps);
        histogram.normalize();
        histogram
    }

    fn sample(&self) -> Histogram {
        let mut histogram = Histogram::default();

        for pixel in self.pixels.iter() {
            let OkHsl { h, s, .. } = pixel.to_hsl();

            let bin = h.to_degrees() as usize;
            let weight = (s - self.sat_cutoff).max(0.0) / (1.0 - self.sat_cutoff);
            let weight = weight.powf(self.sat_weight_power);

            histogram.data[bin % 360] += weight;
        }

        histogram
    }
}

impl Histogram {
    fn normalize(&mut self) {
        let max = self.data.iter().cloned().fold(0.0, f64::max);
        if max > 0.0 {
            for value in self.data.iter_mut() {
                *value /= max;
            }
        }
    }

    fn smooth(&mut self, λ: f64, steps: usize) {
        let mut buf0 = self.data;
        let mut buf1 = [0.0; 360];
        let (mut current, mut next) = (&mut buf0, &mut buf1);

        for _ in 0..steps {
            for i in 0..360 {
                let l = current[(i + 359) % 360];
                let r = current[(i + 1) % 360];

                next[i] = f64::lerp(current[i], (l + r) * 0.5, λ);
            }
            std::mem::swap(&mut current, &mut next);
        }

        self.data = *current;
    }

    pub fn create_builder(&self, hue_mode: HueMode, spread: Option<f64>) -> HueBuilder {
        let hue_count = match hue_mode {
            HueMode::Analogous => 1,
            HueMode::Complementary => 2,
            HueMode::Triadic => 1,
            HueMode::Custom => 6,
        };

        let peak_sep = match hue_mode {
            HueMode::Analogous => 0,
            HueMode::Complementary => 90,
            HueMode::Triadic => 0,
            HueMode::Custom => 30,
        };

        let hues = self.find_peaks(hue_count, peak_sep);

        match hue_mode {
            HueMode::Analogous => HueBuilder::Analogous {
                primary: hues[0].to_degrees(),
                spread: spread.unwrap_or(60.0),
            },
            HueMode::Complementary => {
                let offset = hues[1].to_degrees() - hues[0].to_degrees();

                let spread = spread.unwrap_or(45.0);

                if offset.abs() > spread.max(90.0) {
                    HueBuilder::Complementary {
                        primary: hues[0].to_degrees(),
                        offset,
                        spread,
                        reverse: false,
                    }
                } else {
                    HueBuilder::Complementary {
                        primary: hues[0].to_degrees(),
                        offset: 180.0,
                        spread,
                        reverse: false,
                    }
                }
            }
            HueMode::Triadic => HueBuilder::Triadic {
                primary: hues[0].to_degrees(),
                spread: spread.unwrap_or(30.0),
            },
            HueMode::Custom => {
                let mut hues = hues
                    .iter()
                    .map(|&h| h.to_degrees())
                    .sorted_by(|a, b| HueBuilder::temp(*a).partial_cmp(&HueBuilder::temp(*b)).unwrap());

                HueBuilder::Custom {
                    cold: hues.next().unwrap(),
                    cool: hues.next().unwrap(),
                    coolish: hues.next().unwrap(),
                    warmish: hues.next().unwrap(),
                    warm: hues.next().unwrap(),
                    hot: hues.next().unwrap(),
                    offset: 0.0,
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn print(&self) -> IoResult<()> {
        let (width, _) = terminal::size()?;
        let chunk_size = if width >= 122 {
            3
        } else if width >= 92 {
            4
        } else {
            5
        };

        const HEIGHT: usize = 24;
        queue!(
            stdout(),
            Print("┌"),
            Print("─".repeat(self.data.len() / chunk_size)),
            Print("┐\n")
        )?;

        for y in (0..HEIGHT).rev() {
            queue!(stdout(), Print("│"))?;
            for (i, chunk) in self.data.iter().chunks(chunk_size).into_iter().enumerate() {
                let value = chunk.sum::<f64>() / chunk_size as f64;
                let whole = (value * (8 * HEIGHT) as f64) as usize / 8;
                let remainder = (value * (8 * HEIGHT) as f64) as usize % 8;
                let fg = okhsl(
                    ((i * chunk_size) as f64 - (chunk_size as f64 / 2.0)).to_radians(),
                    1.0,
                    0.5,
                );
                let bg = okhsl(fg.h + PI, 0.15, 0.15);
                let [r, g, b] = fg.to_srgb().to_bytes();
                let [br, bg, bb] = bg.to_srgb().to_bytes();

                let style = ContentStyle {
                    foreground_color: Some(TermColor::Rgb { r, g, b }),
                    background_color: Some(TermColor::Rgb { r: br, g: bg, b: bb }),
                    ..Default::default()
                };

                if whole > y {
                    queue!(stdout(), PrintStyledContent(style.apply("█")))?;
                } else if whole == y && remainder > 0 {
                    let partial_block = match remainder {
                        1 => "▁",
                        2 => "▂",
                        3 => "▃",
                        4 => "▄",
                        5 => "▅",
                        6 => "▆",
                        7 => "▇",
                        _ => "█",
                    };
                    queue!(stdout(), PrintStyledContent(style.apply(partial_block)))?;
                } else {
                    queue!(stdout(), PrintStyledContent(style.apply(" ")))?;
                }
            }
            queue!(stdout(), Print("│\n"))?;
        }

        queue!(
            stdout(),
            Print("└"),
            Print("─".repeat(self.data.len() / chunk_size)),
            Print("┘\n")
        )?;

        stdout().flush()?;

        Ok(())
    }

    fn find_peaks(&self, count: usize, peak_sep: usize) -> Vec<f64> {
        if count == 0 {
            vec![]
        } else if count == 1 {
            vec![(self.find_peak() as f64).to_radians()]
        } else {
            let mut clone = self.clone();
            let mut peaks = Vec::with_capacity(count);

            for _ in 0..count {
                let peak = clone.find_peak();
                peaks.push((peak as f64).to_radians());
                clone.remove_peak(peak, peak_sep);
            }
            peaks
        }
    }

    fn find_peak(&self) -> usize {
        let mut peak = 0;
        let mut max_value = 0.0;
        for (i, &value) in self.data.iter().enumerate() {
            if value > max_value {
                max_value = value;
                peak = i;
            }
        }

        peak
    }

    fn remove_peak(&mut self, peak: usize, peak_sep: usize) {
        let Self { ref mut data, .. } = *self;

        for i in 0..360 {
            let dist = usize::min((i + 360 - peak) % 360, (peak + 360 - i) % 360);

            if dist <= peak_sep {
                data[i] *= f64::smoothstep(dist as f64 / peak_sep as f64, 0.0, 1.0);
            }
        }
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self { data: [0.0; 360] }
    }
}
