use std::{
    fmt::{Display, Formatter, LowerExp, LowerHex, Result as FmtResult, UpperExp, UpperHex},
    io::{self, Error as IoError},
};

use byteorder::ByteOrder;
use glam::*;

use super::*;

/// A color in sRGB (gamma corrected) space.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SRgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// A color in sRGB (gamma corrected) space with an alpha channel.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SRgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub const fn srgb(r: f64, g: f64, b: f64) -> SRgb {
    SRgb { r, g, b }
}

pub const fn srgba(r: f64, g: f64, b: f64, a: f64) -> SRgba {
    SRgba { r, g, b, a }
}

pub const fn srgb_u8(r: u8, g: u8, b: u8) -> SRgb {
    SRgb::from_bytes(&[r, g, b])
}

pub const fn srgba_u8(r: u8, g: u8, b: u8, a: u8) -> SRgba {
    SRgba::from_bytes(&[r, g, b, a])
}

pub fn srgb_u24(packed: u32) -> SRgb {
    SRgba::unpack_be(packed).without_a()
}

pub fn sargb_u32(packed: u32) -> SRgba {
    SRgba::unpack_be(packed)
}

/// sRGB gamma function.
pub fn gamma(x: f64) -> f64 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(0.4166666666666667) - 0.055
    }
}

/// Inverse sRGB gamma function.
pub fn inv_gamma(x: f64) -> f64 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

impl SRgb {
    pub const BLACK: Self = srgb(0.0, 0.0, 0.0);
    pub const GREY: Self = srgb(0.5, 0.5, 0.5);
    pub const WHITE: Self = srgb(1.0, 1.0, 1.0);
    pub const RED: Self = srgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = srgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = srgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = srgb(1.0, 1.0, 0.0);
    pub const CYAN: Self = srgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = srgb(1.0, 0.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Convert to linear RGB.
    pub fn to_rgb(&self) -> Rgb {
        self.clone().into()
    }

    /// Convert the color to CIE XYZ.
    pub fn to_xyz(&self) -> CieXyz {
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

    /// Convert to a vector with x, y, z components corresponding to r, g, b.
    pub fn as_vec3(&self) -> DVec3 {
        self.clone().into()
    }

    /// Add an alpha channel to the color.
    pub fn with_a(&self, a: f64) -> Rgba {
        Rgba::new(self.r, self.g, self.b, a)
    }

    /// Convert the color to a array of bytes in the format (R, G, B).
    pub fn to_bytes(&self) -> [u8; 3] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
        ]
    }

    /// Create a color from a array of bytes in the format (R, G, B).
    pub const fn from_bytes(by: &[u8; 3]) -> Self {
        Self::new(by[0] as f64 / 255.0, by[1] as f64 / 255.0, by[2] as f64 / 255.0)
    }

    pub fn as_array(&self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }

    pub fn from_slice(slice: &[f64; 3]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }
}

impl From<Rgb> for SRgb {
    fn from(rgb: Rgb) -> Self {
        Self::new(gamma(rgb.r), gamma(rgb.g), gamma(rgb.b))
    }
}

impl Into<Rgb> for SRgb {
    fn into(self) -> Rgb {
        Rgb::new(inv_gamma(self.r), inv_gamma(self.g), inv_gamma(self.b))
    }
}

impl From<DVec3> for SRgb {
    fn from(v: DVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Into<DVec3> for SRgb {
    fn into(self) -> DVec3 {
        dvec3(self.r, self.g, self.b)
    }
}

impl From<&[f64; 3]> for SRgb {
    fn from(slice: &[f64; 3]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 3]> for SRgb {
    fn into(self) -> [f64; 3] {
        self.as_array()
    }
}

impl Display for SRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let [r, g, b] = self.to_bytes();
        write!(f, "({r}, {g}, {b})")
    }
}

impl UpperExp for SRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$E}, {g:.precis$E}, {b:.precis$E})")
        } else {
            write!(f, "({r:E}, {g:E}, {b:E})")
        }
    }
}

impl LowerExp for SRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$e}, {g:.precis$e}, {b:.precis$e})")
        } else {
            write!(f, "({r:e}, {g:e}, {b:e})")
        }
    }
}

impl UpperHex for SRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width() && width == 3 {
            let [r, g, b] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:X}{g:X}{b:X}")
        } else {
            write!(f, "{:06X}", self.with_a(0.0).pack_be())
        }
    }
}

impl LowerHex for SRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width() && width == 3 {
            let [r, g, b] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:x}{g:x}{b:x}")
        } else {
            write!(f, "{:06x}", self.with_a(0.0).pack_be())
        }
    }
}

impl SRgba {
    pub const CLEAR: Self = srgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = srgba(0.0, 0.0, 0.0, 1.0);
    pub const GREY: Self = srgba(0.5, 0.5, 0.5, 1.0);
    pub const WHITE: Self = srgba(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = srgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = srgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = srgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = srgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = srgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = srgba(1.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to linear RGBA.
    pub fn to_rgba(&self) -> Rgba {
        self.clone().into()
    }

    /// Convert the color to CIE XYZ.
    pub fn to_xyza(&self) -> CieXyza {
        self.clone().into()
    }

    /// Convert the color to OkLuv.
    pub fn to_luva(&self) -> OkLuva {
        self.clone().into()
    }

    /// Convert the color to OkHsl
    pub fn to_hsla(&self) -> OkHsla {
        self.clone().into()
    }

    /// Convert to a vector with x, y, z, w components corresponding to r, g, b, a.
    pub fn as_vec4(&self) -> DVec4 {
        self.clone().into()
    }

    /// Remove the alpha channel from the color.
    pub fn without_a(&self) -> SRgb {
        SRgb::new(self.r, self.g, self.b)
    }

    /// Convert the color to a array of bytes in the format (R, G, B, A).
    pub fn to_bytes(&self) -> [u8; 4] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        ]
    }

