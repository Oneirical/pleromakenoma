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
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Startup, distribute_starting_cards)
        .add_systems(Update, select_card)
        .add_systems(Update, move_text_labels)
        .add_systems(Update, push_world_polarity)
        .add_systems(Update, banish_and_replace)
        .add_systems(Update, claim_balanced_worlds)
        .add_systems(Update, swap_pleroma_kenoma)
        .run();
}

#[derive(Component)]
struct PolarityMarker{
    polarity: i8,
    world: u8,
    dimension: bool,
}

#[derive(Component)]
struct Card{
    value: i8,
    position: u8,
    active: bool,
}

#[derive(Component)]
struct TextLabel{
    number: u8
}

#[derive(Component)]
struct SwapSpace{}

#[derive(Component)]
struct FifthMarker{}

#[derive(Component)]
struct Pleromic{
    pleroma: bool,
    dist: f32,
}

#[derive(Component)]
struct Active{}

#[derive(Component)]
struct Deck{
    capacity: u16
}

#[derive(Component)]
struct BalancedWorlds{
    capacity: u16
}

#[derive(Component)]
struct WorldManager{
    kenoma: bool,
}

#[derive(Component)]
struct Dimension{
    name: String,
    world: u8,
    pleroma: bool,
}

static mut WORLD_PHASE: i8 = 0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Rectangle
    commands.spawn(WorldManager{kenoma: true});
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
        let options = [-4,-3,-2,2,3,4];
        let starting_offset: Vec<_> = options
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
        let starting_offset = *starting_offset[0];
        let end_x : f32 = if starting_offset > 0{
            190. + (starting_offset-2) as f32*80.
        }
        else {
            -110. + (starting_offset+1) as f32*80.
        };
        commands.spawn((SpriteSheetBundle {
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
        },
        Dimension{
            name: "Goemorphos".to_owned(),
            world: i,
            pleroma: false
        }
        ));
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 9,
                custom_size: Some(Vec2::new(80.0, 80.0)),
                //color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(end_x, 260.0-(i as f32 * 120.0), 0.),
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        },
        PolarityMarker{
            polarity: starting_offset,
            world: i,
            dimension: false,
        }
    ));
            commands.spawn((SpriteSheetBundle {
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
        },
        Dimension{
            name: "Goemorphos".to_owned(),
            world: i,
            pleroma: true,
        }
        ));
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 9,
                custom_size: Some(Vec2::new(80.0, 80.0)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(end_x+120., -1500.+260.0-(i as f32 * 120.0), 0.),
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        },
        PolarityMarker{
            polarity: starting_offset,
            world: i,
            dimension: true,
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
            },
            ..default()
        },
    ));
}

