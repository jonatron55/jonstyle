// This is adapted from Björn Ottosson's work on perceptual color spaces:
// - https://bottosson.github.io/posts/oklab/
// - https://bottosson.github.io/posts/colorpicker/
// - https://bottosson.github.io/posts/gamutclipping/

// Copyright (c) 2021 Björn Ottosson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and /or sell
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
    f64::consts::{PI, TAU},
    fmt::{Display, Formatter, Result as FmtResult},
};

use glam::*;

use super::*;

/// A color in Björn Ottosson's [OkHsl perceptual color space](https://bottosson.github.io/posts/colorpicker/).
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OkHsl {
    /// Hue in radians
    pub h: f64,

    /// Saturation and lightness in the range [0, 1]
    pub s: f64,

    /// Lightness in the range [0, 1]
    pub l: f64,
}

/// A color in Björn Ottosson's [OkHsl perceptual color space](https://bottosson.github.io/posts/colorpicker/) with an
/// alpha channel.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OkHsla {
    /// Hue in radians
    pub h: f64,

    /// Saturation in the range [0, 1]
    pub s: f64,

    /// Lightness in the range [0, 1]
    pub l: f64,

    /// Alpha in the range [0, 1]
    pub a: f64,
}

pub const fn okhsl(h: f64, s: f64, l: f64) -> OkHsl {
    OkHsl { h, s, l }
}

pub const fn okhsla(h: f64, s: f64, l: f64, a: f64) -> OkHsla {
    OkHsla { h, s, l, a }
}

impl OkHsl {
    pub const fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }

    /// Convert to CIE XYZ.
    pub fn to_xyz(&self) -> CieXyz {
        self.clone().into()
    }

    /// Convert to OkLuv.
    pub fn to_luv(&self) -> OkLuv {
        self.clone().into()
    }

    /// Convert to linear RGB.
    pub fn to_rgb(&self) -> Rgb {
        self.clone().into()
    }

    /// Convert to gamma-corrected sRGB.
    pub fn to_srgb(&self) -> SRgb {
        self.clone().into()
    }

    /// Convert to a vector with x, y, z components corresponding to h, s, l.
    pub fn as_vec3(&self) -> DVec3 {
        self.clone().into()
    }

    pub fn with_a(&self, a: f64) -> OkHsla {
        OkHsla::new(self.h, self.s, self.l, a)
    }

    pub fn as_array(&self) -> [f64; 3] {
        [self.h, self.s, self.l]
    }

    pub fn from_slice(slice: &[f64; 3]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }
}

impl From<OkLuv> for OkHsl {
    fn from(luv: OkLuv) -> Self {
        let OkLuv { l, u, v } = luv;

        let c = (u.powi(2) + v.powi(2)).sqrt();

        // Achromatic colors carry no meaningful hue or saturation, and the
        // chroma reconstruction below degenerates as c -> 0 (and at l = 0 or
        // l = 1). Detect them up front so floating-point residue in u and v
        // cannot produce a spurious hue or saturation.
        if c < 1e-6 {
            return OkHsl {
                h: 0.0,
                s: 0.0,
                l: toe(l),
            };
        }

        let u_ = u / c;
        let v_ = v / c;

        let mut h = (-v).atan2(-u) + PI;

        let Cs { c_0, c_mid, c_max } = get_cs(l, u_, v_);

        let mut s = if c < c_mid {
            let k_1 = 0.8 * c_0;
            let k_2 = 1.0 - k_1 / c_mid;

            let t = c / (k_1 + k_2 * c);
            t * 0.8
        } else {
            let k_0 = c_mid;
            let k_1 = 0.2 * c_mid.powi(2) * 1.5625 / c_0;
            let k_2 = 1.0 - k_1 / (c_max - c_mid);

            let t = (c - k_0) / (k_1 + k_2 * (c - k_0));
            0.8 + 0.2 * t
        };

        // Monochromatic colours have no meaningful hue saturation, but NaN is
        // inconvenient to work with, so we set it to zero.
        if !h.is_finite() {
            h = 0.0;
        }

        if !s.is_finite() {
            s = 0.0;
        }

        let l = toe(l);
        OkHsl { h, s, l }
    }
}

