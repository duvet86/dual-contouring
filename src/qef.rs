// use glm::Mat3x3;

// #[derive(Clone, Copy)]
// pub struct QefData {
//     ata_00: f32,
//     ata_01: f32,
//     ata_02: f32,
//     ata_11: f32,
//     ata_12: f32,
//     ata_22: f32,

//     atb_x: f32,
//     atb_y: f32,
//     atb_z: f32,

//     btb: f32,

//     mass_point_x: f32,
//     mass_point_y: f32,
//     mass_point_z: f32,

//     num_points: u32,
// }

// impl QefData {
//     pub fn clear() -> Self {
//         Self {
//             ata_00: 0.0,
//             ata_01: 0.0,
//             ata_02: 0.0,
//             ata_11: 0.0,
//             ata_12: 0.0,
//             ata_22: 0.0,
//             atb_x: 0.0,
//             atb_y: 0.0,
//             atb_z: 0.0,
//             btb: 0.0,
//             mass_point_x: 0.0,
//             mass_point_y: 0.0,
//             mass_point_z: 0.0,
//             num_points: 0,
//         }
//     }

//     pub fn set(
//         ata_00: f32,
//         ata_01: f32,
//         ata_02: f32,
//         ata_11: f32,
//         ata_12: f32,
//         ata_22: f32,

//         atb_x: f32,
//         atb_y: f32,
//         atb_z: f32,

//         btb: f32,

//         mass_point_x: f32,
//         mass_point_y: f32,
//         mass_point_z: f32,

//         num_points: u32,
//     ) -> Self {
//         Self {
//             ata_00,
//             ata_01,
//             ata_02,
//             ata_11,
//             ata_12,
//             ata_22,
//             atb_x,
//             atb_y,
//             atb_z,
//             btb,
//             mass_point_x,
//             mass_point_y,
//             mass_point_z,
//             num_points,
//         }
//     }

//     pub fn setFromQefData(rhs: &QefData) -> Self {
//         Self {
//             ata_00: rhs.ata_00,
//             ata_01: rhs.ata_01,
//             ata_02: rhs.ata_02,
//             ata_11: rhs.ata_11,
//             ata_12: rhs.ata_12,
//             ata_22: rhs.ata_22,
//             atb_x: rhs.atb_x,
//             atb_y: rhs.atb_y,
//             atb_z: rhs.atb_z,
//             btb: rhs.btb,
//             mass_point_x: rhs.mass_point_x,
//             mass_point_y: rhs.mass_point_y,
//             mass_point_z: rhs.mass_point_z,
//             num_points: rhs.num_points,
//         }
//     }

//     pub fn add(&self, rhs: &QefData) -> Self {
//         Self {
//             ata_00: self.ata_00 + rhs.ata_00,
//             ata_01: self.ata_01 + rhs.ata_01,
//             ata_02: self.ata_02 + rhs.ata_02,
//             ata_11: self.ata_11 + rhs.ata_11,
//             ata_12: self.ata_12 + rhs.ata_12,
//             ata_22: self.ata_22 + rhs.ata_22,
//             atb_x: self.atb_x + rhs.atb_x,
//             atb_y: self.atb_y + rhs.atb_y,
//             atb_z: self.atb_z + rhs.atb_z,
//             btb: self.btb + rhs.btb,
//             mass_point_x: self.mass_point_x + rhs.mass_point_x,
//             mass_point_y: self.mass_point_y + rhs.mass_point_y,
//             mass_point_z: self.mass_point_z + rhs.mass_point_z,
//             num_points: self.num_points + rhs.num_points,
//         }
//     }
// }

// pub struct QefSolver {
//     data: QefData,
//     ata: glm::Mat2x3,
//     atb: glm::Vec3,

//     mass_point: glm::Vec3,
//     x: glm::Vec3,
//     has_solution: bool,
// }

// impl QefSolver {
//     pub fn add(&self, px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32) -> Self {
//         let norm = glm::normalize(&glm::vec3(nx, ny, nz));
//         let dot = nx * px + ny;

//         let data = QefData {
//             ata_00: self.data.ata_00 + norm.x * norm.x,
//             ata_01: self.data.ata_01 + norm.x * norm.y,
//             ata_02: self.data.ata_02 + norm.x * norm.z,
//             ata_11: self.data.ata_11 + norm.y * norm.y,
//             ata_12: self.data.ata_12 + norm.y * norm.z,
//             ata_22: self.data.ata_22 + norm.z * norm.z,
//             atb_x: self.data.atb_x + dot * norm.x,
//             atb_y: self.data.atb_y + dot * norm.y,
//             atb_z: self.data.atb_z + dot * norm.z,
//             btb: self.data.btb + dot * dot,
//             mass_point_x: self.data.mass_point_x + px,
//             mass_point_y: self.data.mass_point_y + py,
//             mass_point_z: self.data.mass_point_z + pz,
//             num_points: self.data.num_points + 1,
//         };

