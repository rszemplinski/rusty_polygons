use bevy::{
    pbr::{MeshPipeline, MeshPipelineKey},
    prelude::{AssetServer, FromWorld, Handle, Resource, Shader},
    render::render_resource::SpecializedMeshPipeline,
};

#[derive(Resource)]
pub struct CustomRenderPipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

impl FromWorld for CustomRenderPipeline {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        Self {
            mesh_pipeline: world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("shaders/custom_render_pipeline.wgsl") as Handle<Shader>,
        }
    }
}

impl SpecializedMeshPipeline for CustomRenderPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
    ) -> Result<
        bevy::render::render_resource::RenderPipelineDescriptor,
        bevy::render::render_resource::SpecializedMeshPipelineError,
    > {
        todo!()
    }
}
