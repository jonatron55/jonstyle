use lazy_static::lazy_static;

use crate::color::{
    ciexyz, gamma, inv_gamma, okhsl, okluv, rgb, rgb_u24, rgba, rgba_u32, rgba_u8, srgb, srgb_u24, srgba, srgba_u32,
    srgba_u8, CieXyz, OkHsl, OkLuv, Rgb, Rgba, SRgb, SRgba,
};

trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
}

#[test]
fn test_packed_color_layouts() {
    let bytes = [0x12, 0x34, 0x56, 0x78];
    let encoded = srgba_u8(0x12, 0x34, 0x56, 0x78);
    assert_eq!(encoded.pack_be(), 0x12345678);
    assert_eq!(encoded.pack_le(), 0x78563412);
    assert_eq!(SRgba::unpack_be(0x12345678).to_bytes(), bytes);
    assert_eq!(SRgba::unpack_le(0x78563412).to_bytes(), bytes);
    assert_eq!(srgba_u32(0x12345678).to_bytes(), bytes);
    assert_eq!(srgb_u24(0x123456).to_bytes(), [0x12, 0x34, 0x56]);

    let linear = rgba_u8(0x12, 0x34, 0x56, 0x78);
    assert_eq!(linear.pack_be(), 0x12345678);
    assert_eq!(linear.pack_le(), 0x78563412);
    assert_eq!(Rgba::unpack_be(0x12345678).to_bytes(), bytes);
    assert_eq!(Rgba::unpack_le(0x78563412).to_bytes(), bytes);
    assert_eq!(rgba_u32(0x12345678).to_bytes(), bytes);
    assert_eq!(rgb_u24(0x123456).to_bytes(), [0x12, 0x34, 0x56]);
}

fn assert_channels_close<const CHANNELS: usize>(actual: [f64; CHANNELS], expected: [f64; CHANNELS], tolerance: f64) {
    for (channel, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= tolerance,
            "channel {channel}: expected {expected:?}, got {actual:?}, tolerance {tolerance}"
        );
    }
}

fn xyz_reference_colors() -> [(Rgb, CieXyz); 5] {
    [
        (Rgb::BLACK, ciexyz(0.0, 0.0, 0.0)),
        (Rgb::RED, ciexyz(0.412390799, 0.212639006, 0.019330819)),
        (Rgb::GREEN, ciexyz(0.357584339, 0.715168679, 0.119194780)),
        (Rgb::BLUE, ciexyz(0.180480788, 0.072192315, 0.950532152)),
        (Rgb::WHITE, ciexyz(0.950455926, 1.0, 1.089057751)),
    ]
}

#[test]
fn test_rgb_to_xyz_reference() {
    for (linear, expected) in xyz_reference_colors() {
        assert_channels_close(linear.to_xyz().as_array(), expected.as_array(), 1e-8);
        assert_channels_close(
            linear.with_a(0.123456789).to_xyza().without_a().as_array(),
            expected.as_array(),
            1e-8,
        );
    }
}

#[test]
fn test_xyz_to_rgb_reference() {
    for (expected, xyz) in xyz_reference_colors() {
        assert_channels_close(xyz.to_rgb().as_array(), expected.as_array(), 1e-8);
        assert_channels_close(
            xyz.with_a(0.123456789).to_rgba().without_a().as_array(),
            expected.as_array(),
            1e-8,
        );
    }
}

#[test]
fn test_xyz_to_srgb_matches_conversion_chain() {
    let xyz = ciexyz(0.2, 0.3, 0.4);
    assert_channels_close(xyz.to_srgb().as_array(), xyz.to_rgb().to_srgb().as_array(), 1e-12);
}

#[test]
fn test_xyza_to_srgba_matches_conversion_chain() {
    let xyza = ciexyz(0.2, 0.3, 0.4).with_a(0.123456789);
    assert_channels_close(xyza.to_srgba().as_array(), xyza.to_rgba().to_srgba().as_array(), 1e-12);
}

