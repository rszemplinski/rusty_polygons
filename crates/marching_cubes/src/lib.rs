pub mod lookup_tables;

pub fn get_cube_index(iso_level: f32, grid: [f32; 8]) -> u8 {
    let mut cube_index = 0;
    if grid[0] < iso_level {
        cube_index |= 1;
    }
    if grid[1] < iso_level {
        cube_index |= 2;
    }
    if grid[2] < iso_level {
        cube_index |= 4;
    }
    if grid[3] < iso_level {
        cube_index |= 8;
    }
    if grid[4] < iso_level {
        cube_index |= 16;
    }
    if grid[5] < iso_level {
        cube_index |= 32;
    }
    if grid[6] < iso_level {
        cube_index |= 64;
    }
    if grid[7] < iso_level {
        cube_index |= 128;
    }
    cube_index
}
