use alloc::sync::Arc;

/// This enum stores colors. \
/// Supported formats:
/// - RGB
/// - ARGB
/// - BGRA
/// - 256 color mode(U8)
///
/// This enum allows you to convert colors into different formats easily.
#[derive(Debug, Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    Argb(u8, u8, u8, u8),
    Bgra(u8, u8, u8, u8),
    U8(Palette, u8),
}

impl Color {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::Rgb(red, green, blue)
    }

    pub fn new_argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self::Argb(alpha, red, green, blue)
    }

    pub fn new_bgra(blue: u8, green: u8, red: u8, alpha: u8) -> Self {
        Self::Bgra(blue, green, red, alpha)
    }

    pub fn new_u8(palette: Palette, index: u8) -> Self {
        Self::U8(palette, index)
    }
}

impl Color {
    pub fn red(&self) -> u8 {
        match self {
            &Self::Argb(_, red, _, _) => red,
            &Self::Bgra(_, _, red, _) => red,
            &Self::Rgb(red, _, _) => red,
            Self::U8(palette, color_index) => palette.get_color(*color_index).red(),
        }
    }

    pub fn blue(&self) -> u8 {
        match self {
            &Self::Argb(_, _, _, blue) => blue,
            &Self::Bgra(blue, _, _, _) => blue,
            &Self::Rgb(_, _, blue) => blue,
            Self::U8(palette, color_index) => palette.get_color(*color_index).blue(),
        }
    }

    pub fn green(&self) -> u8 {
        match self {
            &Self::Argb(_, _, green, _) => green,
            &Self::Bgra(_, green, _, _) => green,
            &Self::Rgb(_, green, _) => green,
            Self::U8(palette, color_index) => palette.get_color(*color_index).green(),
        }
    }

    pub fn alpha(&self) -> u8 {
        match self {
            &Self::Argb(alpha, _, _, _) => alpha,
            &Self::Bgra(_, _, _, alpha) => alpha,
            &Self::Rgb(_, _, _) => 0,
            Self::U8(palette, color_index) => palette.get_color(*color_index).alpha(),
        }
    }
}

impl Color {
    pub fn mix(&self, other: &Color) -> Self {
        let mut red = ((self.red() as u32 * (0xff - self.alpha()) as u32
            + other.red() as u32 * (0xff - other.alpha()) as u32)
            >> 8) as u8;
        let mut green = ((self.green() as u32 * (0xff - self.alpha()) as u32
            + other.green() as u32 * (0xff - other.alpha()) as u32)
            >> 8) as u8;
        let mut blue = ((self.blue() as u32 * (0xff - self.alpha()) as u32
            + other.blue() as u32 * (0xff - other.alpha()) as u32)
            >> 8) as u8;

        if self.alpha() != 0xff {
            if self.red() != 0 {
                red += 1;
            }
            if self.green() != 0 {
                green += 1;
            }
            if self.blue() != 0 {
                blue += 1;
            }
        }
        if other.alpha() != 0xff {
            if other.red() != 0 {
                red += 1;
            }
            if other.green() != 0 {
                green += 1;
            }
            if other.blue() != 0 {
                blue += 1;
            }
        }

        let mut alpha = ((self.alpha() as u32 * self.alpha() as u32
            + other.alpha() as u32 * (0xff - self.alpha()) as u32)
            >> 8) as u8;

        if other.alpha() != 0 && self.alpha() != 0xff {
            alpha += 1;
        }
        if self.alpha() != 0 {
            alpha += 1;
        }

        Self::new_argb(alpha, red, green, blue)
    }

    pub fn as_rgb_tuple(&self) -> (u8, u8, u8) {
        match self {
            &Self::Argb(_, red, green, blue) => (red, green, blue),
            &Self::Bgra(blue, green, red, _) => (red, green, blue),
            &Self::Rgb(red, green, blue) => (red, green, blue),
            Self::U8(palette, color_index) => palette.get_color(*color_index).as_rgb_tuple(),
        }
    }

    pub fn as_argb_tuple(&self) -> (u8, u8, u8, u8) {
        match self {
            &Self::Argb(alpha, red, green, blue) => (alpha, red, green, blue),
            &Self::Bgra(blue, green, red, alpha) => (alpha, red, green, blue),
            &Self::Rgb(red, green, blue) => (0, red, green, blue),
            Self::U8(palette, color_index) => palette.get_color(*color_index).as_argb_tuple(),
        }
    }

    pub fn as_bgra_tuple(&self) -> (u8, u8, u8, u8) {
        let (alpha, red, green, blue) = self.as_argb_tuple();
        (blue, green, red, alpha)
    }

    pub fn as_u8(&self, palette: &Palette) -> Option<u8> {
        palette.try_find_color(self.clone())
    }
}

impl Color {
    pub fn as_argb_u32(&self) -> u32 {
        let (alpha, red, green, blue) = self.as_argb_tuple();
        u32::from_be_bytes([alpha, red, green, blue])
    }

    pub fn as_bgra_u32(&self) -> u32 {
        let (blue, green, red, alpha) = self.as_bgra_tuple();
        u32::from_be_bytes([blue, green, red, alpha])
    }

    pub fn as_0rgb_u32(&self) -> u32 {
        let (red, green, blue) = self.as_rgb_tuple();
        u32::from_be_bytes([0, red, green, blue])
    }

    pub fn as_rgb0_u32(&self) -> u32 {
        let (red, green, blue) = self.as_rgb_tuple();
        u32::from_be_bytes([red, green, blue, 0])
    }

    pub fn as_bgr0_u32(&self) -> u32 {
        let (red, green, blue) = self.as_rgb_tuple();
        u32::from_be_bytes([blue, green, red, 0])
    }
}

impl Color {
    pub fn from_argb_u32(color: u32) -> Self {
        let [alpha, red, green, blue] = color.to_be_bytes();
        Self::new_argb(alpha, red, green, blue)
    }

    pub fn from_0rgb_u32(color: u32) -> Self {
        let [_, red, green, blue] = color.to_be_bytes();
        Self::new_rgb(red, green, blue)
    }

    pub fn from_rgb0_u32(color: u32) -> Self {
        let [red, green, blue, _] = color.to_be_bytes();
        Self::new_rgb(red, green, blue)
    }

    pub fn from_bgra_u32(color: u32) -> Self {
        let [blue, green, red, alpha] = color.to_be_bytes();
        Self::new_bgra(blue, green, red, alpha)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.as_argb_tuple() == other.as_argb_tuple()
    }
}

/// This is the palette for 256 color mode. \
/// It stores 256 colors, and you can use them through the index.
#[derive(Debug, Clone)]
pub struct Palette {
    colors: Arc<[Color; 256]>,
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: Arc::new(core::array::from_fn(|_| {
                Color::new_argb(0, 0xff, 0xff, 0xff)
            })),
        }
    }
}

impl Palette {
    pub fn change_color(&mut self, index: u8, color: Color) -> &mut Self {
        Arc::make_mut(&mut self.colors)[index as usize] = color;
        self
    }

    pub fn get_color(&self, index: u8) -> Color {
        self.colors[index as usize].clone()
    }

    pub(crate) fn try_find_color(&self, required_color: Color) -> Option<u8> {
        self.colors
            .iter()
            .enumerate()
            .find(|(_, color)| **color == required_color)
            .map(|(id, _)| id as u8)
    }
}
