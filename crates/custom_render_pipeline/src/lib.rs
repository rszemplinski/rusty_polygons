use bevy::{
    core_pipeline::core_3d::AlphaMask3d,
    prelude::Plugin,
    render::{
        render_phase::AddRenderCommand, render_resource::SpecializedMeshPipelines, RenderApp,
    },
};
use render_pipeline::{CustomRenderPipeline, DrawCustom};

mod render_pipeline;

pub struct CustomRenderPipelinePlugin;

impl Plugin for CustomRenderPipelinePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_plugin(ExtractComponentPlugin::<VoxelTerrainMesh>::default())
        // .add_plugin(terrain_uniforms::VoxelTerrainUniformsPlugin);
        app.sub_app_mut(RenderApp)
            .add_render_command::<AlphaMask3d, DrawCustom>()
            .init_resource::<CustomRenderPipeline>()
            .init_resource::<SpecializedMeshPipelines<CustomRenderPipeline>>();
        // .add_system_to_stage(RenderStage::Queue, queue_voxel_meshes);
    }
}