fn distribute_starting_cards(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let card_values = [rand::thread_rng().gen_range(1..7),rand::thread_rng().gen_range(1..7),rand::thread_rng().gen_range(1..7),rand::thread_rng().gen_range(1..7)];
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(15.9, 15.9),
        80, 2, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for i in 0..4_u8{
        let img_path = "spritesheet.png".to_owned();
        let card_value = card_values[i as usize];
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*i as f32, y: -400., z: 0. },
                end: Vec3::new(-400.+80.*i as f32, -250., 0.),
            },
        );
        let tween_text = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time.
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*i as f32, y: -400., z: 0. },
                end: Vec3::new(-400.+80.*i as f32, -300., 0.),
            },
        );
        let font = asset_server.load("Play-Regular.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;
        let bindings = ["1","2","3","4"];
        commands.spawn( // card hotkeys
            (
                Text2dBundle {
                    text: Text::from_section(bindings[i as usize], text_style.clone())
                    .with_alignment(text_alignment),
                ..default()
                },
                Animator::new(tween_text),
                TextLabel{
                    number: i,
                },
                Pleromic{ pleroma: false, dist: 100.},
                Active{}
            )
        );
        commands.spawn((SpriteSheetBundle { // cards
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
        Pleromic{ pleroma: false, dist: 675.},
        Active{}
        ));
    }

    // WHITE
    // 
    // 
    
    for i in 0..4_u8{
        let img_path = "spritesheet.png".to_owned();
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let card_value = card_values[i as usize];
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*i as f32, y: -1500.-400., z: 0. },
                end: Vec3::new(-400.+80.*i as f32, -1500.-250., 0.),
            },
        );
        let tween_text = Tween::new(
            // Use a quadratic easing on both endpoints.
            EaseFunction::QuadraticInOut,
            // Animation time.
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*i as f32, y: -1500.-400., z: 0. },
                end: Vec3::new(-400.+80.*i as f32, -1500.-300., 0.),
            },
        );
        let font = asset_server.load("Play-Regular.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::BLACK,
        };
        let text_alignment = TextAlignment::Center;
        let bindings = ["1","2","3","4"];
        commands.spawn( // card hotkeys
            (
                Text2dBundle {
                    text: Text::from_section(bindings[i as usize], text_style.clone())
                    .with_alignment(text_alignment),
                ..default()
                },
                Animator::new(tween_text),
                TextLabel{
                    number: i,
                },
                Pleromic{ pleroma: true, dist: 100.},
            )
        );
        commands.spawn((SpriteSheetBundle { // cards
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : (card_value-1) as usize,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                color: Color::rgb(0.0, 0.0, 0.0),
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
        Pleromic{ pleroma: true, dist: 675.},
        ));
    }

    // END
    // 
    // 


    let tween_deck = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -480., y: -400., z: 0. },
            end: Vec3::new(-480., -218., 0.),
        },
    );
    let tween_num_bal = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -480., y: -400., z: 0. },
            end: Vec3::new(-480., -278., 0.),
        },
    );
    let tween_bal = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -520., y: -400., z: 0. },
            end: Vec3::new(-520., -280., 0.),
        },
    );
    let tween = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -520., y: -400., z: 0. },
            end: Vec3::new(-520., -220., 0.),
        },
    );
    let font = asset_server.load("Play-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn( // deck counter
        (
            Text2dBundle {
                text: Text::from_section("64", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
            },
            Animator::new(tween_deck),
            Deck{
                capacity: 64,
            },
            Pleromic{ pleroma: false, dist: 1110.},
            Active{},
        )
    );
    commands.spawn( // world counter
        (
            Text2dBundle {
                text: Text::from_section("0", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
            },
            Animator::new(tween_num_bal),
            BalancedWorlds{
                capacity: 0,
            },
            Pleromic{ pleroma: false, dist: 1110.},
            Active{},
        )
    );
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };
    let tween_deck = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -480., y: -1500.-400., z: 0. },
            end: Vec3::new(-480., -1500.-218., 0.),
        },
    );
    let tween_num_bal = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -480., y: -1500.-400., z: 0. },
            end: Vec3::new(-480., -1500.-278., 0.),
        },
    );
    commands.spawn( // deck counter
        (
            Text2dBundle {
                text: Text::from_section("64", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
            },
            Animator::new(tween_deck),
            Deck{
                capacity: 64,
            },
            Pleromic{ pleroma: true, dist: 1110.},
        )
    );
    commands.spawn( // world counter
        (
            Text2dBundle {
                text: Text::from_section("0", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
            },
            Animator::new(tween_num_bal),
            BalancedWorlds{
                capacity: 0,
            },
            Pleromic{ pleroma: true, dist: 1110.},
        )
    );

    commands.spawn((SpriteSheetBundle { // deck icon
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 10_usize,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        ..default()
    },
    Animator::new(tween),
    Pleromic{ pleroma: false, dist: 1110.},
    Active{},
    ));
    commands.spawn((SpriteSheetBundle { // world icon
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 8_usize,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_z(PI/4.0),
            ..default()
        },
        ..default()
    },
    Pleromic{ pleroma: false, dist: 1110.},
    Active{},
    Animator::new(tween_bal),
    ));
    let tween_bal = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -520., y: -1500.-400., z: 0. },
            end: Vec3::new(-520., -1500.-280., 0.),
        },
    );
    let tween = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Animation time.
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3 { x: -520., y: -1500.-400., z: 0. },
            end: Vec3::new(-520., -1500.-220., 0.),
        },
    );
    commands.spawn((SpriteSheetBundle { // deck icon
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 10_usize,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            color: Color::rgb(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    },
    Animator::new(tween),
    Pleromic{ pleroma: true, dist: 1110.},
    Active{},
    ));
    commands.spawn((SpriteSheetBundle { // world icon
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 8_usize,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            color: Color::rgb(0.0, 0.0, 0.0),
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_z(PI/4.0),
            ..default()
        },
        ..default()
    },
    Pleromic{ pleroma: true, dist: 1110.},
    Active{},
    Animator::new(tween_bal),
    ));
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 11_usize,
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3{ x: -10., y: -500., z: 0.0},
            ..default()
        },
        ..default()
    },
    SwapSpace{},
    ));
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 11_usize,
            custom_size: Some(Vec2::new(64.0, 64.0)),
            color: Color::rgb(0.0, 0.0, 0.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(120.0, -2000.0, 0.),
            ..default()
        },
        ..default()
    },
    SwapSpace{},
    ));
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };
    commands.spawn(
        (
            Text2dBundle {
                text: Text::from_section("5", text_style.clone())
                .with_alignment(text_alignment),
                transform: Transform {
                    translation: Vec3{ x: -40., y: -500., z: 0.0},
                    ..default()
                },
            ..default()
            },
            FifthMarker{},
            Pleromic{ pleroma: false, dist: 100.},
            Active{},
        )
    );
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };
    commands.spawn(
        (
            Text2dBundle {
                text: Text::from_section("5", text_style.clone())
                .with_alignment(text_alignment),
                transform: Transform {
                    translation: Vec3{ x: -40., y: -2000., z: 0.0},
                    ..default()
                },
            ..default()
            },
            FifthMarker{},
            Pleromic{ pleroma: true, dist: 100.},
        )
    );

}