impl Into<OkLuv> for OkHsl {
    fn into(self) -> OkLuv {
        let OkHsl { h, s, l } = self;

        if l == 1.0 {
            return OkLuv::new(1.0, 0.0, 0.0);
        } else if l == 0.0 {
            return OkLuv::new(0.0, 0.0, 0.0);
        }

        let u_ = h.cos();
        let v_ = h.sin();
        let l = inv_toe(l);

        let Cs { c_0, c_mid, c_max } = get_cs(l, u_, v_);

        let c = if s < 0.8 {
            let t = 1.25 * s;
            let k_1 = 0.8 * c_0;
            let k_2 = 1.0 - k_1 / c_mid;

            t * k_1 / (1.0 - k_2 * t)
        } else {
            let t = 5.0 * (s - 0.8);
            let k_0 = c_mid;
            let k_1 = 0.2 * c_mid.powi(2) * 1.5625 / c_0;
            let k_2 = 1.0 - k_1 / (c_max - c_mid);

            k_0 + t * k_1 / (1.0 - k_2 * t)
        };

        OkLuv::new(l, c * u_, c * v_)
    }
}

impl From<DVec3> for OkHsl {
    fn from(v: DVec3) -> Self {
        OkHsl::new(v.x, v.y, v.z)
    }
}

impl Into<DVec3> for OkHsl {
    fn into(self) -> DVec3 {
        dvec3(self.h, self.s, self.l)
    }
}

impl From<&[f64; 3]> for OkHsl {
    fn from(slice: &[f64; 3]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 3]> for OkHsl {
    fn into(self) -> [f64; 3] {
        self.as_array()
    }
}

impl OkHsla {
    pub const fn new(h: f64, s: f64, l: f64, a: f64) -> Self {
        Self { h, s, l, a }
    }

    /// Convert to CIE XYZA
    pub fn to_xyza(&self) -> CieXyza {
        self.clone().into()
    }

    /// Convert to OkLuva.
    pub fn to_luva(&self) -> OkLuva {
        self.clone().into()
    }

    /// Convert to linear RGB.
    pub fn to_rgba(&self) -> Rgba {
        self.clone().into()
    }

    /// Convert to gamma-corrected sRGBA.
    pub fn to_srgba(&self) -> SRgba {
        self.clone().into()
    }

    /// Convert to a vector with x, y, z components corresponding to h, s, l.
    pub fn as_vec4(&self) -> DVec4 {
        self.clone().into()
    }

    pub fn without_a(&self) -> OkHsl {
        OkHsl::new(self.h, self.s, self.l)
    }

    pub fn as_array(&self) -> [f64; 4] {
        [self.h, self.s, self.l, self.a]
    }

