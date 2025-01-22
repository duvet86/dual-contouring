pub struct SMat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m11: f32,
    pub m12: f32,
    pub m22: f32,
}

impl SMat3 {
    pub fn new(m00: f32, m01: f32, m02: f32, m11: f32, m12: f32, m22: f32) -> Self {
        Self::set_symmetric(m00, m01, m02, m11, m12, m22)
    }

    pub fn new_empty() -> Self {
        Self::clear()
    }

    pub fn clear() -> Self {
        Self {
            m00: 0.0,
            m01: 0.0,
            m02: 0.0,
            m11: 0.0,
            m12: 0.0,
            m22: 0.0,
        }
    }

    pub fn set_symmetric(m00: f32, m01: f32, m02: f32, m11: f32, m12: f32, m22: f32) -> Self {
        Self {
            m00,
            m01,
            m02,
            m11,
            m12,
            m22,
        }
    }

    pub fn set_symmetric_from_rhs(rhs: &SMat3) -> Self {
        Self {
            m00: rhs.m00,
            m01: rhs.m01,
            m02: rhs.m02,
            m11: rhs.m11,
            m12: rhs.m12,
            m22: rhs.m22,
        }
    }

    pub fn fnorm(self) -> f32 {
        ((self.m00 * self.m00)
            + (self.m01 * self.m01)
            + (self.m02 * self.m02)
            + (self.m01 * self.m01)
            + (self.m11 * self.m11)
            + (self.m12 * self.m12)
            + (self.m02 * self.m02)
            + (self.m12 * self.m12)
            + (self.m22 * self.m22))
            .sqrt()
    }

    pub fn off(&self) -> f32 {
        (2.0 * ((self.m01 * self.m01) + (self.m02 * self.m02) + (self.m12 * self.m12))).sqrt()
    }

    pub fn rot01(&self, c: f32, s: f32) -> Self {
        let cc = c * c;
        let ss = s * s;
        let mix = 2.0 * c * s * self.m01;

        SMat3::set_symmetric(
            cc * self.m00 - mix + ss * self.m11,
            0.0,
            c * self.m02 - s * self.m12,
            ss * self.m00 + mix + cc * self.m11,
            s * self.m02 + c * self.m12,
            self.m22,
        )
    }

    pub fn rot02(&self) -> Self {
        let (c, s) = calc_symmetric_givens_coefficients(self.m00, self.m02, self.m22);

        let cc = c * c;
        let ss = s * s;
        let mix = 2.0 * c * s * self.m02;

        SMat3::set_symmetric(
            cc * self.m00 - mix + ss * self.m22,
            c * self.m01 - s * self.m12,
            0.0,
            self.m11,
            s * self.m01 + c * self.m12,
            ss * self.m00 + mix + cc * self.m22,
        )
    }

    pub fn rot12(&self) -> Self {
        let (c, s) = calc_symmetric_givens_coefficients(self.m11, self.m12, self.m22);

        let cc = c * c;
        let ss = s * s;
        let mix = 2.0 * c * s * self.m12;

        SMat3::set_symmetric(
            self.m00,
            c * self.m01 - s * self.m02,
            s * self.m01 + c * self.m02,
            cc * self.m11 - mix + ss * self.m22,
            0.0,
            ss * self.m11 + mix + cc * self.m22,
        )
    }
}

#[derive(Clone, Copy)]
pub struct Mat3 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
}