    /// Create a color from a array of bytes in the format (R, G, B, A).
    pub const fn from_bytes(by: &[u8; 4]) -> Self {
        Self::new(
            by[0] as f64 / 255.0,
            by[1] as f64 / 255.0,
            by[2] as f64 / 255.0,
            by[3] as f64 / 255.0,
        )
    }

    pub fn as_array(&self) -> [f64; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_slice(slice: &[f64; 4]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    /// Pack the color into a 32-bit integer in the format 0xAARRGGBB (big endian) or 0xBBGGRRAA (little endian).
    pub fn pack<O: ByteOrder>(&self) -> u32 {
        let r = (self.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (self.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (self.b * 255.0).round().clamp(0.0, 255.0) as u8;
        let a = (self.a * 255.0).round().clamp(0.0, 255.0) as u8;
        O::read_u32(&[a, r, g, b])
    }

    /// Unpack a 32-bit integer in the format 0xAARRGGBB (big endian) or 0xBBGGRRAA (little endian) into an sRGBA color.
    pub fn unpack<O: ByteOrder>(packed: u32) -> Self {
        let buf = &mut [0; 4];
        O::write_u32(buf, packed);
        let a = buf[0] as f64 / 255.0;
        let r = buf[1] as f64 / 255.0;
        let g = buf[2] as f64 / 255.0;
        let b = buf[3] as f64 / 255.0;
        Self::new(r, g, b, a)
    }

    pub fn pack_le(&self) -> u32 {
        self.pack::<byteorder::LittleEndian>()
    }

    pub fn pack_be(&self) -> u32 {
        self.pack::<byteorder::BigEndian>()
    }

    pub fn unpack_le(packed: u32) -> Self {
        Self::unpack::<byteorder::LittleEndian>(packed)
    }

    pub fn unpack_be(packed: u32) -> Self {
        Self::unpack::<byteorder::BigEndian>(packed)
    }
}

impl From<Rgba> for SRgba {
    fn from(rgba: Rgba) -> Self {
        Self::new(gamma(rgba.r), gamma(rgba.g), gamma(rgba.b), rgba.a)
    }
}

impl Into<Rgba> for SRgba {
    fn into(self) -> Rgba {
        Rgba::new(inv_gamma(self.r), inv_gamma(self.g), inv_gamma(self.b), self.a)
    }
}

impl From<DVec4> for SRgba {
    fn from(v: DVec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}

impl Into<DVec4> for SRgba {
    fn into(self) -> DVec4 {
        dvec4(self.r, self.g, self.b, self.a)
    }
}

impl From<&[f64; 4]> for SRgba {
    fn from(slice: &[f64; 4]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 4]> for SRgba {
    fn into(self) -> [f64; 4] {
        self.as_array()
    }
}

impl Display for SRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let [r, g, b, a] = self.to_bytes();
        write!(f, "({r}, {g}, {b}, {a})")
    }
}

impl UpperExp for SRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b, a } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$E}, {g:.precis$E}, {b:.precis$E}, {a:.precis$E})")
        } else {
            write!(f, "({r:E}, {g:E}, {b:E}, {a:E})")
        }
    }
}

impl LowerExp for SRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b, a } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$e}, {g:.precis$e}, {b:.precis$e}, {a:.precis$e})")
        } else {
            write!(f, "({r:e}, {g:e}, {b:e}, {a:e})")
        }
    }
}

impl UpperHex for SRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width() && width == 4 {
            let [r, g, b, a] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{a:X}{r:X}{g:X}{b:X}")
        } else {
            write!(f, "{:08X}", self.pack_be())
        }
    }
}

impl LowerHex for SRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width() && width == 4 {
            let [r, g, b, a] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{a:x}{r:x}{g:x}{b:x}")
        } else {
            write!(f, "{:08x}", self.pack_be())
        }
    }
}

pub trait ReadSRgb: io::Read {
    fn read_srgb_u8(&mut self) -> Result<SRgb, IoError> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;
        Ok(SRgb::from_bytes(&buf))
    }

    fn read_srgba_u8(&mut self) -> Result<SRgba, IoError> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(SRgba::from_bytes(&buf))
    }
}
impl<R: io::Read + ?Sized> ReadSRgb for R {}

pub trait WriteSRgb: io::Write {
    fn write_srgb_u8(&mut self, rgb: SRgb) -> Result<(), IoError> {
        self.write_all(&rgb.to_bytes())
    }

    fn write_srgba_u8(&mut self, rgba: SRgba) -> Result<(), IoError> {
        self.write_all(&rgba.to_bytes())
    }
}
impl<W: io::Write + ?Sized> WriteSRgb for W {}