#[test]
fn test_srgb_with_alpha_preserves_colorspace() {
    let encoded = srgb(0.2, 0.5, 0.8);
    let expected = srgba(0.2, 0.5, 0.8, 0.123456789).to_rgba();
    assert_channels_close(
        encoded.with_a(0.123456789).to_rgba().as_array(),
        expected.as_array(),
        1e-12,
    );
}

#[test]
fn test_gamma_reference_values() {
    for (encoded, linear) in [(0.0, 0.0), (0.5, 0.21404114048223255), (1.0, 1.0)] {
        assert_channels_close([inv_gamma(encoded)], [linear], 1e-14);
        assert_channels_close([gamma(linear)], [encoded], 1e-14);
    }
    assert_channels_close([gamma(0.5)], [0.7353569830524495], 1e-14);
    assert_channels_close(srgb(0.5, 0.5, 0.5).to_rgb().as_array(), [0.21404114048223255; 3], 1e-14);
    assert_channels_close(rgb(0.5, 0.5, 0.5).to_srgb().as_array(), [0.7353569830524495; 3], 1e-14);
}

#[test]
fn test_gamma_breakpoints() {
    for linear in [0.0031308 - 1e-8, 0.0031308, 0.0031308 + 1e-8] {
        let expected = if linear <= 0.0031308 {
            12.92 * linear
        } else {
            1.055 * f64::powf(linear, 1.0 / 2.4) - 0.055
        };
        assert_channels_close([gamma(linear)], [expected], 1e-14);
    }
    for encoded in [0.04045 - 1e-8, 0.04045, 0.04045 + 1e-8] {
        let expected = if encoded <= 0.04045 {
            encoded / 12.92
        } else {
            f64::powf((encoded + 0.055) / 1.055, 2.4)
        };
        assert_channels_close([inv_gamma(encoded)], [expected], 1e-14);
    }
}

#[test]
fn test_all_byte_values_roundtrip() {
    for byte in 0..=u8::MAX {
        let bytes = [byte, byte.wrapping_add(85), byte.wrapping_add(170), 255 - byte];
        assert_eq!(SRgb::from_bytes(&[bytes[0], bytes[1], bytes[2]]).to_bytes(), bytes[..3]);
        assert_eq!(Rgb::from_bytes(&[bytes[0], bytes[1], bytes[2]]).to_bytes(), bytes[..3]);
        let encoded = SRgba::from_bytes(&bytes);
        let linear = Rgba::from_bytes(&bytes);
        assert_eq!(encoded.to_bytes(), bytes);
        assert_eq!(linear.to_bytes(), bytes);
        assert_eq!(encoded.to_rgba().to_srgba().to_bytes(), bytes);
        assert_eq!(SRgba::unpack_be(encoded.pack_be()).to_bytes(), bytes);
        assert_eq!(SRgba::unpack_le(encoded.pack_le()).to_bytes(), bytes);
        assert_eq!(Rgba::unpack_be(linear.pack_be()).to_bytes(), bytes);
        assert_eq!(Rgba::unpack_le(linear.pack_le()).to_bytes(), bytes);
    }
}

#[test]
fn test_byte_rounding_and_clipping() {
    for (value, expected) in [
        (-0.1, 0),
        (0.0, 0),
        (0.49 / 255.0, 0),
        (0.5 / 255.0, 1),
        (127.49 / 255.0, 127),
        (0.5, 128),
        (1.0, 255),
        (1.1, 255),
    ] {
        let encoded = srgba(value, value, value, value);
        let linear = rgba(value, value, value, value);
        let bytes = [expected; 4];
        assert_eq!(encoded.to_bytes(), bytes, "sRGBA value {value}");
        assert_eq!(linear.to_bytes(), bytes, "RGBA value {value}");
        assert_eq!(encoded.without_a().to_bytes(), [expected; 3]);
        assert_eq!(linear.without_a().to_bytes(), [expected; 3]);
        assert_eq!(encoded.pack_be(), u32::from_be_bytes(bytes));
        assert_eq!(encoded.pack_le(), u32::from_le_bytes(bytes));
        assert_eq!(linear.pack_be(), u32::from_be_bytes(bytes));
        assert_eq!(linear.pack_le(), u32::from_le_bytes(bytes));
    }
}

