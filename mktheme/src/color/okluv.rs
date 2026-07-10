// This is adapted from Björn Ottosson's work on perceptual color spaces:
// - https://bottosson.github.io/posts/oklab/
// - https://bottosson.github.io/posts/colorpicker/
// - https://bottosson.github.io/posts/gamutclipping/
//
// Ottosson's model names its channels "l", "a", and "b", but they are specified
// here as "l", "u", "v" to allow an alpha channel named "a". The model is
// otherwise identical.

// Copyright (c) 2021 Björn Ottosson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{self, Error as IoError},
};

use glam::*;

use super::*;

/// A color in the Björn Ottosson's OkLuv perceptual color space. Ottosson originally named
/// ["OkLab"](https://bottosson.github.io/posts/oklab/), but in this implementation, the channels are named "l", u", and
/// "v" to allow an alpha channel named "a".
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OkLuv {
    pub l: f64,
    pub u: f64,
    pub v: f64,
}

/// A color in the Björn Ottosson's OkLuv perceptual color space with an alpha channel. Ottosson originally named
/// ["OkLab"](https://bottosson.github.io/posts/oklab/), but in this implementation, the channels are named "l", "u",
/// and "v" to allow an alpha channel named "a".
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OkLuva {
    pub l: f64,
    pub u: f64,
    pub v: f64,
    pub a: f64,
}

pub const fn okluv(l: f64, u: f64, v: f64) -> OkLuv {
    OkLuv { l, u, v }
}

pub const fn okluva(l: f64, u: f64, v: f64, a: f64) -> OkLuva {
    OkLuva { l, u, v, a }
}

impl OkLuv {
    pub const fn new(l: f64, u: f64, v: f64) -> Self {
        Self { l, u, v }
    }

    /// Convert the color to CIE XYZ.
    pub fn to_xyz(&self) -> CieXyz {
        self.clone().into()
    }

    /// Convert the color to linear RGB.
    pub fn to_rgb(&self) -> Rgb {
        self.clone().into()
    }

    /// Convert the color to gamma-corrected RGB.
    pub fn to_srgb(&self) -> SRgb {
        self.clone().into()
    }

    /// Convert the color to OkHSL.
    pub fn to_hsl(&self) -> OkHsl {
        self.clone().into()
    }

    /// Convert the color to a vector.
    pub fn as_vec3(&self) -> DVec3 {
        self.clone().into()
    }

    pub fn with_a(&self, a: f64) -> OkLuva {
        OkLuva::new(self.l, self.u, self.v, a)
    }

    pub fn as_array(&self) -> [f64; 3] {
        [self.l, self.u, self.v]
    }

    pub fn from_slice(slice: &[f64; 3]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }
}

const FROM_RGB1: DMat3 = dmat3(
    dvec3(0.4122214708, 0.2119034982, 0.0883024619),
    dvec3(0.5363325363, 0.6806995451, 0.2817188376),
    dvec3(0.0514459929, 0.1073969566, 0.6299787005),
);

const FROM_RGB2: DMat3 = dmat3(
    dvec3(0.2104542553, 1.9779984951, 0.0259040371),
    dvec3(0.7936177850, -2.4285922050, 0.7827717662),
    dvec3(-0.0040720468, 0.4505937099, -0.8086757660),
);

const TO_RGB1: DMat3 = dmat3(
    dvec3(4.0767416621, -1.2684380046, -0.0041960863),
    dvec3(-3.3077115913, 2.6097574011, -0.7034186147),
    dvec3(0.2309699292, -0.3413193965, 1.7076147010),
);

const TO_RGB2: DMat3 = dmat3(
    dvec3(1.0000000000, 1.0000000000, 1.0000000000),
    dvec3(0.3963377774, -0.1055613458, -0.0894841775),
    dvec3(0.2158037573, -0.0638541728, -1.2914855480),
);

const FROM_XYZ1: DMat3 = dmat3(
    dvec3(0.8189330101, 0.0329845436, 0.0482003018),
    dvec3(0.3618667424, 0.9293118715, 0.2643662691),
    dvec3(-0.1288597137, 0.0361456387, 0.6338517070),
);

const FROM_XYZ2: DMat3 = dmat3(
    dvec3(0.2104542553, 0.7936177850, -0.0040720468),
    dvec3(1.9779984951, -2.4285922050, 0.4505937099),
    dvec3(0.0259040371, 0.7827717662, -0.8086757660),
);

const TO_XYZ1: DMat3 = dmat3(
    dvec3(1.2270138511, -0.0405801784, -0.0763812845),
    dvec3(-0.5577999807, 1.1122568696, -0.4214819784),
    dvec3(0.2812561490, -0.0716766787, 1.5861632204),
);

