use bevy::{prelude::Plugin, render::RenderApp};

pub mod cpu;
pub mod lookup_tables;

pub struct MarchingCubesPlugin;

impl Plugin for MarchingCubesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let render_app = app.sub_app_mut(RenderApp);
    }
}