#[test]
fn test_float_alpha_preserved() {
    for alpha in [0.0, 0.123456789, 0.5, 1.0] {
        let encoded = srgba(0.2, 0.5, 0.8, alpha);
        let linear = encoded.to_rgba();
        let xyz = encoded.to_xyza();
        let luv = encoded.to_luva();
        let hsl = encoded.to_hsla();
        let results = [
            linear.a,
            xyz.a,
            luv.a,
            hsl.a,
            linear.to_xyza().a,
            linear.to_luva().a,
            linear.to_hsla().a,
            xyz.to_rgba().a,
            xyz.to_srgba().a,
            xyz.to_luva().a,
            xyz.to_hsla().a,
            luv.to_xyza().a,
            luv.to_rgba().a,
            luv.to_srgba().a,
            luv.to_hsla().a,
            hsl.to_xyza().a,
            hsl.to_luva().a,
            hsl.to_rgba().a,
            hsl.to_srgba().a,
            linear.to_srgba().a,
        ];
        for actual in results {
            assert_eq!(actual, alpha);
        }
    }
}

#[test]
fn test_okhsl_endpoints_and_neutral() {
    for hue in [0.0, 1.0, 3.0, 6.0] {
        for saturation in [0.0, 0.5, 1.0] {
            for lightness in [0.0, 1.0] {
                let color = okhsl(hue, saturation, lightness);
                assert_eq!(color.to_luv().as_array(), [lightness, 0.0, 0.0]);
                assert_channels_close(color.to_srgb().as_array(), [lightness; 3], 1e-6);
            }
        }
        let neutral = okhsl(hue, 0.0, 0.5).to_srgb();
        assert_channels_close(neutral.as_array(), [neutral.r; 3], 1e-6);
    }
}