impl Mat3 {
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    pub fn set(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    pub fn clear() -> Self {
        Self::set(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    pub fn set_from_rhs(rhs: &Mat3) -> Self {
        Self::set(
            rhs.m00, rhs.m01, rhs.m02, rhs.m10, rhs.m11, rhs.m12, rhs.m20, rhs.m21, rhs.m22,
        )
    }

    pub fn set_symmetric(a00: f32, a01: f32, a02: f32, a11: f32, a12: f32, a22: f32) -> Self {
        Self::set(a00, a01, a02, a01, a11, a12, a02, a12, a22)
    }

    pub fn set_symmetric_from_rhs(rhs: &SMat3) -> Self {
        Self::set_symmetric(rhs.m00, rhs.m01, rhs.m02, rhs.m11, rhs.m12, rhs.m22)
    }

    pub fn fnorm(&self) -> f32 {
        ((self.m00 * self.m00)
            + (self.m01 * self.m01)
            + (self.m02 * self.m02)
            + (self.m10 * self.m10)
            + (self.m11 * self.m11)
            + (self.m12 * self.m12)
            + (self.m20 * self.m20)
            + (self.m21 * self.m21)
            + (self.m22 * self.m22))
            .sqrt()
    }

    pub fn off(&self) -> f32 {
        ((self.m01 * self.m01)
            + (self.m02 * self.m02)
            + (self.m10 * self.m10)
            + (self.m12 * self.m12)
            + (self.m20 * self.m20)
            + (self.m21 * self.m21))
            .sqrt()
    }

    pub fn mmul(&self, b: &Mat3) -> Self {
        Mat3::new(
            self.m00 * b.m00 + self.m01 * b.m10 + self.m02 * b.m20,
            self.m00 * b.m01 + self.m01 * b.m11 + self.m02 * b.m21,
            self.m00 * b.m02 + self.m01 * b.m12 + self.m02 * b.m22,
            self.m10 * b.m00 + self.m11 * b.m10 + self.m12 * b.m20,
            self.m10 * b.m01 + self.m11 * b.m11 + self.m12 * b.m21,
            self.m10 * b.m02 + self.m11 * b.m12 + self.m12 * b.m22,
            self.m20 * b.m00 + self.m21 * b.m10 + self.m22 * b.m20,
            self.m20 * b.m01 + self.m21 * b.m11 + self.m22 * b.m21,
            self.m20 * b.m02 + self.m21 * b.m12 + self.m22 * b.m22,
        )
    }

    pub fn mmul_ata(&self) -> SMat3 {
        SMat3::set_symmetric(
            self.m00 * self.m00 + self.m10 * self.m10 + self.m20 * self.m20,
            self.m00 * self.m01 + self.m10 * self.m11 + self.m20 * self.m21,
            self.m00 * self.m02 + self.m10 * self.m12 + self.m20 * self.m22,
            self.m01 * self.m01 + self.m11 * self.m11 + self.m21 * self.m21,
            self.m01 * self.m02 + self.m11 * self.m12 + self.m21 * self.m22,
            self.m02 * self.m02 + self.m12 * self.m12 + self.m22 * self.m22,
        )
    }

    pub fn transpose(&self) -> Self {
        Mat3::new(
            self.m00, self.m10, self.m20, self.m01, self.m11, self.m21, self.m02, self.m12,
            self.m22,
        )
    }

    pub fn rot01_post(&self, c: f32, s: f32) -> Self {
        Mat3::new(
            c * self.m00 - s * self.m01,
            s * self.m00 + c * self.m01,
            self.m02,
            c * self.m10 - s * self.m11,
            s * self.m10 + c * self.m11,
            self.m12,
            c * self.m20 - s * self.m21,
            s * self.m20 + c * self.m21,
            self.m22,
        )
    }

    pub fn rot02_post(&self, c: f32, s: f32) -> Self {
        Mat3::new(
            c * self.m00 - s * self.m02,
            self.m01,
            s * self.m00 + c * self.m02,
            c * self.m10 - s * self.m12,
            self.m11,
            s * self.m10 + c * self.m12,
            c * self.m20 - s * self.m22,
            self.m21,
            s * self.m20 + c * self.m22,
        )
    }

    pub fn rot12_post(&self, c: f32, s: f32) -> Self {
        Mat3::new(
            self.m00,
            c * self.m01 - s * self.m02,
            s * self.m01 + c * self.m02,
            self.m10,
            c * self.m11 - s * self.m12,
            s * self.m11 + c * self.m12,
            self.m20,
            c * self.m21 - s * self.m22,
            s * self.m21 + c * self.m22,
        )
    }

    pub fn rotate01(&self, vtav: &SMat3) -> Self {
        if vtav.m01 == 0.0 {
            self.clone()
        } else {
            let (c, s) = calc_symmetric_givens_coefficients(vtav.m00, vtav.m01, vtav.m11);

            // let vtavRot01 = vtav.rot01(c, s);

            self.rot01_post(c, s)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_empty() -> Self {
        Self::clear()
    }

    pub fn clear() -> Self {
        Self::set(0.0, 0.0, 0.0)
    }

    pub fn set(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn set_from_rhs(rhs: &Vec3) -> Self {
        Self::set(rhs.x, rhs.y, rhs.z)
    }

    pub fn add_scaled(&self, s: f32, rhs: &Vec3) -> Self {
        Self::set(self.x + s * rhs.x, self.y + s * rhs.y, self.z + s * rhs.z)
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn scale(&self, s: f32) -> Self {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn normalize(&self) -> Self {
        let len2 = self.dot(&self);

        if len2.abs() < 1e-12 {
            Vec3::clear()
        } else {
            self.scale(1.0 / len2.sqrt())
        }
    }

    pub fn sub(&self, b: &Vec3) -> Self {
        Vec3::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }

    pub fn vmul(&self, a: &Mat3) -> Self {
        Vec3::new(
            (a.m00 * self.x) + (a.m01 * self.y) + (a.m02 * self.z),
            (a.m10 * self.x) + (a.m11 * self.y) + (a.m12 * self.z),
            (a.m20 * self.x) + (a.m21 * self.y) + (a.m22 * self.z),
        )
    }

    pub fn vmul_symmetric(&self, a: &SMat3) -> Vec3 {
        Vec3::new(
            (a.m00 * self.x) + (a.m01 * self.y) + (a.m02 * self.z),
            (a.m01 * self.x) + (a.m11 * self.y) + (a.m12 * self.z),
            (a.m02 * self.x) + (a.m12 * self.y) + (a.m22 * self.z),
        )
    }
}

pub fn calc_symmetric_givens_coefficients(a_pp: f32, a_pq: f32, a_qq: f32) -> (f32, f32) {
    if a_pq == 0.0 {
        (1.0, 0.0)
    } else {
        let tau = (a_qq - a_pp) / (2.0 * a_pq);
        let stt = (1.0 + tau * tau).sqrt();

        let temp1 = if tau >= 0.0 { tau + stt } else { tau - stt };
        let tan = 1.0 / temp1;

        let c = 1.0 / (1.0 + tan * tan).sqrt();

        (1.0 / (1.0 + tan * tan).sqrt(), tan * c)
    }
}
