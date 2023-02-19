use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d;

use crate::lookup_tables;

fn generate_noise() -> [f32; 1000] {
    let mut src = [0.0; 1000];
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                src[x + y * 10 + z * 10 * 10] = simplex_noise_2d(Vec2::new(x as f32, y as f32));
            }
        }
    }
    src
}

#[derive(Debug)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

pub fn march() -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();

    let density = 0.5;
    let noise = generate_noise();
    for (i, _) in noise.iter().enumerate() {
        let x = i % 10;
        let y = (i / 10) % 10;
        let z = i / 100;

        if x >= 9 || y >= 9 || z >= 9 {
            continue;
        }

        let cube_corners = [
            noise[x + y * 10 + z * 10 * 10],
            noise[x + 1 + y * 10 + z * 10 * 10],
            noise[x + 1 + (y + 1) * 10 + z * 10 * 10],
            noise[x + (y + 1) * 10 + z * 10 * 10],
            noise[x + y * 10 + (z + 1) * 10 * 10],
            noise[x + 1 + y * 10 + (z + 1) * 10 * 10],
            noise[x + 1 + (y + 1) * 10 + (z + 1) * 10 * 10],
            noise[x + (y + 1) * 10 + (z + 1) * 10 * 10],
        ];

        let pos = Vec3::new(x as f32, y as f32, z as f32);
        let cube_index = get_cube_index(cube_corners, density);
        let edges = lookup_tables::TRIANGLE_TABLE[cube_index as usize];

        let mut i = 0;
        loop {
            if edges[i] == -1 {
                break;
            }

            let a0 = lookup_tables::EDGE_CONNECTIONS[edges[i] as usize][0];
            let b0 = lookup_tables::EDGE_CONNECTIONS[edges[i] as usize][1];

            let a1 = lookup_tables::EDGE_CONNECTIONS[edges[i + 1] as usize][0];
            let b1 = lookup_tables::EDGE_CONNECTIONS[edges[i + 1] as usize][1];

            let a2 = lookup_tables::EDGE_CONNECTIONS[edges[i + 2] as usize][0];
            let b2 = lookup_tables::EDGE_CONNECTIONS[edges[i + 2] as usize][1];

            let vertex_a = interpolate(
                Vec3::from_array(lookup_tables::CUBE_CORNERS[a0 as usize]),
                cube_corners[a0 as usize],
                Vec3::from_array(lookup_tables::CUBE_CORNERS[b0 as usize]),
                cube_corners[b0 as usize],
                density,
            );
            let vertex_b = interpolate(
                Vec3::from_array(lookup_tables::CUBE_CORNERS[a1 as usize]),
                cube_corners[a1 as usize],
                Vec3::from_array(lookup_tables::CUBE_CORNERS[b1 as usize]),
                cube_corners[b1 as usize],
                density,
            );
            let vertex_c = interpolate(
                Vec3::from_array(lookup_tables::CUBE_CORNERS[a2 as usize]),
                cube_corners[a2 as usize],
                Vec3::from_array(lookup_tables::CUBE_CORNERS[b2 as usize]),
                cube_corners[b2 as usize],
                density,
            );

            triangles.push(Triangle(vertex_a + pos, vertex_b + pos, vertex_c + pos));

            i += 3;
        }
    }

    triangles
}

fn interpolate(
    edge_vertex1: Vec3,
    value_at_vertex1: f32,
    edge_vertex2: Vec3,
    value_at_vertex2: f32,
    density: f32,
) -> Vec3 {
    let interpolated_value = edge_vertex1
        + (density - value_at_vertex1) * (edge_vertex2 - edge_vertex1)
            / (value_at_vertex2 - value_at_vertex1);
    interpolated_value
}

fn get_cube_index(grid: [f32; 8], density: f32) -> u8 {
    let mut cube_index = 0;
    if grid[0] < density {
        cube_index |= 1;
    }
    if grid[1] < density {
        cube_index |= 2;
    }
    if grid[2] < density {
        cube_index |= 4;
    }
    if grid[3] < density {
        cube_index |= 8;
    }
    if grid[4] < density {
        cube_index |= 16;
    }
    if grid[5] < density {
        cube_index |= 32;
    }
    if grid[6] < density {
        cube_index |= 64;
    }
    if grid[7] < density {
        cube_index |= 128;
    }

    cube_index
}

#[cfg(test)]
mod tests {
    use super::{generate_noise, march};

    #[test]
    fn it_works() {
        march();
    }
}