fn move_text_labels(
    query_world: Query<&WorldManager>,
    mut query: Query<(Entity, &mut TextLabel, &Pleromic, &Transform)>,
    query_swap: Query<(Entity, &Transform), With<SwapSpace>>,
    query_swap_text: Query<(Entity, &Transform), With<FifthMarker>>,
    mut commands: Commands,
){
    if unsafe {
        WORLD_PHASE != 1 && WORLD_PHASE != 5
    }{
        return;
    }
    if (unsafe { WORLD_PHASE } == 1){
        let mut pleroma = true;
        for world in query_world.iter(){
            if world.kenoma { pleroma = false};
        }
        for (entity_id, text, plero, trans) in query.iter_mut() {
            let text_num = text.number;
            let bump_y = if plero.pleroma { -1500.} else {0.};
            let bump_x = if pleroma { 160.} else {-40.};
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: trans.translation,
                    end: Vec3::new(bump_x, bump_y+220.-120.*text_num as f32, 0.),
                },
            ).with_completed(|_entity, _tween|{world_phase_update(3)});
            commands.entity(entity_id).insert(Animator::new(tween));
            world_phase_update(2);
    
        }
        for (entity_id, transform) in query_swap.iter(){
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(transform.translation.x, transform.translation.y + 290., transform.translation.z),
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        for (entity_id, trans) in query_swap_text.iter(){
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: trans.translation,
                    end: Vec3::new(trans.translation.x, trans.translation.y+250., trans.translation.z),
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }
    else if (unsafe { WORLD_PHASE } == 5){
        let mut pleroma = true;
        for world in query_world.iter(){
            if world.kenoma { pleroma = false};
        }
        for (entity_id, text,plero, trans) in query.iter_mut() {
            let text_num = text.number;
            let bump_y = if plero.pleroma { -1500.} else {0.};
            let bump_x = if pleroma { 675.} else {0.};
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    end: Vec3::new(bump_x+-400.+80.*text_num as f32, bump_y+-300., 0.),
                    start: trans.translation,
                },
            ).with_completed(|_entity, _tween|{world_phase_update(0)});
            commands.entity(entity_id).insert(Animator::new(tween));
            world_phase_update(6);
        }
        for (entity_id, transform) in query_swap.iter(){
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(transform.translation.x, transform.translation.y - 290., transform.translation.z),
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        for (entity_id, trans) in query_swap_text.iter(){
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: trans.translation,
                    end: Vec3::new(trans.translation.x, trans.translation.y-250., trans.translation.z),
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }


}

fn world_phase_update(new_phase: i8){
    unsafe {
        WORLD_PHASE = new_phase;
    }
}