#[test]
fn test_okhsl_near_endpoints_finite() {
    for hue in [0.0, 1.0, 3.0, 6.0] {
        for saturation in [0.0, 0.5, 1.0] {
            for lightness in [1e-8, 1.0 - 1e-8] {
                let color = okhsl(hue, saturation, lightness);
                let luv = color.to_luv();
                let linear = color.to_rgb();
                let encoded = color.to_srgb();
                let recovered = encoded.to_hsl();
                for channels in [
                    luv.as_array(),
                    linear.as_array(),
                    encoded.as_array(),
                    recovered.as_array(),
                ] {
                    assert!(
                        channels.into_iter().all(f64::is_finite),
                        "non-finite result for {color:?}: {channels:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn test_okhsl_hue_periodicity() {
    for hue in [0.0, 0.7, 3.0, 6.0] {
        let expected = okhsl(hue, 0.75, 0.5).to_srgb();
        for turns in [-2.0, -1.0, 1.0, 2.0] {
            let actual = okhsl(hue + turns * std::f64::consts::TAU, 0.75, 0.5).to_srgb();
            assert_channels_close(actual.as_array(), expected.as_array(), 1e-12);
        }
    }
}

impl ApproxEq for OkLuv {
    fn approx_eq(&self, other: &Self) -> bool {
        (self.l - other.l).abs() < 1e-2 && (self.u - other.u).abs() < 1e-2 && (self.v - other.v).abs() < 1e-2
    }
}

impl ApproxEq for OkHsl {
    fn approx_eq(&self, other: &Self) -> bool {
        let s_eq = (self.s - other.s).abs() < 1e-2;
        let l_eq = (self.l - other.l).abs() < 1e-2;

        // Hue is undefined for achromatic colors (saturation ~ 0), so only
        // compare it when both colors are saturated enough for hue to be
        // meaningful.
        let h_eq = self.s < 1e-2 || other.s < 1e-2 || {
            let dh = (self.h.to_degrees() - other.h.to_degrees()).abs();
            dh.min(360.0 - dh) < 1.0
        };

        h_eq && s_eq && l_eq
    }
}

/// Compares two byte slices channel-by-channel, allowing a small rounding
/// difference per 8-bit channel.
fn bytes_approx_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| (i16::from(*x) - i16::from(*y)).abs() <= 1)
}

impl ApproxEq for SRgb {
    fn approx_eq(&self, other: &Self) -> bool {
        bytes_approx_eq(&self.to_bytes(), &other.to_bytes())
    }
}

impl ApproxEq for SRgba {
    fn approx_eq(&self, other: &Self) -> bool {
        bytes_approx_eq(&self.to_bytes(), &other.to_bytes())
    }
}

macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $($arg:tt)*) => {
        assert!(
            $left.approx_eq(&$right),
            $($arg)*
        );
    };
}

lazy_static! {
    // Reference values generated at full f64 precision from the reference
    // implementation at https://github.com/bottosson/bottosson.github.io.git
    // (see `misc/colorpicker/colorconversion.js`). Full precision is required
    // to reconstruct  highly saturated blues near the OkHsl gamut cusp
    // OkHsl hue is stored in radians; achromatic colors use a hue of zero.
    static ref TEST_COLORS: [(SRgb, OkLuv, OkHsl); 9] = [
        (srgb_u24(0x41007C), okluv(0.31901834737671403, 0.08045657664454703, -0.15245021867412095), okhsl(f64::to_radians(297.82313780010116), 1.0015012746550398, 0.2187558516622941)),
        (srgb_u24(0xC73B4A), okluv(0.5619829937129464, 0.16618671829861953, 0.05730581826069486), okhsl(f64::to_radians(19.025622470133136), 0.8083662848673744, 0.4921237018364035)),
        (srgb_u24(0x55CFBC), okluv(0.7806164791590512, -0.11227303043173248, -0.0026019296286092786), okhsl(f64::to_radians(181.32759314536136), 0.7387278635932258, 0.7448178772994302)),
        (srgb_u24(0xFF0000), okluv(0.6279553606145516, 0.22486306106597398, 0.1258462985307351), okhsl(f64::to_radians(29.23388519234263), 1.0000000001433997, 0.5680846525040862)),
        (srgb_u24(0x00FF00), okluv(0.8664396115356694, -0.23388757418790818, 0.17949847989672985), okhsl(f64::to_radians(142.49533888780996), 0.9999999700728788, 0.8445289645307816)),
        (srgb_u24(0x0000FF), okluv(0.4520137183853429, -0.03245698416876397, -0.3115281476783751), okhsl(f64::to_radians(264.052020638055), 0.9999999948631134, 0.3665653394260194)),
        (srgb_u24(0x808080), okluv(0.5998708017071177, 0.0, 0.0), okhsl(f64::to_radians(0.0), 0.0, 0.5357064594063015)),
        (srgb_u24(0xFFFFFF), okluv(1.0, 0.0, 0.0), okhsl(f64::to_radians(0.0), 0.0, 1.0)),
        (srgb_u24(0x000000), okluv(0.0, 0.0, 0.0), okhsl(f64::to_radians(0.0), 0.0, 0.0)),
    ];
}

#[test]
pub fn test_srgb_to_okluv() {
    for (srgb, luv, _) in TEST_COLORS.iter() {
        let actual = srgb.to_luv();
        assert_approx_eq!(
            actual,
            *luv,
            "Expected SRGB #{srgb:X} to convert to {luv}, actual {actual}"
        );
    }
}

#[test]
pub fn test_okluv_to_srgb() {
    for (srgb, luv, _) in TEST_COLORS.iter() {
        let actual = luv.to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected OkLuv {luv} to convert to SRGB #{srgb:X}, actual #{actual:X}"
        );
    }
}

#[test]
pub fn test_srgb_to_okhsl() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        let actual = srgb.to_hsl();
        assert_approx_eq!(
            actual,
            *okhsl,
            "Expected SRGB #{srgb:X} to convert to OkHsl {okhsl}, actual {actual}"
        );
    }
}

#[test]
pub fn test_okhsl_to_srgb() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        let actual = okhsl.to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected OkHsl {okhsl} to convert to SRGB #{srgb:X}, actual #{actual:X}"
        );
    }
}

