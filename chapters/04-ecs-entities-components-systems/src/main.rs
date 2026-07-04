use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ECS: A Ball on the Court".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

/// Marker component: "this entity is the ball".
#[derive(Component)]
struct Ball;

fn setup(mut commands: Commands) {
    // Without a camera, nothing gets drawn.
    commands.spawn(Camera2d);

    // The floor: a long, dark strip near the bottom of the screen.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.20, 0.25), Vec2::new(1280.0, 40.0)),
        Transform::from_xyz(0.0, -320.0, 0.0),
    ));

    // The ball: an orange square for now (it becomes a circle later).
    commands.spawn((
        Ball,
        Sprite::from_color(Color::srgb(0.96, 0.55, 0.13), Vec2::splat(50.0)),
        Transform::from_xyz(0.0, -275.0, 1.0),
    ));
}