fn swap_pleroma_kenoma(
    mut query_world: Query<&mut WorldManager>,
    mut query_pleroma: Query<(Entity, &Transform, &Pleromic), With<Pleromic>>,
    mut commands: Commands,
){
    if unsafe {
        WORLD_PHASE != 8
    }{
        return;
    }
    let mut current_dim = true;
    for mut world in query_world.iter_mut(){
        if world.kenoma {current_dim = false};
        world.kenoma = !world.kenoma;
    }
    if !current_dim{
        for (entity_id, trans, item) in query_pleroma.iter_mut() {
            let start_vec = trans.translation;
            let dist = item.dist;
            let end_vec = Vec3::new(start_vec.x+dist, start_vec.y, start_vec.z);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: start_vec,
                    end: end_vec,
                },
            ).with_completed(|_entity, _tween|{world_phase_update(4)});
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }
    else {
        for (entity_id, trans, item) in query_pleroma.iter_mut() {
            let start_vec = trans.translation;
            let dist = item.dist;
            let end_vec = Vec3::new(start_vec.x-dist, start_vec.y, start_vec.z);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: start_vec,
                    end: end_vec,
                },
            ).with_completed(|_entity, _tween|{world_phase_update(4)});
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }
    world_phase_update(-3);
}

fn claim_balanced_worlds(
    mut query: Query<(Entity, &mut PolarityMarker, &Transform)>,
    query_card: Query<&Card>,
    query_world: Query<&WorldManager>,
    mut query_worlds: Query<(Entity, &mut Dimension)>,
    mut query_w_deck: Query<&mut BalancedWorlds>,
    mut query_text_deck: Query<&mut Text, With<BalancedWorlds>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    if unsafe {
        WORLD_PHASE != 7
    }{
        return;
    }
    let mut card_offset = 0;
    for card in query_card.iter() {
        if !card.active{
            continue;
        }
        card_offset = card.value;
    }
    for world in query_world.iter(){
        if world.kenoma {card_offset = -card_offset};
    }
    let mut balanced_worlds: Vec<u8> = Vec::new();
    for (_entity_id, pol , trans) in query.iter(){
        if pol.polarity == 0 && trans.translation.y > -1000.{
            balanced_worlds.push(pol.world);
        }
    }
    assert!(balanced_worlds.len() < 5);
    for (entity_id, world) in query_worlds.iter_mut() {
        let world_num = world.world;
        if !balanced_worlds.contains(&world_num){
            continue;
        }
        let start_vec = if world.pleroma{
            Vec3{ x: 120.0, y: -1500.+260.0-(world_num as f32 * 120.0), z: 0.0}
        } else {
            Vec3{ x: 0.0, y: 260.0-(world_num as f32 * 120.0), z: 0.0}
        };
        let end_vec = if world.pleroma{
            Vec3{ x: 120.0, y: -1000.+260.0-(world_num as f32 * 120.0), z: 0.0}
        } else {
            Vec3{ x: 0.0, y: 760.0-(world_num as f32 * 120.0), z: 0.0}
        };
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(300),
            TransformPositionLens {
                start: start_vec,
                end: end_vec,
            },
        );
        commands.entity(entity_id).insert(Animator::new(tween));
        commands.entity(entity_id).remove::<Dimension>();
        let possible_worlds = ["Goemorphos"];
        let new_world_name: Vec<_> = possible_worlds
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
        let new_world_name = *new_world_name[0];
        let img_path = "spritesheet.png".to_owned();
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let end_vec = if world.pleroma{
            Vec3{ x: 120.0, y: -1500.+260.0-(world_num as f32 * 120.0), z: 0.0}
        } else {
            Vec3{ x: 0.0, y: 260.0-(world_num as f32 * 120.0), z: 0.0}
        };
        let start_vec = if world.pleroma{
            Vec3{ x: 120.0, y: -1000.+260.0-(world_num as f32 * 120.0), z: 0.0}
        } else {
            Vec3{ x: 0.0, y: 760.0-(world_num as f32 * 120.0), z: 0.0}
        };
        let color = if world.pleroma{
            Color::rgb(0.0, 0.0, 0.0)
        } else {
            Color::rgb(1.0, 1.0, 1.0)
        };
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(1300),
            TransformPositionLens {
                start: start_vec,
                end: end_vec,
            },
        ).with_completed(|_entity, _tween|{world_phase_update(8)});
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : 8, // replace this with the world texture
                custom_size: Some(Vec2::new(64.0, 64.0)),
                color,
                ..default()
            },
            transform: Transform {
                translation: start_vec,
                rotation: Quat::from_rotation_z(PI/4.0),
                ..default()
            },
            ..default()
        },
        Dimension{
            name: new_world_name.to_owned(),
            world: world_num,
            pleroma: world.pleroma,
        },
        Animator::new(tween),
        ));
    }
    for (entity_id, mut pol, trans) in query.iter_mut(){
        if !balanced_worlds.contains(&pol.world){
            continue;
        }
        assert!(pol.polarity == 0);
        pol.polarity = card_offset;
        assert!(pol.polarity != 0);
        let end_x;
        let end_vector: Vec3;
        if pol.polarity > 0{
            end_x = 190. + (pol.polarity-2) as f32*80.;
        }
        else {
            end_x = -110. + (pol.polarity+1) as f32*80.;
        }
        if pol.dimension{
            end_vector = Vec3::new(end_x+120., -1500.+260.0-(pol.world as f32 * 120.0), 0.);
        }
        else {
            end_vector = Vec3::new(end_x, 260.0-(pol.world as f32 * 120.0), 0.);
        }
        let tween = Tween::new(
            EaseFunction::BackInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: trans.translation,
                end: end_vector
            },
        );
        commands.entity(entity_id).insert(Animator::new(tween));

    }
    let mut cap = 0;
    for mut deck in query_w_deck.iter_mut(){
        deck.capacity += balanced_worlds.len() as u16;
        cap = deck.capacity;
    }
    for mut text in query_text_deck.iter_mut(){
        text.sections[0].value = cap.to_string();
    }
    world_phase_update(-6);
    if balanced_worlds.is_empty(){ world_phase_update(8)};
}