#[test]
pub fn test_roundtrip() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        // OkHsl -> * -> OkHsl
        let actual = okhsl.to_luv().to_hsl();
        assert_approx_eq!(
            actual,
            *okhsl,
            "Expected OkHsl {okhsl} to round-trip through OkLuv, got {actual}"
        );
        let actual = okhsl.to_rgb().to_hsl();
        assert_approx_eq!(
            actual,
            *okhsl,
            "Expected OkHsl {okhsl} to round-trip through RGB, got {actual}"
        );

        // sRGB -> * -> sRGB
        let actual = srgb.to_rgb().to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected SRGB #{srgb:X} to round-trip through RGB, got #{actual:X}"
        );
        let actual = srgb.to_luv().to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected SRGB #{srgb:X} to round-trip through OkLuv, got #{actual:X}"
        );
        let actual = srgb.to_hsl().to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected SRGB #{srgb:X} to round-trip through OkHsl, got #{actual:X}"
        );
    }
}

/// Exercises the CIE XYZ conversion matrices, which are otherwise only reached
/// indirectly.
#[test]
pub fn test_xyz_roundtrip() {
    for (srgb, luv, _) in TEST_COLORS.iter() {
        // linear RGB <-> XYZ
        let actual = srgb.to_rgb().to_xyz().to_rgb().to_srgb();
        assert_approx_eq!(
            actual,
            *srgb,
            "Expected SRGB #{srgb:X} to round-trip through XYZ, got #{actual:X}"
        );

        // OkLuv <-> XYZ
        let actual = luv.to_xyz().to_luv();
        assert_approx_eq!(
            actual,
            *luv,
            "Expected OkLuv {luv} to round-trip through XYZ, got {actual}"
        );
    }
}

/// Verifies that the alpha channel is carried through the OkLuva / OkHsla
/// conversions unchanged.
#[test]
pub fn test_alpha_preserved() {
    for alpha in [0x00u8, 0x66, 0xC0, 0xFF] {
        let srgba = srgba_u8(0xC7, 0x3B, 0x4A, alpha);

        let actual = srgba.to_luva().to_srgba();
        assert_approx_eq!(
            actual,
            srgba,
            "Expected SRGBA #{srgba:X} to round-trip through OkLuva, got #{actual:X}"
        );
        let actual = srgba.to_hsla().to_srgba();
        assert_approx_eq!(
            actual,
            srgba,
            "Expected SRGBA #{srgba:X} to round-trip through OkHsla, got #{actual:X}"
        );

        // The alpha byte must be preserved exactly, not merely approximately.
        assert_eq!(
            srgba.to_luva().to_srgba().to_bytes()[3],
            alpha,
            "Expected alpha to be preserved through OkLuva"
        );
        assert_eq!(
            srgba.to_hsla().to_srgba().to_bytes()[3],
            alpha,
            "Expected alpha to be preserved through OkHsla"
        );
    }
}

/// Achromatic colors have zero saturation and a well-defined (finite) result,
/// exercising the NaN guards for a mid-gray that does not hit the l == 0 / l == 1
/// early-return paths.
#[test]
pub fn test_achromatic() {
    for grey in [srgb_u24(0x101010), srgb_u24(0x808080), srgb_u24(0xC0C0C0)] {
        let hsl = grey.to_hsl();
        assert!(hsl.h.is_finite(), "hue must be finite for gray #{grey:X}");
        assert!(hsl.s.is_finite(), "saturation must be finite for gray #{grey:X}");
        assert!(hsl.s < 1e-2, "gray #{grey:X} should be unsaturated, got s = {}", hsl.s);
    }
}

/// Round-trips a grid of sRGB colors through OkHsl to catch systematic drift
/// that a handful of hand-picked colors would miss.
#[test]
pub fn test_srgb_hsl_grid_roundtrip() {
    let steps = [0u8, 51, 102, 153, 204, 255];
    for &r in &steps {
        for &g in &steps {
            for &b in &steps {
                let original = SRgb::from_bytes(&[r, g, b]);
                let hsl = original.to_hsl();
                let actual = hsl.to_srgb();
                assert_approx_eq!(
                    actual,
                    original,
                    "Expected SRGB #{original:X} to round-trip through HSL, got #{actual:X}"
                );
            }
        }
    }
}
