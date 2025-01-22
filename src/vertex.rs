use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::svd::Vec3;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub coords: Vec3,
    pub idx: i32,
}

impl Vertex {
    pub fn new(idx: i32, coords: Vec3) -> Self {
        Self { coords, idx }
    }

    pub fn new_empty() -> Self {
        Self {
            coords: Vec3::new_empty(),
            idx: 0,
        }
    }
}

pub struct VertexContainer {
    pub center_point: Vec3,
    pub geometric_center_point: Vec3,

    pub left_boundary_point: Vertex,
    pub rigth_boundary_point: Vertex,
    pub up_boundary_point: Vertex,
    pub down_boundary_point: Vertex,
    pub front_boundary_point: Vertex,
    pub back_boundary_point: Vertex,

    pub points: Vec<Vertex>,
}

impl VertexContainer {
    pub fn load_ply(file_path: &String) -> VertexContainer {
        let mut vc = VertexContainer {
            center_point: Vec3::new_empty(),
            geometric_center_point: Vec3::new_empty(),
            left_boundary_point: Vertex::new_empty(),
            rigth_boundary_point: Vertex::new_empty(),
            up_boundary_point: Vertex::new_empty(),
            down_boundary_point: Vertex::new_empty(),
            front_boundary_point: Vertex::new_empty(),
            back_boundary_point: Vertex::new_empty(),
            points: Vec::new(),
        };

        if let Ok(lines) = Self::read_lines(file_path) {
            for line in lines.skip(7).map_while(Result::ok) {
                let coors: Vec<f32> = line
                    .rsplit(' ')
                    .map(|line| line.parse::<f32>().unwrap())
                    .collect();

                vc.add_point(coors[0], coors[1], coors[2]);
            }
        }

        vc
    }

    fn add_point(&mut self, x: f32, y: f32, z: f32) {
        let idx = self.points.len();
        let c = Vec3::new(x, y, z);

        self.points.push(Vertex::new(idx as i32, c));

        if self.points.len() > 1 {
            if self.points.len() == 2 {
                self.center_point = self.points[idx - 1].coords;
            } else {
                self.center_point = Self::calculate_mid_point(
                    &self.center_point,
                    &self.points[idx].coords,
                    self.points.len() as f32 - 1.0,
                    1.0,
                );
            }
        }

        if self.points.len() == 1 {
            self.geometric_center_point = self.points[0].coords;
            self.up_boundary_point = self.points[0];
            self.down_boundary_point = self.points[0];
            self.left_boundary_point = self.points[0];
            self.rigth_boundary_point = self.points[0];
            self.front_boundary_point = self.points[0];
            self.back_boundary_point = self.points[0];
        } else {
            if self.points[self.points.len() - 1].coords.x < self.left_boundary_point.coords.x {
                self.left_boundary_point = self.points[self.points.len() - 1];
            }
            if self.points[self.points.len() - 1].coords.x > self.rigth_boundary_point.coords.x {
                self.rigth_boundary_point = self.points[self.points.len() - 1];
            }
            if self.points[self.points.len() - 1].coords.y < self.down_boundary_point.coords.y {
                self.down_boundary_point = self.points[self.points.len() - 1];
            }
            if self.points[self.points.len() - 1].coords.y > self.up_boundary_point.coords.y {
                self.up_boundary_point = self.points[self.points.len() - 1];
            }
            if self.points[self.points.len() - 1].coords.z < self.back_boundary_point.coords.z {
                self.back_boundary_point = self.points[self.points.len() - 1];
            }
            if self.points[self.points.len() - 1].coords.z > self.front_boundary_point.coords.z {
                self.front_boundary_point = self.points[self.points.len() - 1];
            }

            self.geometric_center_point.x =
                (self.left_boundary_point.coords.x + self.rigth_boundary_point.coords.x) / 2.0;
            self.geometric_center_point.y =
                (self.up_boundary_point.coords.y + self.down_boundary_point.coords.y) / 2.0;
            self.geometric_center_point.z =
                (self.back_boundary_point.coords.z + self.front_boundary_point.coords.z) / 2.0;
        }
    }

    fn calculate_mid_point(c1: &Vec3, c2: &Vec3, c1_coeff: f32, c2_coeff: f32) -> Vec3 {
        Vec3::new(
            (c1_coeff * c1.x + c2_coeff * c2.x) / (c1_coeff + c2_coeff),
            (c1_coeff * c1.y + c2_coeff * c2.y) / (c1_coeff + c2_coeff),
            (c1_coeff * c1.z + c2_coeff * c2.z) / (c1_coeff + c2_coeff),
        )
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