    pub fn from_slice(slice: &[f64; 4]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<OkLuva> for OkHsla {
    fn from(luva: OkLuva) -> Self {
        let luv: OkLuv = luva.without_a();
        let hsl: OkHsl = luv.into();
        hsl.with_a(luva.a)
    }
}

impl Into<OkLuva> for OkHsla {
    fn into(self) -> OkLuva {
        let hsl: OkHsl = self.without_a();
        let luv: OkLuv = hsl.into();
        luv.with_a(self.a)
    }
}

impl From<DVec4> for OkHsla {
    fn from(v: DVec4) -> Self {
        OkHsla::new(v.x, v.y, v.z, v.w)
    }
}

impl Into<DVec4> for OkHsla {
    fn into(self) -> DVec4 {
        dvec4(self.h, self.s, self.l, self.a)
    }
}

impl From<&[f64; 4]> for OkHsla {
    fn from(slice: &[f64; 4]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 4]> for OkHsla {
    fn into(self) -> [f64; 4] {
        self.as_array()
    }
}

impl Display for OkHsl {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { h, s, l } = self;
        let h = h.to_degrees();
        if let Some(precis) = f.precision() {
            write!(f, "({h:.0}°, {s:.precis$}, {l:.precis$})")
        } else {
            write!(f, "({h:.0}°, {s:.2}, {l:.2})")
        }
    }
}

impl Display for OkHsla {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { h, s, l, a } = self;
        let h = h.to_degrees();
        if let Some(precis) = f.precision() {
            write!(f, "({h:.0}°, {s:.precis$}, {l:.precis$}, {a:.precis$})")
        } else {
            write!(f, "({h:.0}°, {s:.2}, {l:.2}, {a:.2})")
        }
    }
}

/// The number of Halley's-method refinement iterations used when solving for
/// the maximum saturation and the gamut boundary. One iteration matches the
/// reference implementation; additional iterations tighten the result for the
/// numerically sensitive blue hues.
const HALLEY_ITERATIONS: usize = 2;

/// Finds the maximum saturation possible for a given hue that fits in sRGB
/// Saturation here is defined as *S* = *C*/*L*
///
/// *u* and *v* must be normalized so *u*² + *v*² = 1
fn compute_max_sat(u: f64, v: f64) -> f64 {
    // Max saturation will be when one of r, g or b goes below zero.

    // Select different coefficients depending on which component goes below zero first
    let (k0, k1, k2, k3, k4, wl, wm, ws) = if -1.88170328 * u - 0.80936493 * v > 1.0 {
        // Red component
        (
            1.19086277,
            1.76576728,
            0.59662641,
            0.75515197,
            0.56771245,
            4.0767416621,
            -3.3077115913,
            0.2309699292,
        )
    } else if 1.81444104 * u - 1.19445276 * v > 1.0 {
        // Green component
        (
            0.73956515,
            -0.45954404,
            0.08285427,
            0.12541070,
            0.14503204,
            -1.2684380046,
            2.6097574011,
            -0.3413193965,
        )
    } else {
        // Blue component
        (
            1.35733652,
            -0.00915799,
            -1.15130210,
            -0.50559606,
            0.00692167,
            -0.0041960863,
            -0.7034186147,
            1.7076147010,
        )
    };

    // Approximate max saturation using a polynomial:
    let mut sat = k0 + k1 * u + k2 * v + k3 * u * u + k4 * u * v;

    // Refine with Halley's method. A single iteration matches the reference and
    // gives an error below 1e-6 for most hues; the blue hues (where dS/dh is
    // nearly infinite) benefit from the extra iterations.
    let k_l = 0.3963377774 * u + 0.2158037573 * v;
    let k_m = -0.1055613458 * u - 0.0638541728 * v;
    let k_s = -0.0894841775 * u - 1.2914855480 * v;

    for _ in 0..HALLEY_ITERATIONS {
        let l_ = 1.0 + sat * k_l;
        let m_ = 1.0 + sat * k_m;
        let s_ = 1.0 + sat * k_s;

        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);

        let l_ds = 3.0 * k_l * l_ * l_;
        let m_ds = 3.0 * k_m * m_ * m_;
        let s_ds = 3.0 * k_s * s_ * s_;

        let l_ds2 = 6.0 * k_l * k_l * l_;
        let m_ds2 = 6.0 * k_m * k_m * m_;
        let s_ds2 = 6.0 * k_s * k_s * s_;

        let f = wl * l + wm * m + ws * s;
        let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
        let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

        sat -= f * f1 / (f1 * f1 - 0.5 * f * f2);
    }

