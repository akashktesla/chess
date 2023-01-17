#![allow(warnings)]
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::time::Duration;
use std::thread::sleep;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Resource)]
pub struct GameAssets {
    chess_scene: Handle<Scene>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ChessBoard{
    board: Handle<Scene>,
    wking: Handle<Scene>,
    wqueen: Handle<Scene>,
    wrook: Handle<Scene>,
    wbishop: Handle<Scene>,
    wknight: Handle<Scene>,
    wpawn: Handle<Scene>,
    bking: Handle<Scene>,
    bqueen: Handle<Scene>,
    brook: Handle<Scene>,
    bbishop: Handle<Scene>,
    bknight: Handle<Scene>,
    bpawn: Handle<Scene>,
}

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin)
        .register_type::<ChessBoard>()
        // Our Systems
        .add_startup_system(asset_loading)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_camera)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    //loading... if u want to reuse it add it to commands.add_resource
    let board_handle = assets.load("chess_board.glb#Scene0");
    commands.spawn(
        SceneBundle{
            scene:board_handle.clone(),
            transform:Transform::from_xyz(0., 0., 0.),
            ..Default::default()

        }
        );
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
