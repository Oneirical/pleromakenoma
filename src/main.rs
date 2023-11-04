use std::f32::consts::PI;
use bevy::{prelude::*, render::camera::Viewport, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pleroma & Kenoma".into(),
                        resolution: (1152.0, 648.0).into(),
                        //resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

#[derive(Component)]
struct LeftCamera;

#[derive(Component)]
struct RightCamera;

#[derive(Component)]
struct PolarityMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(1152.0/2.0+150., 648.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(350.0, -1500.0, 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(1152.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50.0, -180.0, 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(1152.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(500.0, -1680.0, 0.)),
        ..default()
    });
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        80, 2, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for i in 0..29{ // white markers
        let y_c = -100.0+(i as f32/7.0).floor()*120.0;
        let x_c = -590.0+(i as f32%7.0)*80.0;
        let tex = 6.0-(i as f32%7.0);
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : tex as usize,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: x_c, y: y_c, z: 0.0},
                ..default()
            },
            ..default()
        });
    }
    for i in 0..25{ // black markers
        let y_c = -1600.0+(i as f32/6.0).floor()*120.0;
        let x_c = 230.0+(i as f32%6.0)*80.0;
        let tex = i as f32%6.0;
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : tex as usize,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: x_c, y: y_c, z: 0.0},
                ..default()
            },
            ..default()
        });
    }
    for i in 0..4{ // white
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 8,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                //color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: 0.0, y: 260.0-(i as f32 * 120.0), z: 0.0},
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        });
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 9,
                custom_size: Some(Vec2::new(80.0, 80.0)),
                //color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: 0.0, y: 260.0-(i as f32 * 120.0), z: 0.0},
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        },
        PolarityMarker,
    ));
    }
    for i in 0..4{ // black
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 8,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: 120.0, y: -1500.+260.0-(i as f32 * 120.0), z: 0.0},
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        });
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 9,
                custom_size: Some(Vec2::new(80.0, 80.0)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3{ x: 120.0, y: -1500.+260.0-(i as f32 * 120.0), z: 0.0},
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        },
        PolarityMarker
    ));
    }
    commands.spawn((
        Camera2dBundle {
            transform: Transform{
                translation: Vec3 { x: -1152./4., y: 0., z: 0. },
                ..default()
            },
            camera: Camera{
                order: 0,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(1152/2, 648),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..default()
        },
        LeftCamera,
    ));
    commands.spawn((
        Camera2dBundle {
            transform: Transform{
                translation: Vec3 { x: 1152./4.+119., y: -1500., z: 0. },
                ..default()
            },
            camera: Camera{
                order: 1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(1152/2, 0),
                    physical_size: UVec2::new(1152/2, 648),
                    ..Default::default()
                }),
                ..Default::default()
            },
            camera_2d: Camera2d{
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RightCamera,
    ));
}


fn character_movement(
    mut characters: Query<(&mut Transform, &PolarityMarker)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}