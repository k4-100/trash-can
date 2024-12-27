use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use plugins::debug::DebugPlugin;
// use bevy_egui::{egui, EguiContext, EguiPlugin};
// use bevy_inspector_egui::prelude::*;
// use std::any::TypeId;

pub mod plugins;
pub mod utils;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::game::GamePlugin))
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(DebugPlugin)
        // .add_plugins(EguiPlugin)
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        // .add_systems(Update, inspector_ui)
        // .add_systems(Startup, ())
        // .add_systems(Update, ())
        .run();
}

// fn inspector_ui(world: &mut World) {
//     let Ok(egui_context) = world
//         .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
//         .get_single(world)
//     else {
//         return;
//     };
//     let mut egui_context = egui_context.clone();
//
//     egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
//         egui::ScrollArea::vertical().show(ui, |ui| {
//             // equivalent to `WorldInspectorPlugin`
//             bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
//
//             egui::CollapsingHeader::new("Materials").show(ui, |ui| {
//                 bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
//             });
//
//             ui.heading("Entities");
//             bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
//         });
//     });
// }
