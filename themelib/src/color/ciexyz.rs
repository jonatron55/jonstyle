use std::io::{self, Error as IoError};

use glam::*;

use super::*;

const TO_RGB: DMat3 = dmat3(
    dvec3(3.240969942, -1.537383178, -0.498610760),
    dvec3(-0.969243636, 1.875967502, 0.041555057),
    dvec3(0.055630080, -0.203976959, 1.056971514),
);

const FROM_RGB: DMat3 = dmat3(
    dvec3(0.412390799, 0.357584339, 0.180480788),
    dvec3(0.212639006, 0.715168679, 0.072192315),
    dvec3(0.019330819, 0.119194780, 0.950532152),
);

/// A color in the CIE XYZ color space.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct CieXyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A color in the CIE XYZ color space.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct CieXyza {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub a: f64,
}

pub const fn ciexyz(x: f64, y: f64, z: f64) -> CieXyz {
    CieXyz { x, y, z }
}

pub const fn ciexyza(x: f64, y: f64, z: f64, a: f64) -> CieXyza {
    CieXyza { x, y, z, a }
}

impl CieXyz {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Convert the color to linear RGB.
    pub fn to_rgb(&self) -> Rgb {
        self.clone().into()
    }

    /// Convert the color to linear RGB.
    pub fn to_srgb(&self) -> Rgb {
        self.clone().into()
    }

    /// Convert the color to OkLuv.
    pub fn to_luv(&self) -> OkLuv {
        self.clone().into()
    }

    /// Convert the color to OkHsl
    pub fn to_hsl(&self) -> OkHsl {
        self.clone().into()
    }

    /// Convert the color to a vector.
    pub fn as_vec3(&self) -> DVec3 {
        self.clone().into()
    }

    /// Add an alpha channel to the color.
    pub fn with_a(&self, a: f64) -> CieXyza {
        CieXyza::new(self.x, self.y, self.z, a)
    }

    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from_slice(slice: &[f64; 3]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }
}

impl From<Rgb> for CieXyz {
    fn from(rgb: Rgb) -> Self {
        let rgb = rgb.as_vec3();
        let xyz = TO_RGB * rgb;
        Self::from(xyz)
    }
}

impl Into<Rgb> for CieXyz {
    fn into(self) -> Rgb {
        let xyz = self.as_vec3();
        let rgb = FROM_RGB * xyz;
        Rgb::from(rgb)
    }
}

impl From<DVec3> for CieXyz {
    fn from(v: DVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Into<DVec3> for CieXyz {
    fn into(self) -> DVec3 {
        dvec3(self.x, self.y, self.z)
    }
}

impl From<&[f64; 3]> for CieXyz {
    fn from(slice: &[f64; 3]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 3]> for CieXyz {
    fn into(self) -> [f64; 3] {
        self.as_array()
    }
}

impl CieXyza {
    pub const fn new(x: f64, y: f64, z: f64, a: f64) -> Self {
        Self { x, y, z, a }
    }

    /// Convert the color to linear RGBA.
    pub fn to_rgba(&self) -> Rgba {
        self.clone().into()
    }

    /// Convert the color to gamma corrected sRGBA.
    pub fn to_srgba(&self) -> Rgba {
        self.clone().into()
    }

    /// Convert the color to OkLuva.
    pub fn to_luva(&self) -> OkLuva {
        self.clone().into()
    }

    /// Convert the color to OkHsla
    pub fn to_hsla(&self) -> OkHsla {
        self.clone().into()
    }

    /// Convert the color to a vector.
    pub fn as_vec4(&self) -> DVec4 {
        self.clone().into()
    }

    /// Remove the alpha channel from the color.
    pub fn without_a(&self) -> CieXyz {
        CieXyz::new(self.x, self.y, self.z)
    }

    pub fn as_array(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.a]
    }

    pub fn from_slice(slice: &[f64; 4]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<Rgba> for CieXyza {
    fn from(rgba: Rgba) -> Self {
        let rgb = rgba.without_a().as_vec3();
        let xyz = TO_RGB * rgb;
        Self::new(xyz.x, xyz.y, xyz.z, rgba.a)
    }
}

impl Into<Rgba> for CieXyza {
    fn into(self) -> Rgba {
        let xyz = self.without_a().as_vec3();
        let rgb = FROM_RGB * xyz;
        Rgb::from(rgb).with_a(self.a)
    }
}

impl From<DVec4> for CieXyza {
    fn from(v: DVec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}

impl Into<DVec4> for CieXyza {
    fn into(self) -> DVec4 {
        dvec4(self.x, self.y, self.z, self.a)
    }
}

impl From<&[f64; 4]> for CieXyza {
    fn from(slice: &[f64; 4]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 4]> for CieXyza {
    fn into(self) -> [f64; 4] {
        self.as_array()
    }
}