fn banish_and_replace(
    mut query: Query<(Entity, &mut Card, &Pleromic, &Transform)>,
    mut query_text_deck: Query<&mut Text, With<Deck>>,
    query_world: Query<&WorldManager>,
    mut query_deck: Query<&mut Deck>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>
){
    if unsafe {
        WORLD_PHASE != 4
    }{
        return;
    }
    let mut pleroma = true;
    let mut scan_count = 8;
    for world in query_world.iter(){
        if world.kenoma {pleroma = false};
    }
    let card_value = rand::thread_rng().gen_range(1..7);
    for (entity_id, card, plero, trans) in query.iter_mut() {
        scan_count-=1;
        if !card.active{
            continue;
        }
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(300),
            TransformPositionLens {
                start: trans.translation,
                end: Vec3::new(trans.translation.x, trans.translation.y-230., 0.),
            },
        );
        commands.entity(entity_id).insert(Animator::new(tween));
        commands.entity(entity_id).remove::<Card>();
        let img_path = "spritesheet.png".to_owned();
        let texture_handle = asset_server.load(&img_path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            80, 2, None, None
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let start_vec = if plero.pleroma && pleroma{
            Vec3::new(275.+80.*card.position as f32, -1500.-400.,0. )
        } else if pleroma && !plero.pleroma{
            Vec3::new(275.+80.*card.position as f32, -400.,0. )
        } else if !pleroma && plero.pleroma {
            Vec3::new(-400.+80.*card.position as f32, -1500.-400.,0. )
        } else {
            Vec3::new(-400.+80.*card.position as f32, -400.,0. )
        };
        let end_vec = if plero.pleroma && pleroma{
            Vec3::new(275.+80.*card.position as f32, -1500.-250.,0. )
        } else if pleroma && !plero.pleroma{
            Vec3::new(275.+80.*card.position as f32, -250.,0. )
        } else if !pleroma && plero.pleroma {
            Vec3::new(-400.+80.*card.position as f32, -1500.-250.,0. )
        } else {
            Vec3::new(-400.+80.*card.position as f32, -250.,0. )
        };
        let color = if plero.pleroma{
            Color::rgb(0.0, 0.0, 0.0)
        } else {
            Color::rgb(1.0, 1.0, 1.0)
        };
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: start_vec,
                end: end_vec,
            },
        );
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite{
                index : (card_value-1) as usize,
                custom_size: Some(Vec2::new(64.0, 64.0)),
                color,
                ..default()
            },
            transform: Transform { translation: Vec3 { x: -400.+80.*card.position as f32, y: -400., z: 0. }, ..Default::default()},
            ..default()
        },
        Card{
            value: card_value,
            position: card.position,
            active: false,
        },
        Pleromic{
            pleroma: plero.pleroma,
            dist: 675.
        },
        Animator::new(tween),
        )); 
    }
    let mut cap = 64;
    for mut deck in query_deck.iter_mut(){
        deck.capacity -= 1;
        cap = deck.capacity;
    }
    for mut text in query_text_deck.iter_mut(){
        text.sections[0].value = cap.to_string();
    }
    if scan_count == 0{
        world_phase_update(5);
    } 
}

