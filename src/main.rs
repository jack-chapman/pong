use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "PONG".to_string(),
            width: 1200.,
            height: 800.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Scoreboard {
            left_score: 0,
            right_score: 0,
        })
        .add_startup_system(setup.system())
        .run()
}

enum PlayerSide {
    Left,
    Right,
}

struct Paddle {
    side: PlayerSide,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    left_score: usize,
    right_score: usize,
}

enum Collider {
    Solid,
    Scoreable,
    Paddle,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // add cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // scoreboard
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Relative,
                    position: Rect {
                        top: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "0 - 0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });

    // paddles/players
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(-400.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .insert(Paddle {
            side: PlayerSide::Left,
        })
        .insert(Collider::Paddle);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(400.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 100.0)),
            ..Default::default()
        })
        .insert(Paddle {
            side: PlayerSide::Right,
        })
        .insert(Collider::Paddle);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });

    // walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Scoreable);

    // right wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Scoreable);
    // top wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);

    // bottom wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Collider::Solid);
}
