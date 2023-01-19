#![allow(warnings)]
use std::f32::consts::PI;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;
use std::time::Duration;
use std::thread::sleep;
use std::thread;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        // Window Setup
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "chess".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
    .add_plugins(DefaultPickingPlugins)
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin)
        .register_type::<ChessBoard>()
        // Our Systems
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_basic_chess_board)
        .add_startup_system(spawn_camera)
        .add_system(camera_controls)
        .add_system(chess_movement_script)
        .add_system(chess_data_piece)
        .add_system(chess_data_square)
        // .add_system(test_selection)
        .run();
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[derive(Resource)]
pub struct ChessBoard{
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

#[derive(Resource)]
#[derive(Debug)]
struct DataBase{
    data:HashMap<String,Transform>,
    piece_flag:bool,
    square_flag:bool,
    piece_id:String,
}



fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    //loading... if u want to reuse it add it to commands.add_resource
    // commands.insert_resource(ChessBoard)
    let board = assets.load("board.glb#Scene0");
    let wpawn = assets.load("wpawn.glb#Scene0");
    let wrook = assets.load("wrook.glb#Scene0");
    let wbishop = assets.load("wbishop.glb#Scene0");
    let wknight = assets.load("wknight.glb#Scene0");
    let wking = assets.load("wking.glb#Scene0");
    let wqueen = assets.load("wqueen.glb#Scene0");
    let bpawn = assets.load("bpawn.glb#Scene0");
    let brook = assets.load("brook.glb#Scene0");
    let bbishop = assets.load("bbishop.glb#Scene0");
    let bknight = assets.load("bknight.glb#Scene0");
    let bking = assets.load("bking.glb#Scene0");
    let bqueen = assets.load("bqueen.glb#Scene0");
    let chess_board = ChessBoard{
        board,
        wking,
        wqueen,
        wrook,
        wbishop,
        wknight,
        wpawn,
        bking,
        bqueen,
        brook,
        bbishop,
        bknight,
        bpawn,
    };
    let db = DataBase{data:HashMap::new(),piece_flag:false,square_flag:false,piece_id:"".to_string()};
    commands.insert_resource(db);
    commands.insert_resource(chess_board);

}

pub fn spawn_basic_chess_board(
    mut commands:Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    chess_board:Res<ChessBoard>){

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.).into());

    commands.spawn(
        SceneBundle{
            scene:chess_board.board.clone(),
            transform:Transform::from_xyz(0., 0., 0.),
            ..Default::default()

        }
    )
        .insert(Name::new("chess board"));

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(-3., 0., 21.,)
                .with_scale(Vec3::new(3., 18., 3.))
        ))
        .insert(Name::new("BLACK KING"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bking.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./18.,1./3.)),
                        ..Default::default()

                })
            .insert(Name::new("black king"));

        }); 

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(3., 0., 21.,)
                .with_scale(Vec3::new(3., 15., 3.))
        ))
        .insert(Name::new("BLACK QUEEN"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bqueen.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./15.,1./3.)),
                        ..Default::default()

                })
            .insert(Name::new("black queen"));

        }); 


    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(9., 0., 21.,)
                .with_scale(Vec3::new(3., 14., 3.))
        ))
        .insert(Name::new("BLACK BISHOP"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bbishop.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./14.,1./3.)),
                        ..Default::default()

                })
            .insert(Name::new("black bishop"));

        }); 

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(-9., 0., 21.,)
                .with_scale(Vec3::new(3., 14., 3.))
        ))
        .insert(Name::new("BLACK BISHOP"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bbishop.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./14.,1./3.)),
                        ..Default::default()

                })
            .insert(Name::new("black bishop"));

        }); 


    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(-15., 0., 21.,)
                .with_scale(Vec3::new(3., 10., 3.))
        ))
        .insert(Name::new("BLACK KNIGHT"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bknight.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./10.,1./3.))
                        .with_rotation(Quat::from_rotation_y(1.5)),
                        ..Default::default()

                })
            .insert(Name::new("black knight"));

        }); 

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(15., 0., 21.,)
                .with_scale(Vec3::new(3., 10., 3.))
        ))
        .insert(Name::new("BLACK KNIGHT"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
    .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bknight.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./10.,1./3.))
                        .with_rotation(Quat::from_rotation_y(1.5)),
                        ..Default::default()

                })
            .insert(Name::new("black knight"));
        }); 

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(21., 0., 21.,)
                .with_scale(Vec3::new(3., 10., 3.))
        ))
        .insert(Name::new("BLACK ROOK"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
        .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.brook.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./10.,1./3.))
                        .with_rotation(Quat::from_rotation_y(1.5)),
                        ..Default::default()

                })
            .insert(Name::new("black rook"));
        }); 

    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(-21., 0., 21.,)
                .with_scale(Vec3::new(3., 10., 3.))
        ))
        .insert(Name::new("BLACK ROOK"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
        .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.brook.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./10.,1./3.))
                        .with_rotation(Quat::from_rotation_y(1.5)),
                        ..Default::default()
                })
            .insert(Name::new("black rook"));
        });

    for i in 0..8{
    commands
        .spawn(SpatialBundle::from_transform(
                Transform::from_xyz(-21.+i as f32*6., 0., 15.,)
                .with_scale(Vec3::new(3., 10., 3.))
        ))
        .insert(Name::new("BLACK PAWN"))
        .insert(meshes.add(shape::Cube::default().into()))
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color.clone()),
        })
        .insert(default_collider_color.clone())
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(
                SceneBundle{
                    scene:chess_board.bpawn.clone(),
                    transform:Transform::from_xyz(0., -0., 0.)
                        .with_scale(Vec3::new(1./3.,1./10.,1./3.))
                        .with_rotation(Quat::from_rotation_y(1.5)),
                        ..Default::default()
                })
            .insert(Name::new("black pawn"));
        });
    }

    //tiles
    for i in 0..8{
        for j in 0..8{
            commands
                .spawn(SpatialBundle::from_transform(
                        Transform::from_xyz(-21.+i as f32*6., 0. , -21.+j as f32*6.,)
                        .with_scale(Vec3::new(6., 1., 6.))
                ))
                .insert(Name::new("chess_square"))
                .insert(meshes.add(shape::Cube::default().into()))
                .insert(Highlighting {
                    initial: default_collider_color.clone(),
                    hovered: Some(selected_collider_color.clone()),
                    pressed: Some(selected_collider_color.clone()),
                    selected: Some(selected_collider_color.clone()),
                })
            .insert(default_collider_color.clone())
                .insert(PickableBundle::default());

        }
    }
    //white pieces
    commands.spawn(
        SceneBundle{
            scene:chess_board.wking.clone(),
            transform:Transform::from_xyz(-3., 0., -21.),
            ..Default::default()

        })
    .insert(Name::new("white king"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wqueen.clone(),
            transform:Transform::from_xyz(3., 0., -21.),
            ..Default::default()

        })
    .insert(Name::new("white queen"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wbishop.clone(),
            transform:Transform::from_xyz(9., 0., -21.),
            ..Default::default()

        })
    .insert(Name::new("white bishop"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wbishop.clone(),
            transform:Transform::from_xyz(-9., 0., -21.),
            ..Default::default()

        })
    .insert(Name::new("white bishop"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wknight.clone(),
            transform:Transform::from_xyz(15., 0., -21.)
                .with_rotation(Quat::from_rotation_y(-1.5)),
                ..Default::default()

        })
    .insert(Name::new("white knight"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wknight.clone(),
            transform:Transform::from_xyz(-15., 0., -21.)
                .with_rotation(Quat::from_rotation_y(1.5)),
                ..Default::default()

        })
    .insert(Name::new("white knight"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wrook.clone(),
            transform:Transform::from_xyz(21., 0., -21.),
            ..Default::default()

        })
    .insert(Name::new("white rook"));

    commands.spawn(
        SceneBundle{
            scene:chess_board.wrook.clone(),
            transform:Transform::from_xyz(-21., 0., -21.),
            ..Default::default()
        })
    .insert(Name::new("white rook"));

    for i in 0..8{
        commands.spawn(
            SceneBundle{
                scene:chess_board.wpawn.clone(),
                transform:Transform::from_xyz(-21.+i as f32*6., 0., -15.),
                ..Default::default()
            })
        .insert(Name::new("white pawn"));
    }

}


fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 150000.0,
                range:200.,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(0., 100.0, 0.),
            ..default()
        })
    .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.5, 35.0, 65.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert_bundle(PickingCameraBundle::default());
}



fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut db:ResMut<DataBase>,
    time: Res<Time>,
    ) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let mut up = camera.up();
    up.x = 0.0;
    up = up.normalize();

    let speed = 30.0;
    let rotate_speed = 0.3;
    //Leafwing
    if keyboard.pressed(KeyCode::K) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::J) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::H) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::L) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::U) {
        camera.translation += up * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::O) {
        camera.translation -= up * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::X, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::D) {
        camera.rotate_axis(Vec3::X, -rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::F) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::S) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }

    if keyboard.pressed(KeyCode::A) && db.piece_flag{
        let trans = db.data["piece"].translation;
        let mut camera = camera_query.single_mut();
        camera.translation.x = trans.x;
        camera.translation.y = trans.y+8.;
        camera.translation.z = trans.z;
    }
}

fn chess_data_piece(
    mut db:ResMut<DataBase>,
    mut selection:Query<(&Name,&Selection,&mut Transform,Entity)>,
    keyboard:Res<Input<KeyCode>>){

    for (name,selection,mut transform,entity) in selection.iter_mut(){
        if selection.selected() && name.to_string() != "chess_square".to_string(){
            db.data.insert("piece".to_string(),transform.clone());
            db.piece_id = format!("{:?}",entity);
            db.piece_flag=true;
        }
    }
}

//support script
fn chess_data_square(
    mut db:ResMut<DataBase>,
    mut selection:Query<(&Name,&Selection,&mut Transform)>,
    ){
    if db.piece_flag{
        for (name,selection,mut transform) in selection.iter_mut(){
            if selection.selected() && name.to_string() == "chess_square".to_string(){
                db.data.insert("square".to_string(),transform.clone());
                db.square_flag = true;
            }
        }

    }

}

fn chess_movement_script(
    mut db:ResMut<DataBase>,
    mut selection:Query<(&Selection,&mut Transform,Entity)>,
    ){
    if db.square_flag && db.piece_flag{
        for (selection,mut transform,entity) in selection.iter_mut(){
            if format!("{:?}",entity) == db.piece_id{
                db.piece_flag=false;
                db.square_flag=false;
                let trans = db.data[&"square".to_string()].translation;
                transform.translation = trans;
            }

        }
    }



}









