use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

use crate::lookup_tables;

#[derive(Debug)]
struct Voxel {
    position: Vec3,
    value: f32,
}

struct Triangle {
    vertices: [[f32; 3]; 3],
}

pub struct MarchingCubesTerrain {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
}

fn generate_noise(grid_size: usize) -> Vec<Voxel> {
    let mut voxels: Vec<Voxel> = Vec::with_capacity(grid_size * grid_size * grid_size);
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                voxels.push(Voxel {
                    position: Vec3::new(x as f32, y as f32, z as f32),
                    value: simplex_noise_2d(Vec2::new(x as f32, y as f32)),
                });
            }
        }
    }
    voxels
}

fn get_cube_index(cube: &[Voxel; 8], density: f32) -> usize {
    let mut cube_index = 0;
    if cube[0].value < density {
        cube_index |= 1;
    }
    if cube[1].value < density {
        cube_index |= 2;
    }
    if cube[2].value < density {
        cube_index |= 4;
    }
    if cube[3].value < density {
        cube_index |= 8;
    }
    if cube[4].value < density {
        cube_index |= 16;
    }
    if cube[5].value < density {
        cube_index |= 32;
    }
    if cube[6].value < density {
        cube_index |= 64;
    }
    if cube[7].value < density {
        cube_index |= 128;
    }

    cube_index
}

fn polygonize_cube(
    cube: &[Voxel; 8],
    density: f32,
    vertex_buffer: &mut Vec<[f32; 3]>,
    index_buffer: &mut Vec<u32>,
) {
    let cube_index = get_cube_index(cube, density);
}

pub fn marching_cubes(
    grid_size: usize,
    density: f32,
    vertex_buffer: &mut Vec<Vec3>,
    index_buffer: &mut Vec<u32>,
) {
    let noise = generate_noise(grid_size);

    for i in 0..noise.len() {
        let x = i / (grid_size * grid_size);
        let y = (i / grid_size) % grid_size;
        let z = i % grid_size;

        if x >= grid_size - 1 || y >= grid_size - 1 || z >= grid_size - 1 {
            continue;
        }

        let cube = vec![
            &noise[x + y * grid_size + z * grid_size * grid_size],
            &noise[(x + 1) + y * grid_size + z * grid_size * grid_size],
            &noise[(x + 1) + (y + 1) * grid_size + z * grid_size * grid_size],
            &noise[x + (y + 1) * grid_size + z * grid_size * grid_size],
            &noise[x + y * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + y * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + (y + 1) * grid_size + (z + 1) * grid_size * grid_size],
            &noise[x + (y + 1) * grid_size + (z + 1) * grid_size * grid_size],
        ];
    }
}

#[cfg(test)]
mod tests {
    // use super::march;

    use bevy::prelude::Vec3;

    use super::marching_cubes;

    #[test]
    fn it_works() {
        let mut vertex_buffer: Vec<Vec3> = Vec::new();
        let mut index_buffer: Vec<u32> = Vec::new();
        marching_cubes(10, 0.4, &mut vertex_buffer, &mut index_buffer);
    }
}
