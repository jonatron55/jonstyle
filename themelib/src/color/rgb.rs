use std::{
    fmt::{Display, Formatter, LowerExp, LowerHex, Result as FmtResult, UpperExp, UpperHex},
    io::{self, Error as IoError},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Sub, SubAssign},
};

use super::*;
use byteorder::ByteOrder;
use glam::*;

/// A color in linear RGB space.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rgb {
    /// Red component of the color, in the range [0.0, 1.0].
    pub r: f64,

    /// Green component of the color, in the range [0.0, 1.0].
    pub g: f64,

    /// Blue component of the color, in the range [0.0, 1.0].
    pub b: f64,
}

/// A color in linear RGB space with an alpha channel.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rgba {
    /// Red component of the color, in the range [0.0, 1.0].
    pub r: f64,

    /// Green component of the color, in the range [0.0, 1.0].
    pub g: f64,

    /// Blue component of the color, in the range [0.0, 1.0].
    pub b: f64,

    /// Alpha component of the color, in the range [0.0, 1.0].
    pub a: f64,
}

pub const fn rgb(r: f64, g: f64, b: f64) -> Rgb {
    Rgb { r, g, b }
}

pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Rgba {
    Rgba { r, g, b, a }
}

pub const fn rgb_u8(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::from_bytes(&[r, g, b])
}

pub const fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Rgba {
    Rgba::from_bytes(&[r, g, b, a])
}

pub fn rgb_u24(packed: u32) -> Rgb {
    Rgba::unpack_be(packed).without_a()
}

pub fn rgba_u32(packed: u32) -> Rgba {
    Rgba::unpack_be(packed)
}

impl Rgb {
    pub const BLACK: Self = rgb(0.0, 0.0, 0.0);
    pub const GREY: Self = rgb(0.21404114048223244, 0.21404114048223244, 0.21404114048223244);
    pub const WHITE: Self = rgb(1.0, 1.0, 1.0);
    pub const RED: Self = rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = rgb(1.0, 1.0, 0.0);
    pub const CYAN: Self = rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = rgb(1.0, 0.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Convert to gamma-corrected sRGB.
    pub fn to_srgb(&self) -> SRgb {
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

    /// Linearly interpolate between two colors.
    pub fn lerp(&self, other: Self, t: f64) -> Self {
        Self::new(
            self.r.lerp(other.r, t),
            self.g.lerp(other.g, t),
            self.b.lerp(other.b, t),
        )
    }

    /// Return the minimum components of two colors.
    pub fn min(&self, other: Self) -> Self {
        Self::new(self.r.min(other.r), self.g.min(other.g), self.b.min(other.b))
    }

    /// Return the maximum components of two colors.
    pub fn max(&self, other: Self) -> Self {
        Self::new(self.r.max(other.r), self.g.max(other.g), self.b.max(other.b))
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

impl From<DVec3> for Rgb {
    fn from(v: DVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Into<DVec3> for Rgb {
    fn into(self) -> DVec3 {
        dvec3(self.r, self.g, self.b)
    }
}

impl From<&[f64; 3]> for Rgb {
    fn from(slice: &[f64; 3]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 3]> for Rgb {
    fn into(self) -> [f64; 3] {
        self.as_array()
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let [r, g, b] = self.to_bytes();
        write!(f, "({r}, {g}, {b})")
    }
}

impl UpperExp for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$E}, {g:.precis$E}, {b:.precis$E})")
        } else {
            write!(f, "({r:E}, {g:E}, {b:E})")
        }
    }
}

impl LowerExp for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$e}, {g:.precis$e}, {b:.precis$e})")
        } else {
            write!(f, "({r:e}, {g:e}, {b:e})")
        }
    }
}

impl UpperHex for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width()
            && width == 3
        {
            let [r, g, b] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:X}{g:X}{b:X}")
        } else {
            write!(f, "{:06X}", self.with_a(0.0).pack_be())
        }
    }
}

impl LowerHex for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width()
            && width == 3
        {
            let [r, g, b] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:x}{g:x}{b:x}")
        } else {
            write!(f, "{:06x}", self.with_a(0.0).pack_be())
        }
    }
}