//         Self {
//             has_solution: false,
//             data,
//             ata: self.ata,
//             atb: self.atb,
//             mass_point: self.mass_point,
//             x: self.x,
//         }
//     }

//     pub fn addVec(&self, p: &glm::Vec3, n: &glm::Vec3) -> Self {
//         self.add(p.x, p.y, p.z, n.x, n.y, n.z)
//     }

//     pub fn addQefData(&self, rhs: &QefData) -> Self {
//         Self {
//             has_solution: false,
//             data: self.data.add(rhs),
//             ata: self.ata,
//             atb: self.atb,
//             mass_point: self.mass_point,
//             x: self.x,
//         }
//     }

//     pub fn getData(&self) -> QefData {
//         self.data
//     }

//     pub fn getError(mut self, pos: &glm::Vec3) -> f32 {
//         if !self.has_solution {
//             self.ata = self.setAta();
//             self.atb = self.setAtb();
//         }

//         let atax = glm::vec3(
//             (self.ata.m11 * pos.x) + (self.ata.m12 * pos.y) + (self.ata.m13 * pos.z),
//             (self.ata.m12 * pos.x) + (self.ata.m21 * pos.y) + (self.ata.m22 * pos.z),
//             (self.ata.m13 * pos.x) + (self.ata.m22 * pos.y) + (self.ata.m23 * pos.z),
//         );

//         glm::dot(&pos, &atax) - 2.0 * glm::dot(&pos, &self.atb) + &self.data.btb
//     }

//     fn setAta(&self) -> glm::Mat2x3 {
//         glm::mat2x3(
//             self.data.ata_00,
//             self.data.ata_01,
//             self.data.ata_02,
//             self.data.ata_11,
//             self.data.ata_12,
//             self.data.ata_22,
//         )
//     }

//     fn setAtb(&self) -> glm::Vec3 {
//         glm::vec3(self.data.atb_x, self.data.atb_y, self.data.atb_z)
//     }

//     fn reset(&self) -> Self {
//         Self {
//             has_solution: false,
//             data: QefData::clear(),
//             ata: self.ata,
//             atb: self.atb,
//             mass_point: self.mass_point,
//             x: self.x,
//         }
//     }

//     pub fn solve(mut self, svd_tol: f32, svd_sweeps: u32, pinv_tol: f32) -> f32 {
//         if self.data.num_points == 0 {
//             panic!("Invalid arguments")
//         }

//         self.mass_point = glm::vec3(
//             self.data.mass_point_x,
//             self.data.mass_point_y,
//             self.data.mass_point_z,
//         );

//         self.mass_point = self.mass_point.scale(1.0 / self.data.num_points as f32);

//         self.ata = self.setAta();
//         self.atb = self.setAtb();

//         let tmpv = glm::vec3(
//             (self.ata.m11 * self.mass_point.x)
//                 + (self.ata.m12 * self.mass_point.y)
//                 + (self.ata.m13 * self.mass_point.z),
//             (self.ata.m12 * self.mass_point.x)
//                 + (self.ata.m21 * self.mass_point.y)
//                 + (self.ata.m22 * self.mass_point.z),
//             (self.ata.m13 * self.mass_point.x)
//                 + (self.ata.m22 * self.mass_point.y)
//                 + (self.ata.m23 * self.mass_point.z),
//         );

//         self.atb = self.atb - tmpv;

//         self.x = glm::vec3(0.0, 0.0, 0.0);

//         1.1

//         // const float result = Svd::solveSymmetric(this->ata, this->atb,
//         //               this->x, svd_tol, svd_sweeps, pinv_tol);
//         // VecUtils::addScaled(this->x, 1, this->massPoint);
//         // this->setAtb();
//         // outx.set(x);
//         // this->hasSolution = true;
//         // return result;
//     }

//     fn solveSymmetric(
//         A: &glm::Mat3x2,
//         b: &glm::Vec3,
//         x: &glm::Vec3,
//         svd_tol: f32,
//         svd_sweeps: u32,
//         pinv_tol: f32,
//     ) -> f32 {
//         1.1
//     }

//     // fn getSymmetricSvd(
//     //     a: &glm::Mat3x2,
//     //     mut vtav: glm::Mat3x2,
//     //     mut v: glm::Mat3,
//     //     tol: f32,
//     //     max_sweeps: u32,
//     // ) {
//     //     vtav = glm::mat3x2(a.m11, a.m12, a.m21, a.m22, a.m31, a.m32);
//     //     v = glm::mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)

//     //     let delta = tol * vtav.norm();

//     //     let mut i = 0;

//     //     // while i < max_sweeps && vtav.norm_squared() > delta  {
//     //     //     vtav.rotat

//     //     //     rotate01(vtav, v);
//     //     // rotate02(vtav, v);
//     //     // rotate12(vtav, v);
//     //     //     i = i + 1;
//     //     // }

//     // }
// }
