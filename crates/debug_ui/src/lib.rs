use std::any::TypeId;

use bevy::{
    asset::{HandleId, ReflectAsset},
    prelude::{
        AppTypeRegistry, Component, CoreStage, IntoSystemDescriptor, Plugin, ReflectResource,
        Resource, StandardMaterial, World,
    },
    reflect::TypeRegistry,
};
use bevy_inspector_egui::{
    bevy_inspector::{
        self,
        hierarchy::{hierarchy_ui, SelectedEntities},
        ui_for_entities_shared_components, ui_for_entity_with_children,
    },
    egui, DefaultInspectorConfigPlugin,
};
use egui_dock::{NodeIndex, Tree};

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, show_ui_system.at_end());
    }
}

fn show_ui_system(world: &mut World) {
    let mut egui_context = world
        .resource_mut::<bevy_egui::EguiContext>()
        .ctx_mut()
        .clone();

    egui::Window::new("UI").show(&egui_context, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            // bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
        });
    });
}
