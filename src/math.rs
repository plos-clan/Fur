pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec2f {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    pub fn scale(&self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn cross(&self, rhs: &Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let normalized = self.scale(self.length().recip());
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
    pub fn extend(&self, z: f32) -> Vec3f {
        Vec3f::new(self.x, self.y, z)
    }
}

impl Vec3f {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn scale(&self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let normalized = self.scale(self.length().recip());
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
    pub fn is_normalized(&self) -> bool {
        (self.length() - 1.0).abs() < 1e-4
    }
    pub fn extend(self, w: f32) -> Vec4f {
        Vec4f::new(self.x, self.y, self.z, w)
    }
    pub fn truncate(self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}

impl Vec4f {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    pub fn scale(&self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn normalize(&self) -> Self {
        let normalized = self.scale(self.length().recip());
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }
    pub fn is_normalized(&self) -> bool {
        (self.length_squared() - 1.0).abs() <= 1e-4
    }
    pub fn to_cartesian_point(&self) -> Vec3f {
        assert_ne!(self.w, 0.0);
        Vec3f::new(self.x / self.w, self.y / self.w, self.z / self.w)
    }
    pub fn to_cartesian_vector(&self) -> Vec3f {
        assert_eq!(self.w, 0.0);
        Vec3f::new(self.x, self.y, self.z)
    }
    pub fn truncate(&self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }
}


pub struct Mat4f {
    pub x_axis: Vec4f,
    pub y_axis: Vec4f,
    pub z_axis: Vec4f,
    pub w_axis: Vec4f,
}

impl Mat4f {
    pub const ZERO: Self = Self::from_cols(Vec4f::ZERO, Vec4f::ZERO, Vec4f::ZERO, Vec4f::ZERO);
    // 单位矩阵
    pub const IDENTITY: Self = Self::from_cols(Vec4f::X, Vec4f::Y, Vec4f::Z, Vec4f::W);

    pub const fn from_cols(x_axis: Vec4f, y_axis: Vec4f, z_axis: Vec4f, w_axis: Vec4f) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }
    pub const fn from_rows_slice(slice: &[f32]) -> Self {
        assert!(slice.len() >= 16);
        Self {
            x_axis: Vec4f::new(slice[0], slice[4], slice[8], slice[12]),
            y_axis: Vec4f::new(slice[1], slice[5], slice[9], slice[13]),
            z_axis: Vec4f::new(slice[2], slice[6], slice[10], slice[14]),
            w_axis: Vec4f::new(slice[3], slice[7], slice[11], slice[15]),
        }
    }
    // 转置
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec4f {
                x: self.x_axis.x,
                y: self.y_axis.x,
                z: self.z_axis.x,
                w: self.w_axis.x,
            },
            y_axis: Vec4f {
                x: self.x_axis.y,
                y: self.y_axis.y,
                z: self.z_axis.y,
                w: self.w_axis.y,
            },
            z_axis: Vec4f {
                x: self.x_axis.z,
                y: self.y_axis.z,
                z: self.z_axis.z,
                w: self.w_axis.z,
            },
            w_axis: Vec4f {
                x: self.x_axis.w,
                y: self.y_axis.w,
                z: self.z_axis.w,
                w: self.w_axis.w,
            },
        }
    }

    pub fn inverse(self) -> Self {
        todo!()
    }
}