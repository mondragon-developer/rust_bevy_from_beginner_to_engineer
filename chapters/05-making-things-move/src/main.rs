use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Making Things Move".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_ball)
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

/// Runs every frame: slide the ball back and forth along the floor,
/// rolling it in the direction it travels.
fn move_ball(time: Res<Time>, mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in &mut query {
        // A smooth wave between -400 and +400 as time passes.
        let x = (time.elapsed_secs() * 0.8).sin() * 400.0;

        // How far we moved this frame decides how much the ball rolls.
        let dx = x - transform.translation.x;
        transform.translation.x = x;
        transform.rotate_z(-dx * 0.02);
    }
}
