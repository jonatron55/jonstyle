use super::*;

// Secondary conversions that invoke the primary conversions.

impl Into<SRgb> for CieXyz {
    fn into(self) -> SRgb {
        self.to_rgb().into()
    }
}

impl Into<CieXyz> for SRgb {
    fn into(self) -> CieXyz {
        self.to_rgb().into()
    }
}

impl Into<CieXyz> for OkHsl {
    fn into(self) -> CieXyz {
        self.to_luv().into()
    }
}

impl Into<OkHsl> for CieXyz {
    fn into(self) -> OkHsl {
        self.to_luv().into()
    }
}

impl Into<Rgb> for OkHsl {
    fn into(self) -> Rgb {
        self.to_luv().into()
    }
}

impl Into<OkHsl> for Rgb {
    fn into(self) -> OkHsl {
        self.to_luv().into()
    }
}

impl Into<SRgb> for OkLuv {
    fn into(self) -> SRgb {
        self.to_rgb().into()
    }
}

impl Into<OkLuv> for SRgb {
    fn into(self) -> OkLuv {
        self.to_rgb().into()
    }
}

impl Into<OkHsl> for SRgb {
    fn into(self) -> OkHsl {
        self.to_luv().into()
    }
}

impl Into<SRgb> for OkHsl {
    fn into(self) -> SRgb {
        self.to_luv().into()
    }
}

impl Into<SRgba> for CieXyza {
    fn into(self) -> SRgba {
        self.to_rgba().into()
    }
}

impl Into<CieXyza> for SRgba {
    fn into(self) -> CieXyza {
        self.to_rgba().into()
    }
}

impl Into<CieXyza> for OkHsla {
    fn into(self) -> CieXyza {
        self.to_luva().into()
    }
}

impl Into<OkHsla> for CieXyza {
    fn into(self) -> OkHsla {
        self.to_luva().into()
    }
}

impl Into<Rgba> for OkHsla {
    fn into(self) -> Rgba {
        self.to_luva().into()
    }
}

impl Into<OkHsla> for Rgba {
    fn into(self) -> OkHsla {
        self.to_luva().into()
    }
}

impl Into<SRgba> for OkLuva {
    fn into(self) -> SRgba {
        self.to_rgba().into()
    }
}

impl Into<OkLuva> for SRgba {
    fn into(self) -> OkLuva {
        self.to_rgba().into()
    }
}

impl Into<OkHsla> for SRgba {
    fn into(self) -> OkHsla {
        self.to_luva().into()
    }
}

impl Into<SRgba> for OkHsla {
    fn into(self) -> SRgba {
        self.to_luva().into()
    }
}

#[cfg(feature = "image")]
impl From<image::Rgb<u8>> for SRgb {
    fn from(pixel: image::Rgb<u8>) -> SRgb {
        SRgb::from_bytes(&pixel.0)
    }
}

#[cfg(feature = "image")]
impl From<image::Rgba<u8>> for SRgba {
    fn from(pixel: image::Rgba<u8>) -> SRgba {
        SRgba::from_bytes(&pixel.0)
    }
}

#[cfg(feature = "image")]
impl From<image::Rgb<u8>> for Rgb {
    fn from(pixel: image::Rgb<u8>) -> Rgb {
        SRgb::from_bytes(&pixel.0).into()
    }
}

#[cfg(feature = "image")]
impl From<image::Rgba<u8>> for Rgba {
    fn from(pixel: image::Rgba<u8>) -> Rgba {
        SRgba::from_bytes(&pixel.0).into()
    }
}