const TO_XYZ2: DMat3 = dmat3(
    dvec3(1.0000000000, 0.3963382625, 0.2158037573),
    dvec3(1.0000000000, -0.1055613458, -0.0638541728),
    dvec3(1.0000000000, -0.0894841775, -1.2914855480),
);

/// Component-wise cube root. Unlike `DVec3::powf(1.0 / 3.0)`, this is defined
/// for negative components (matching the reference `Math.cbrt`), which can
/// occur for out-of-gamut intermediate LMS values and would otherwise produce
/// `NaN`.
fn cbrt(v: DVec3) -> DVec3 {
    dvec3(v.x.cbrt(), v.y.cbrt(), v.z.cbrt())
}

impl From<Rgb> for OkLuv {
    fn from(rgb: Rgb) -> Self {
        let lms = FROM_RGB1 * rgb.as_vec3();
        let lms = cbrt(lms);
        return (FROM_RGB2 * lms).into();
    }
}

impl Into<Rgb> for OkLuv {
    fn into(self) -> Rgb {
        let lms = TO_RGB2 * self.as_vec3();
        let lms = lms * lms * lms;
        return (TO_RGB1 * lms).into();
    }
}

impl From<CieXyz> for OkLuv {
    fn from(xyz: CieXyz) -> Self {
        let lms = FROM_XYZ1 * xyz.as_vec3();
        let lms = cbrt(lms);
        return (FROM_XYZ2 * lms).into();
    }
}

impl Into<CieXyz> for OkLuv {
    fn into(self) -> CieXyz {
        let lms = TO_XYZ2 * self.as_vec3();
        let lms = lms * lms * lms;
        return (TO_XYZ1 * lms).into();
    }
}

impl From<DVec3> for OkLuv {
    fn from(v: DVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Into<DVec3> for OkLuv {
    fn into(self) -> DVec3 {
        dvec3(self.l, self.u, self.v)
    }
}

impl From<&[f64; 3]> for OkLuv {
    fn from(slice: &[f64; 3]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 3]> for OkLuv {
    fn into(self) -> [f64; 3] {
        self.as_array()
    }
}

impl OkLuva {
    pub const fn new(l: f64, u: f64, v: f64, a: f64) -> Self {
        Self { l, u, v, a }
    }

    /// Convert to CIE XYZA.
    pub fn to_xyza(&self) -> CieXyza {
        self.clone().into()
    }

    /// Convert to linear RGBA.
    pub fn to_rgba(&self) -> Rgba {
        self.clone().into()
    }

    /// Convert the color to gamma-corrected RGBA.
    pub fn to_srgba(&self) -> SRgba {
        self.clone().into()
    }

    /// Convert to HSLA.
    pub fn to_hsla(&self) -> OkHsla {
        self.clone().into()
    }

    /// Convert to a vector.
    pub fn as_vec4(&self) -> DVec4 {
        self.clone().into()
    }

    pub fn without_a(&self) -> OkLuv {
        OkLuv::new(self.l, self.u, self.v)
    }

    pub fn as_array(&self) -> [f64; 4] {
        [self.l, self.u, self.v, self.a]
    }

    pub fn from_slice(slice: &[f64; 4]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<Rgba> for OkLuva {
    fn from(rgba: Rgba) -> Self {
        let rgb = rgba.without_a();
        OkLuv::from(rgb).with_a(rgba.a)
    }
}

impl Into<Rgba> for OkLuva {
    fn into(self) -> Rgba {
        let rgb: Rgb = self.without_a().into();
        rgb.with_a(self.a)
    }
}

impl From<CieXyza> for OkLuva {
    fn from(xyza: CieXyza) -> Self {
        let xyz = xyza.without_a();
        OkLuv::from(xyz).with_a(xyza.a)
    }
}

impl Into<CieXyza> for OkLuva {
    fn into(self) -> CieXyza {
        let xyz: CieXyz = OkLuv::from(self.without_a()).into();
        xyz.with_a(self.a)
    }
}

impl From<DVec4> for OkLuva {
    fn from(v: DVec4) -> Self {
        OkLuva::new(v.x, v.y, v.z, v.w)
    }
}

impl Into<DVec4> for OkLuva {
    fn into(self) -> DVec4 {
        dvec4(self.l, self.u, self.v, self.a)
    }
}

impl From<&[f64; 4]> for OkLuva {
    fn from(slice: &[f64; 4]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 4]> for OkLuva {
    fn into(self) -> [f64; 4] {
        self.as_array()
    }
}

impl Display for OkLuv {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { l, u, v } = self;
        write!(f, "({l:.3}, {u:.3}, {v:.3})")
    }
}

impl Display for OkLuva {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { l, u, v, a } = self;
        write!(f, "({l:.3}, {u:.3}, {v:.3}, {a:.3})")
    }
}
