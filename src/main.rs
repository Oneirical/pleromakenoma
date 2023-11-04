use std::{f32::consts::PI, time::Duration};
use bevy::{prelude::*, render::camera::Viewport, core_pipeline::clear_color::ClearColorConfig};
use bevy_tweening::{*, lens::TransformPositionLens};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy_tweening::TweeningPlugin)
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
        .add_systems(Startup, distribute_starting_cards)
        .add_systems(Update, summon_card)
        .add_systems(Update, character_movement)
        .add_event::<CardPlacedEvent>()
        .run();
}

#[derive(Component)]
struct PolarityMarker{
    polarity: i8,
    world: u8,
}

#[derive(Component)]
struct Card{
    value: i8,
    position: u8,
    active: bool,
}

#[derive(Event)]
struct CardPlacedEvent(Card, PolarityMarker);

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
        PolarityMarker{
            polarity: 0,
            world: i,
        }
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
        PolarityMarker{
            polarity: 0,
            world: i
        }
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
    ));
}

fn distribute_starting_cards(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    for i in 0..4 as u8{
        let img_path = "spritesheet.png".to_owned();
        let card_value = rand::thread_rng().gen_range(1..7);
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let card_value = card_value;
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Create a single animation (tween) to move an entity.
        let tween = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time.
            Duration::from_secs(1),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*i as f32, y: -400., z: 0. },
                end: Vec3::new(-400.+80.*i as f32, -270., 0.),
            },
        );
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : (card_value-1) as usize,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        },
        Card{
            value: card_value,
            position: i,
            active: false,
        },
        Animator::new(tween),
    ));
    }

}

fn summon_card(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut ev_card: EventReader<CardPlacedEvent>){
    for ev in ev_card.iter(){
        let card_value = rand::thread_rng().gen_range(1..7);
        let img_path = "spritesheet.png".to_owned();
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let tween = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time.
            Duration::from_secs(1),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*ev.0.position as f32, y: -400., z: 0. },
                end: Vec3::new(-400.+80.*ev.0.position as f32, -270., 0.),
            },
        );
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : card_value as usize,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            ..default()
        },
        Card{
            value: card_value,
            position: ev.0.position,
            active: false,
        },
        Animator::new(tween),
    ));
    }


}


fn character_movement(
    mut query: Query<(Entity, &mut Card)>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
) {
    for (entity_id, card) in query.iter_mut() {
        let card_num = card.position;
        let tween = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::BackInOut,
            // Animation time.
            Duration::from_secs(1),
            // The lens gives access to the Transform component of the Entity,
            // for the Animator to animate it. It also contains the start and
            // end values respectively associated with the progress ratios 0. and 1.
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*card_num as f32, y: -270., z: 0. },
                end: Vec3::new(-400.+80.*card_num as f32, -240., 0.),
            },
        );
        if input.pressed(KeyCode::Q) && card_num == 0 {
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        else if input.pressed(KeyCode::E) && card_num == 2 {
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        else if input.pressed(KeyCode::R) && card_num == 3 {
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        else if input.pressed(KeyCode::W) && card_num == 1 {
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }
}