    sat
}

/// finds *l_cusp* and *c_cusp* for a given hue
///
/// *u* and *v* must be normalized so *u*² + *v*² = 1
fn find_cusp(u: f64, v: f64) -> (f64, f64) {
    // First, find the maximum saturation (saturation S = C/L)
    let s_cusp = compute_max_sat(u, v);

    // Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
    let Rgb { r, g, b } = OkLuv::new(1.0, s_cusp * u, s_cusp * v).to_rgb();
    let l_cusp = r.max(g).max(b).recip().cbrt();
    let c_cusp = l_cusp * s_cusp;

    (l_cusp, c_cusp)
}

/// Finds intersection of the line defined by
///
/// *L* = *L*₀(1 - *t*) + *L*₁*t*;
///
/// *C* = *C*₁*t*;
///
/// *u* and *v* must be normalized so *u*² + *v*² = 1
fn find_gamut_intersect(a: f64, b: f64, l1: f64, c1: f64, l0: f64, l_cusp: f64, c_cusp: f64) -> f64 {
    // Find the intersection for upper and lower half seprately
    if ((l1 - l0) * c_cusp - (l_cusp - l0) * c1) <= 0.0 {
        // Lower half
        c_cusp * l0 / (c1 * l_cusp + c_cusp * (l0 - l1))
    } else {
        // Upper half

        // First intersect with triangle
        let mut t = c_cusp * (l0 - 1.0) / (c1 * (l_cusp - 1.0) + c_cusp * (l0 - l1));

        // Then refine with Halley's method
        let dl = l1 - l0;
        let dc = c1;

        let k_l = 0.3963377774 * a + 0.2158037573 * b;
        let k_m = -0.1055613458 * a - 0.0638541728 * b;
        let k_s = -0.0894841775 * a - 1.2914855480 * b;

        let l_dt = dl + dc * k_l;
        let m_dt = dl + dc * k_m;
        let s_dt = dl + dc * k_s;

        for _ in 0..HALLEY_ITERATIONS {
            let l = l0 + (l1 - l0) * t;
            let c = t * c1;

            let l_ = l + c * k_l;
            let m_ = l + c * k_m;
            let s_ = l + c * k_s;

            let l = l_.powi(3);
            let m = m_.powi(3);
            let s = s_.powi(3);

            let ldt = 3.0 * l_dt * l_ * l_;
            let mdt = 3.0 * m_dt * m_ * m_;
            let sdt = 3.0 * s_dt * s_ * s_;

            let ldt2 = 6.0 * l_dt * l_dt * l_;
            let mdt2 = 6.0 * m_dt * m_dt * m_;
            let sdt2 = 6.0 * s_dt * s_dt * s_;

            let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.0;
            let r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
            let r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

            let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
            let t_r = -r * u_r;

            let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.0;
            let g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
            let g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

            let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
            let t_g = -g * u_g;

            let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.0;
            let b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
            let b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

            let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
            let t_b = -b * u_b;

            let t_r = if u_r >= 0.0 { t_r } else { f64::MAX };
            let t_g = if u_g >= 0.0 { t_g } else { f64::MAX };
            let t_b = if u_b >= 0.0 { t_b } else { f64::MAX };

            t += t_r.min(t_g).min(t_b);
        }

        t
    }
}

/// toe function for L_r
fn toe(x: f64) -> f64 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    0.5 * (k_3 * x - k_1 + ((k_3 * x - k_1) * (k_3 * x - k_1) + 4.0 * k_2 * k_3 * x).sqrt())
}

/// inverse toe function for L_r
fn inv_toe(x: f64) -> f64 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    (x * x + k_1 * x) / (k_3 * (x + k_2))
}

fn to_st(l: f64, c: f64) -> (f64, f64) {
    (c / l, c / (1.0 - l))
}

/// Returns a smooth approximation of the location of the cusp. This polynomial
/// was created by an optimization process. It has been designed so that
/// *S_mid* < *S_max* and *T_mid* < *T_max*.
fn get_st_mid(u_: f64, v_: f64) -> (f64, f64) {
    let s = 0.11516993
        + (7.44778970
            + 4.15901240 * v_
            + u_ * (-2.19557347
                + 1.75198401 * v_
                + u_ * (-2.13704948 - 10.02301043 * v_ + u_ * (-4.24894561 + 5.38770819 * v_ + 4.69891013 * u_))))
            .recip();

    let t = 0.11239642
        + (1.61320320 - 0.68124379 * v_
            + u_ * (0.40370612
                + 0.90148123 * v_
                + u_ * (-0.27087943 + 0.61223990 * v_ + u_ * (0.00299215 - 0.45399568 * v_ - 0.14661872 * u_))))
            .recip();

    (s, t)
}

struct Cs {
    c_0: f64,
    c_mid: f64,
    c_max: f64,
}

fn get_cs(l: f64, u_: f64, v_: f64) -> Cs {
    let (l_cusp, c_cusp) = find_cusp(u_, v_);

    let c_max = find_gamut_intersect(u_, v_, l, 1.0, l, l_cusp, c_cusp);
    let (s_max, t_max) = to_st(l_cusp, c_cusp);

    // Scale factor to compensate for the curved part of gamut shape:
    let k = c_max / (l * s_max).min((1.0 - l) * t_max);

    let (s_mid, t_mid) = get_st_mid(u_, v_);

    // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
    let c_u = l * s_mid;
    let c_v = (1.0 - l) * t_mid;
    let c_mid = 0.9 * k * ((c_u.powi(4).recip() + c_v.powi(4).recip()).recip().sqrt()).sqrt();

    // for C_0, the shape is independent of hue, so ST are constant. Values picked to roughly be the average values of ST.
    let c_u = l * 0.4;
    let c_v = (1.0 - l) * 0.8;

    // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
    let c_0 = (c_u.powi(2).recip() + c_v.powi(2).recip()).recip().sqrt();

    Cs { c_0, c_mid, c_max }
}
