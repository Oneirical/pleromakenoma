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
        .add_event::<CardPlacedEvent>()
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
struct Deck{
    capacity: u16
}

#[derive(Event)]
struct CardPlacedEvent(Card); 

static mut WORLD_PHASE: i8 = 0;

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
    let img_path = "spritesheet.png".to_owned();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        80, 2, None, None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for i in 0..4_u8{
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
        commands.spawn(
            (
                Text2dBundle {
                    text: Text::from_section(bindings[i as usize], text_style.clone())
                    .with_alignment(text_alignment),
                ..default()
                },
                Animator::new(tween_text),
                TextLabel{
                    number: i,
                }
            )
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
    commands.spawn(
        (
            Text2dBundle {
                text: Text::from_section("64", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
            },
            Animator::new(tween_deck),
            Deck{
                capacity: 64,
            }
        )
    );
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 10_usize,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        ..default()
    },
    Animator::new(tween),
    ));
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            index : 11_usize,
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3{ x: 0., y: -500., z: 0.0},
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
        )
    );

}

fn move_text_labels(
    mut query: Query<(Entity, &mut TextLabel)>,
    query_swap: Query<(Entity, &Transform), With<SwapSpace>>,
    query_swap_text: Query<Entity, With<FifthMarker>>,
    mut commands: Commands,
){
    if unsafe {
        WORLD_PHASE != 1 && WORLD_PHASE != 5
    }{
        return;
    }
    if (unsafe { WORLD_PHASE } == 1){
        for (entity_id, text) in query.iter_mut() {
            let text_num = text.number;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: Vec3::new(-400.+80.*text_num as f32, -300., 0.),
                    end: Vec3::new(-40., 220.-120.*text_num as f32, 0.),
                },
            ).with_completed(|_entity, _tween|{world_phase_update(3)});
            commands.entity(entity_id).insert(Animator::new(tween));
            world_phase_update(2);
    
        }
        for (entity_id, transform) in query_swap.iter(){
            let start_vec = if transform.translation.y < -1500.{
                Vec3::new(120., -2000., 0.)
            }
            else{
                Vec3::new(0., -500., 0.)
            };
            let end_vec = if transform.translation.y < -1500.{
                Vec3::new(120., -1710., 0.)
            }
            else{
                Vec3::new(0., -210., 0.)
            };
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: start_vec,
                    end: end_vec,
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
        for entity_id in query_swap_text.iter(){
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: Vec3::new(-40., -500., 0.),
                    end: Vec3::new(-40., -250., 0.),
                },
            );
            commands.entity(entity_id).insert(Animator::new(tween));
        }
    }
    else if (unsafe { WORLD_PHASE } == 5){
        for (entity_id, text) in query.iter_mut() {
            let text_num = text.number;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs(1),
                TransformPositionLens {
                    end: Vec3::new(-400.+80.*text_num as f32, -300., 0.),
                    start: Vec3::new(-40., 220.-120.*text_num as f32, 0.),
                },
            ).with_completed(|_entity, _tween|{world_phase_update(0)});
            commands.entity(entity_id).insert(Animator::new(tween));
            world_phase_update(6);
        }
    }


}

fn world_phase_update(new_phase: i8){
    unsafe {
        WORLD_PHASE = new_phase;
    }
}

fn banish_and_replace(
    mut query: Query<(Entity, &mut Card)>,
    mut query_text_deck: Query<&mut Text, With<Deck>>,
    mut query_deck: Query<&mut Deck>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>
){
    if unsafe {
        WORLD_PHASE != 4
    }{
        return;
    }
    for (entity_id, card) in query.iter_mut() {
        if !card.active{
            continue;
        }
        let card_num = card.position;
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(300),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*card_num as f32, y: -220., z: 0. },
                end: Vec3::new(-400.+80.*card_num as f32, -400., 0.),
            },
        );
        commands.entity(entity_id).insert(Animator::new(tween));
        commands.entity(entity_id).remove::<Card>();
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
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3 { x: -400.+80.*card.position as f32, y: -400., z: 0. },
                end: Vec3::new(-400.+80.*card.position as f32, -250., 0.),
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
            position: card.position,
            active: false,
        },
        Animator::new(tween),
    ));
        world_phase_update(5);
    }
    let mut cap = 64;
    for mut deck in query_deck.iter_mut(){
        deck.capacity -= 1;
        cap = deck.capacity;
    }
    for mut text in query_text_deck.iter_mut(){
        text.sections[0].value = cap.to_string();
    }
}

fn push_world_polarity(
    mut query: Query<(Entity, &mut PolarityMarker)>,
    mut query_cards: Query<&mut Card>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
){
    if unsafe { WORLD_PHASE } == 3{
        let mut value_incoming = 0;
        for card in query_cards.iter_mut() {
            if card.active{ value_incoming = card.value}
        }
        if value_incoming == 0{ return;}
        for (entity_id, mut pol) in query.iter_mut() {
            if input.just_released(KeyCode::Key1)
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
    mut query: Query<(Entity, &mut Card)>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
) {
    if unsafe { WORLD_PHASE } == 0{
        for (entity_id, mut card) in query.iter_mut() {
            let card_num = card.position;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: Vec3 { x: -400.+80.*card_num as f32, y: -250., z: 0. },
                    end: Vec3::new(-400.+80.*card_num as f32, -220., 0.),
                },
            );
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
                unsafe { WORLD_PHASE = 1 };
            }
        }
    }
}