fn push_world_polarity(
    mut query: Query<(Entity, &mut PolarityMarker)>,
    query_world: Query<&WorldManager>,
    mut query_cards: Query<&mut Card>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
){
    if unsafe { WORLD_PHASE } == 3{
        let mut pleroma = false;
        for wor in query_world.iter(){
            if !wor.kenoma{
                pleroma = true;
            }
        }
        let mut value_incoming = 0;
        for card in query_cards.iter_mut() {
            if card.active{ value_incoming = card.value}
        }
        if pleroma {value_incoming = -value_incoming};
        if value_incoming == 0{ return;}
        for (entity_id, mut pol) in query.iter_mut() {
            if input.just_released(KeyCode::Key5){
                world_phase_update(7);
            }
            else if input.just_released(KeyCode::Key1)
            || input.just_released(KeyCode::Key2)
            || input.just_released(KeyCode::Key3)
            || input.just_released(KeyCode::Key4) {
                if input.just_released(KeyCode::Key1) && pol.world != 0
                || input.just_released(KeyCode::Key2) && pol.world != 1
                || input.just_released(KeyCode::Key3) && pol.world != 2
                || input.just_released(KeyCode::Key4) && pol.world != 3
                {
                    continue;
                }
                let current_pol = pol.polarity;
                pol.polarity -= value_incoming;
                if pol.polarity > 6{
                    pol.polarity = 6;
                }
                else if pol.polarity < -6{
                    pol.polarity = -6;
                }
                let start_vector: Vec3;
                let end_vector: Vec3;
                let mut start_x = 0.;
                let mut end_x = start_x;
                if current_pol != 0{
                    if current_pol > 0{
                        start_x = 190.+ (current_pol-2) as f32*80.;
                    }
                    else {
                        start_x = -110. + (current_pol+1) as f32*80.;
                    }
                }
                if pol.polarity != 0{
                    if pol.polarity > 0{
                        end_x = 190. + (pol.polarity-2) as f32*80.;
                    }
                    else {
                        end_x = -110. + (pol.polarity+1) as f32*80.;
                    }
                }
                if pol.dimension{
                    start_vector = Vec3::new(start_x+120., -1500.+260.0-(pol.world as f32 * 120.0), 0.);
                    end_vector = Vec3::new(end_x+120., -1500.+260.0-(pol.world as f32 * 120.0), 0.);
                }
                else {
                    start_vector = Vec3::new(start_x, 260.0-(pol.world as f32 * 120.0), 0.);
                    end_vector = Vec3::new(end_x, 260.0-(pol.world as f32 * 120.0), 0.);
                }
                let tween = Tween::new(
                    EaseFunction::BackInOut,
                    Duration::from_secs(1),
                    TransformPositionLens {
                        start: start_vector,
                        end: end_vector
                    },
                );
                commands.entity(entity_id).insert(Animator::new(tween));
                unsafe{ WORLD_PHASE = 4};
            }
        }
    }
}

fn select_card(
    mut query: Query<(Entity, &mut Card, &Transform)>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
) {
    if unsafe { WORLD_PHASE } == 0{
        for (entity_id, mut card, trans) in query.iter_mut() {
            let card_num = card.position;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: trans.translation,
                    end: Vec3::new(trans.translation.x, trans.translation.y+30., 0.),
                },
            ).with_completed(|_entity, _tween|{world_phase_update(1)});
            if input.just_released(KeyCode::Key1)
            || input.just_released(KeyCode::Key2)
            || input.just_released(KeyCode::Key3)
            || input.just_released(KeyCode::Key4) {
                if input.just_released(KeyCode::Key1) && card_num != 0
                || input.just_released(KeyCode::Key2) && card_num != 1
                || input.just_released(KeyCode::Key3) && card_num != 2
                || input.just_released(KeyCode::Key4) && card_num != 3
                {
                    continue;
                }
                commands.entity(entity_id).insert(Animator::new(tween));
                card.active = true;
            }
        }
        if input.just_released(KeyCode::Key1)
        || input.just_released(KeyCode::Key2)
        || input.just_released(KeyCode::Key3)
        || input.just_released(KeyCode::Key4) {
        unsafe { WORLD_PHASE = -4 };
        }
    }
}

