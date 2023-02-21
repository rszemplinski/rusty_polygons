use bevy::prelude::*;

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
    // pub triangles: Vec<TriangleVe,
    pub indices: Vec<u32>,
}

fn get_cube_index(cube: &[&Voxel; 8], density: f32) -> usize {
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

fn interpolate(a: &Voxel, b: &Voxel, density: f32) -> Vec3 {
    let mu = (density - a.value) / (b.value - a.value);
    let x = a.position.x + mu * (b.position.x - a.position.x);
    let y = a.position.y + mu * (b.position.y - a.position.y);
    let z = a.position.z + mu * (b.position.z - a.position.z);
    Vec3::new(x, y, z)
}

fn polygonize_cube(
    cube: &[&Voxel; 8],
    density: f32,
    vertex_buffer: &mut Vec<Vec3>,
    index_buffer: &mut Vec<u32>,
) {
    let cube_index = get_cube_index(cube, density);
    
    if cube_index == 0 || cube_index == 255 {
        return;
    }
    
    let mut i = 0;
    loop {
        if lookup_tables::TRIANGLE_TABLE[cube_index as usize][i] == -1 {
            break;
        }

        let a0 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i] as usize][0];
        let b0 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i] as usize][1];

        let a1 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i + 1] as usize][0];
        let b1 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i + 1] as usize][1];
        
        let a2 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i + 2] as usize][0];
        let b2 = lookup_tables::EDGE_CONNECTIONS[lookup_tables::TRIANGLE_TABLE[cube_index as usize][i + 2] as usize][1];
        
        let vertex_a = interpolate(cube[a0 as usize], cube[b0 as usize], density);
        let vertex_b = interpolate(cube[a1 as usize], cube[b1 as usize], density);
        let vertex_c = interpolate(cube[a2 as usize], cube[b2 as usize], density);
        
        let index_count = index_buffer.len() as u32;
        vertex_buffer.push(vertex_a);
        vertex_buffer.push(vertex_b);
        vertex_buffer.push(vertex_c);
        
        index_buffer.push(index_count);
        index_buffer.push(index_count + 2);
        index_buffer.push(index_count + 1);
        
        i += 3;
    }
}

pub fn marching_cubes(
    points: Vec<f32>,
    grid_size: usize,
    density: f32,
    vertex_buffer: &mut Vec<Vec3>,
    index_buffer: &mut Vec<u32>,
) {
    let noise: Vec<Voxel> = Vec::new();

    for i in 0..noise.len() {
        let x = i % grid_size;
        let y = (i / grid_size) % grid_size;
        let z = i / (grid_size * grid_size);
        
        if x >= grid_size - 1 || y >= grid_size - 1 || z >= grid_size - 1 {
            continue;
        }

        let cube = [
            &noise[x + y * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + y * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + y * grid_size + z * grid_size * grid_size],
            &noise[x + y * grid_size + z * grid_size * grid_size],
            &noise[x + (y + 1) * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + (y + 1) * grid_size + (z + 1) * grid_size * grid_size],
            &noise[(x + 1) + (y + 1) * grid_size + z * grid_size * grid_size],
            &noise[x + (y + 1) * grid_size + (z + 1) * grid_size * grid_size],
        ];
        
        polygonize_cube(&cube, density, vertex_buffer, index_buffer);
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
        marching_cubes(vec![], 3, 0.2, &mut vertex_buffer, &mut index_buffer);
        println!("vertex_buffer: {:?}", vertex_buffer);
        println!("index_buffer: {:?}", index_buffer)
    }
}
