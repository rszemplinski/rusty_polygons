use bevy::prelude::Vec3;

use crate::lookup_tables::{EDGE_TABLE, TRI_TABLE};

pub struct MarchingCubes {
    pub iso_surface: f32,
}

#[derive(Debug)]
pub struct Triangle {
    pub vertex_1: Vec3,
    pub vertex_2: Vec3,
    pub vertex_3: Vec3,
}

impl MarchingCubes {
    pub fn new(iso_surface: f32) -> Self {
        Self { iso_surface }
    }

    fn interpolate_vertex(&self, a: &(Vec3, f32), b: &(Vec3, f32)) -> Vec3 {
        if (self.iso_surface - a.1).abs() < f32::EPSILON {
            return a.0;
        }
        if (self.iso_surface - b.1).abs() < f32::EPSILON {
            return b.0;
        }
        if (a.1 - b.1).abs() < f32::EPSILON {
            return a.0;
        }
        let mu = (self.iso_surface - a.1) / (b.1 - a.1);
        let x = a.0.x + mu * (b.0.x - a.0.x);
        let y = a.0.y + mu * (b.0.y - a.0.y);
        let z = a.0.z + mu * (b.0.z - a.0.z);
        Vec3::new(x, y, z)

        // (a.0 + b.0) / 2.0
    }

    pub fn polygonize(&self, grid: [(Vec3, f32); 8]) -> Vec<Triangle> {
        let mut cube_index = 0;

        let mut vertex_list: [Option<Vec3>; 12] = [None; 12];
        let mut triangle_list: Vec<Triangle> = Vec::with_capacity(5);

        if grid[0].1 < self.iso_surface {
            cube_index |= 1;
        }

        if grid[1].1 < self.iso_surface {
            cube_index |= 2;
        }

        if grid[2].1 < self.iso_surface {
            cube_index |= 4;
        }

        if grid[3].1 < self.iso_surface {
            cube_index |= 8;
        }

        if grid[4].1 < self.iso_surface {
            cube_index |= 16;
        }

        if grid[5].1 < self.iso_surface {
            cube_index |= 32;
        }

        if grid[6].1 < self.iso_surface {
            cube_index |= 64;
        }

        if grid[7].1 < self.iso_surface {
            cube_index |= 128;
        }

        if EDGE_TABLE[cube_index] == 0 {
            return triangle_list;
        }

        if EDGE_TABLE[cube_index] & 1 > 0 {
            vertex_list[0] = Some(Self::interpolate_vertex(self, &grid[0], &grid[1]))
        }

        if EDGE_TABLE[cube_index] & 2 > 0 {
            vertex_list[1] = Some(Self::interpolate_vertex(self, &grid[1], &grid[2]))
        }

        if EDGE_TABLE[cube_index] & 4 > 0 {
            vertex_list[2] = Some(Self::interpolate_vertex(self, &grid[2], &grid[3]))
        }

        if EDGE_TABLE[cube_index] & 8 > 0 {
            vertex_list[3] = Some(Self::interpolate_vertex(self, &grid[3], &grid[0]))
        }

        if EDGE_TABLE[cube_index] & 16 > 0 {
            vertex_list[4] = Some(Self::interpolate_vertex(self, &grid[4], &grid[5]))
        }

        if EDGE_TABLE[cube_index] & 32 > 0 {
            vertex_list[5] = Some(Self::interpolate_vertex(self, &grid[5], &grid[6]))
        }

        if EDGE_TABLE[cube_index] & 64 > 0 {
            vertex_list[6] = Some(Self::interpolate_vertex(self, &grid[6], &grid[7]))
        }

        if EDGE_TABLE[cube_index] & 128 > 0 {
            vertex_list[7] = Some(Self::interpolate_vertex(self, &grid[7], &grid[4]))
        }

        if EDGE_TABLE[cube_index] & 256 > 0 {
            vertex_list[8] = Some(Self::interpolate_vertex(self, &grid[0], &grid[4]))
        }

        if EDGE_TABLE[cube_index] & 512 > 0 {
            vertex_list[9] = Some(Self::interpolate_vertex(self, &grid[1], &grid[5]))
        }

        if EDGE_TABLE[cube_index] & 1024 > 0 {
            vertex_list[10] = Some(Self::interpolate_vertex(self, &grid[2], &grid[6]))
        }

        if EDGE_TABLE[cube_index] & 2048 > 0 {
            vertex_list[11] = Some(Self::interpolate_vertex(self, &grid[3], &grid[7]))
        }

        for i in (0..=12).step_by(3) {
            if TRI_TABLE[cube_index as usize][i] == -1 {
                break;
            }

            let triangle = Triangle {
                vertex_1: vertex_list[TRI_TABLE[cube_index][i] as usize].unwrap(),
                vertex_2: vertex_list[TRI_TABLE[cube_index][i + 1] as usize].unwrap(),
                vertex_3: vertex_list[TRI_TABLE[cube_index][i + 2] as usize].unwrap(),
            };

            triangle_list.push(triangle);
        }

        triangle_list
    }
}
