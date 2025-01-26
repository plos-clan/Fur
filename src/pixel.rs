use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PixelFormat {
    ZeroRgb,
    RgbZero,
    Argb,
    Bgra,
    U8,
}

impl PixelFormat {
    pub fn size(&self) -> usize {
        match self {
            Self::ZeroRgb => 4,
            Self::RgbZero => 4,
            Self::Argb => 4,
            Self::Bgra => 4,
            Self::U8 => 1,
        }
    }
}

impl PixelFormat {
    pub fn color_as_u32(&self, color: &Color) -> u32 {
        match self {
            Self::ZeroRgb => color.as_0rgb_u32(),
            Self::RgbZero => color.as_rgb0_u32(),
            Self::Argb => color.as_argb_u32(),
            Self::Bgra => color.as_bgra_u32(),
            Self::U8 => match color {
                Color::U8(_, color) => *color as u32,
                _ => panic!("Only U8 colors can be converted to U8 pixels"),
            },
        }
    }
}

impl PixelFormat {
    pub fn u32_as_color(&self, color: u32) -> Color {
        match self {
            Self::ZeroRgb => Color::from_0rgb_u32(color),
            Self::RgbZero => Color::from_rgb0_u32(color),
            Self::Argb => Color::from_argb_u32(color),
            Self::Bgra => Color::from_bgra_u32(color),
            Self::U8 => panic!("U32 cannot be converted to U8 color."),
        }
    }
}