impl Add<Rgb> for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign<Rgb> for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        rgb(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign<Rgb> for Rgb {
    fn sub_assign(&mut self, rhs: Rgb) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<f64> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        rgb(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f64> for Rgb {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        rgb(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<Rgb> for Rgb {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Div<f64> for Rgb {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        rgb(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl DivAssign<f64> for Rgb {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Div<Rgb> for Rgb {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        rgb(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl DivAssign<Rgb> for Rgb {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl Not for Rgb {
    type Output = Self;

    fn not(self) -> Self {
        Rgb::new(1.0 - self.r, 1.0 - self.g, 1.0 - self.b)
    }
}

impl Rgba {
    pub const CLEAR: Self = rgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = rgba(0.0, 0.0, 0.0, 1.0);
    pub const GREY: Self = rgba(0.21404114048223244, 0.21404114048223244, 0.21404114048223244, 1.0);
    pub const WHITE: Self = rgba(1.0, 1.0, 1.0, 1.0);
    pub const RED: Self = rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = rgba(1.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to gamma-corrected sRGBA.
    pub fn to_srgba(&self) -> SRgba {
        self.clone().into()
    }

    /// Convert the color to CIE XYZ.
    pub fn to_xyza(&self) -> CieXyza {
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

    /// Convert to a vector with x, y, z, w components corresponding to r, g, b, a.
    pub fn as_vec4(&self) -> DVec4 {
        self.clone().into()
    }

    /// Remove the alpha channel from the color.
    pub fn without_a(&self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
    }

    /// Return the minimum components of two colors.
    pub fn min(&self, other: Self) -> Self {
        Self::new(
            self.r.min(other.r),
            self.g.min(other.g),
            self.b.min(other.b),
            self.a.min(other.a),
        )
    }

    /// Return the maximum components of two colors.
    pub fn max(&self, other: Self) -> Self {
        Self::new(
            self.r.max(other.r),
            self.g.max(other.g),
            self.b.max(other.b),
            self.a.max(other.a),
        )
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

    /// Linearly interpolate between two colors.
    pub fn lerp(&self, other: Self, t: f64) -> Self {
        Self::new(
            self.r.lerp(other.r, t),
            self.g.lerp(other.g, t),
            self.b.lerp(other.b, t),
            self.a.lerp(other.a, t),
        )
    }

    /// Pack the color into a 32-bit integer in the format 0xRRGGBBAA (big endian) or 0xAABBGGRR (little endian).
    pub fn pack<O: ByteOrder>(&self) -> u32 {
        let r = (self.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (self.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (self.b * 255.0).round().clamp(0.0, 255.0) as u8;
        let a = (self.a * 255.0).round().clamp(0.0, 255.0) as u8;
        O::read_u32(&[r, g, b, a])
    }

    /// Unpack a 32-bit integer in the format 0xRRGGBBAA (big endian) or 0xAABBGGRR (little endian) into an RGBA color.
    pub fn unpack<O: ByteOrder>(packed: u32) -> Self {
        let buf = &mut [0; 4];
        O::write_u32(buf, packed);
        let r = buf[0] as f64 / 255.0;
        let g = buf[1] as f64 / 255.0;
        let b = buf[2] as f64 / 255.0;
        let a = buf[3] as f64 / 255.0;
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

impl From<DVec4> for Rgba {
    fn from(v: DVec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}

impl Into<DVec4> for Rgba {
    fn into(self) -> DVec4 {
        dvec4(self.r, self.g, self.b, self.a)
    }
}

impl From<&[f64; 4]> for Rgba {
    fn from(slice: &[f64; 4]) -> Self {
        Self::from_slice(&slice)
    }
}

impl Into<[f64; 4]> for Rgba {
    fn into(self) -> [f64; 4] {
        self.as_array()
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let [r, g, b, a] = self.to_bytes();
        write!(f, "({r}, {g}, {b}, {a})")
    }
}

impl UpperExp for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b, a } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$E}, {g:.precis$E}, {b:.precis$E}, {a:.precis$E})")
        } else {
            write!(f, "({r:E}, {g:E}, {b:E}, {a:E})")
        }
    }
}

impl LowerExp for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { r, g, b, a } = self;
        if let Some(precis) = f.precision() {
            write!(f, "({r:.precis$e}, {g:.precis$e}, {b:.precis$e}, {a:.precis$e})")
        } else {
            write!(f, "({r:e}, {g:e}, {b:e}, {a:e})")
        }
    }
}

impl UpperHex for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width()
            && width == 4
        {
            let [r, g, b, a] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:X}{g:X}{b:X}{a:X}")
        } else {
            write!(f, "{:08X}", self.pack_be())
        }
    }
}

impl LowerHex for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(width) = f.width()
            && width == 4
        {
            let [r, g, b, a] = self.to_bytes().map(|by| by >> 4);
            write!(f, "{r:x}{g:x}{b:x}{a:x}")
        } else {
            write!(f, "{:08x}", self.pack_be())
        }
    }
}

impl Add for Rgba {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub for Rgba {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a - rhs.a)
    }
}

impl SubAssign for Rgba {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl Mul<f64> for Rgba {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        rgba(self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs)
    }
}

impl MulAssign<f64> for Rgba {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Mul for Rgba {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        rgba(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a * rhs.a)
    }
}

impl MulAssign for Rgba {
    fn mul_assign(&mut self, rhs: Rgba) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl Div<f64> for Rgba {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        rgba(self.r / rhs, self.g / rhs, self.b / rhs, self.a / rhs)
    }
}

impl DivAssign<f64> for Rgba {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

impl Div for Rgba {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        rgba(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b, self.a / rhs.a)
    }
}

impl DivAssign for Rgba {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl Not for Rgba {
    type Output = Self;

    fn not(self) -> Self {
        rgba(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, 1.0 - self.a)
    }
}

pub trait ReadRgb: io::Read {
    fn read_rgb_u8(&mut self) -> Result<Rgb, IoError> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;
        Ok(Rgb::from_bytes(&buf))
    }

    fn read_rgba_u8(&mut self) -> Result<Rgba, IoError> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(Rgba::from_bytes(&buf))
    }
}
impl<R: io::Read + ?Sized> ReadRgb for R {}

pub trait WriteRgb: io::Write {
    fn write_rgb_u8(&mut self, rgb: Rgb) -> Result<(), IoError> {
        self.write_all(&rgb.to_bytes())
    }

    fn write_rgba_u8(&mut self, rgba: Rgba) -> Result<(), IoError> {
        self.write_all(&rgba.to_bytes())
    }
}
impl<W: io::Write + ?Sized> WriteRgb for W {}
