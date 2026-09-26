use lazy_static::lazy_static;

use crate::color::{okhsl, okluv, srgb_u24, srgba_u8, OkHsl, OkLuv, SRgb, SRgba};

trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
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
    ($left:expr, $right:expr) => {
        assert!(
            $left.approx_eq(&$right),
            "Expected {} to be approximately equal to {}",
            $left,
            $right
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
        assert_approx_eq!(srgb.to_luv(), *luv);
    }
}

#[test]
pub fn test_okluv_to_srgb() {
    for (srgb, luv, _) in TEST_COLORS.iter() {
        assert_approx_eq!(luv.to_srgb(), *srgb);
    }
}

#[test]
pub fn test_srgb_to_okhsl() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        assert_approx_eq!(srgb.to_hsl(), *okhsl);
    }
}

#[test]
pub fn test_okhsl_to_srgb() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        assert_approx_eq!(okhsl.to_srgb(), *srgb);
    }
}

#[test]
pub fn test_roundtrip() {
    for (srgb, _, okhsl) in TEST_COLORS.iter() {
        // OkHsl -> * -> OkHsl
        assert_approx_eq!(okhsl.to_luv().to_hsl(), *okhsl);
        assert_approx_eq!(okhsl.to_rgb().to_hsl(), *okhsl);

        // sRGB -> * -> sRGB
        assert_approx_eq!(srgb.to_rgb().to_srgb(), *srgb);
        assert_approx_eq!(srgb.to_luv().to_srgb(), *srgb);
        assert_approx_eq!(srgb.to_hsl().to_srgb(), *srgb);
    }
}

/// Exercises the CIE XYZ conversion matrices, which are otherwise only reached
/// indirectly.
#[test]
pub fn test_xyz_roundtrip() {
    for (srgb, luv, _) in TEST_COLORS.iter() {
        // linear RGB <-> XYZ
        assert_approx_eq!(srgb.to_rgb().to_xyz().to_rgb().to_srgb(), *srgb);
        // OkLuv <-> XYZ
        assert_approx_eq!(luv.to_xyz().to_luv(), *luv);
    }
}

/// Verifies that the alpha channel is carried through the OkLuva / OkHsla
/// conversions unchanged.
#[test]
pub fn test_alpha_preserved() {
    for alpha in [0x00u8, 0x66, 0xC0, 0xFF] {
        let srgba = srgba_u8(0xC7, 0x3B, 0x4A, alpha);

        assert_approx_eq!(srgba.to_luva().to_srgba(), srgba);
        assert_approx_eq!(srgba.to_hsla().to_srgba(), srgba);

        // The alpha byte must be preserved exactly, not merely approximately.
        assert_eq!(srgba.to_luva().to_srgba().to_bytes()[3], alpha);
        assert_eq!(srgba.to_hsla().to_srgba().to_bytes()[3], alpha);
    }
}

/// Achromatic colors have zero saturation and a well-defined (finite) result,
/// exercising the NaN guards for a mid-gray that does not hit the l == 0 / l == 1
/// early-return paths.
#[test]
pub fn test_achromatic() {
    for gray in [0x101010u32, 0x808080, 0xC0C0C0] {
        let hsl = srgb_u24(gray).to_hsl();
        assert!(hsl.h.is_finite(), "hue must be finite for gray {gray:#08X}");
        assert!(hsl.s.is_finite(), "saturation must be finite for gray {gray:#08X}");
        assert!(
            hsl.s < 1e-2,
            "gray {gray:#08X} should be unsaturated, got s = {}",
            hsl.s
        );
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
                assert_approx_eq!(hsl.to_srgb(), original);
            }
        }
    }
}
