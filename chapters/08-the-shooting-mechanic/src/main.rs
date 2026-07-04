use bevy::{prelude::*, render::camera::ScalingMode};

// ---------- The court, in numbers ----------

// Fixed play area so the whole court stays visible at any window/canvas size.
const WORLD_W: f32 = 1280.0;
const WORLD_H: f32 = 720.0;

const BALL_R: f32 = 26.0;
const GROUND_Y: f32 = -320.0;
// The ball starts at the free-throw spot, resting on the floor.
const START: Vec2 = Vec2::new(-420.0, GROUND_Y + BALL_R);

const BACKBOARD_X: f32 = 470.0;
const BACKBOARD_Y: f32 = 130.0;
const BACKBOARD_W: f32 = 16.0;
const BACKBOARD_H: f32 = 150.0;
const BACKBOARD_FRONT: f32 = BACKBOARD_X - BACKBOARD_W / 2.0;

const RIM_Y: f32 = 70.0;
const RIM_FRONT_X: f32 = 350.0;
const RIM_BACK_X: f32 = BACKBOARD_FRONT;

// ---------- Game feel — tweak these to change difficulty ----------

const GRAVITY: f32 = -1300.0; // downward acceleration in px/s^2
const CHARGE_TIME: f32 = 1.2; // seconds of holding to reach full power
const MIN_SHOT_SPEED: f32 = 500.0; // launch speed at zero charge
const MAX_SHOT_SPEED: f32 = 2200.0; // launch speed at full charge

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Shooting Mechanic".into(),
                resolution: (WORLD_W, WORLD_H).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.12)))
        .init_resource::<Aim>()
        .add_systems(Startup, setup)
        .add_systems(Update, (aim_and_launch, physics).chain())
        .add_systems(Update, draw_net)
        .run();
}

/// The ball is either resting (shootable) or in the air.
#[derive(PartialEq)]
enum BallState {
    Idle,
    Flying,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    state: BallState,
    // Position before this frame's move — scoring will need it in Chapter 10.
    prev_pos: Vec2,
}

// While aiming: how long the mouse has been held (the charge), capped at CHARGE_TIME.
#[derive(Resource, Default)]
struct Aim {
    active: bool,
    charge: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // The camera scales so the full court is always in view, whatever
    // size the window (or browser canvas) becomes.
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WORLD_W,
                min_height: WORLD_H,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // The ball, now carrying its own physics data.
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_R))),
        MeshMaterial2d(materials.add(Color::srgb(0.95, 0.5, 0.2))),
        Transform::from_translation(START.extend(1.0)),
        Ball {
            velocity: Vec2::ZERO,
            state: BallState::Idle,
            prev_pos: START,
        },
    ));

    // The floor: extra wide so it never shows an edge on wide screens.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::new(WORLD_W * 2.0, 60.0)),
        Transform::from_xyz(0.0, GROUND_Y - 30.0, -1.0),
    ));

    // Support pole behind the hoop.
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.4, 0.42, 0.48),
            Vec2::new(12.0, BACKBOARD_Y - GROUND_Y),
        ),
        Transform::from_xyz(BACKBOARD_X + 16.0, (BACKBOARD_Y + GROUND_Y) / 2.0, -1.0),
    ));

    // The backboard.
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.9, 0.9, 0.95),
            Vec2::new(BACKBOARD_W, BACKBOARD_H),
        ),
        Transform::from_xyz(BACKBOARD_X, BACKBOARD_Y, 0.0),
    ));

    // Solid rim bar across the hoop opening (drawn behind the ball).
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.95, 0.4, 0.1),
            Vec2::new(RIM_BACK_X - RIM_FRONT_X, 7.0),
        ),
        Transform::from_xyz((RIM_FRONT_X + RIM_BACK_X) / 2.0, RIM_Y, 0.5),
    ));
}

/// Where is the mouse cursor, in world coordinates?
fn cursor_world(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let (camera, cam_tf) = cameras.single().ok()?;
    let cursor = window.cursor_position()?;
    camera.viewport_to_world_2d(cam_tf, cursor).ok()
}

// Launch goes from the ball toward the cursor; if the cursor is on the ball, default up-right.
fn aim_dir(ball: Vec2, cursor: Vec2) -> Vec2 {
    let d = cursor - ball;
    if d.length() < 1.0 {
        Vec2::new(0.7, 0.7).normalize()
    } else {
        d.normalize()
    }
}

fn aim_and_launch(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut aim: ResMut<Aim>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut gizmos: Gizmos,
) {
    let Ok((mut ball, mut tf)) = balls.single_mut() else {
        return;
    };

    // R re-spots the ball at the start line.
    if keys.just_pressed(KeyCode::KeyR) {
        reset(&mut ball, &mut tf);
        aim.active = false;
        aim.charge = 0.0;
        return;
    }

    let ball_pos = tf.translation.truncate();
    let Some(cursor) = cursor_world(&windows, &cameras) else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) && ball.state == BallState::Idle {
        aim.active = true;
        aim.charge = 0.0;
    }
    if !aim.active {
        return;
    }

    if mouse.pressed(MouseButton::Left) {
        aim.charge = (aim.charge + time.delta_secs()).min(CHARGE_TIME);
        let power = aim.charge / CHARGE_TIME;
        let speed = MIN_SHOT_SPEED + (MAX_SHOT_SPEED - MIN_SHOT_SPEED) * power;
        let launch = aim_dir(ball_pos, cursor) * speed;
        draw_power_bar(&mut gizmos, ball_pos, power);
        draw_trajectory(&mut gizmos, ball_pos, launch);
    }

    if mouse.just_released(MouseButton::Left) {
        let power = aim.charge / CHARGE_TIME;
        let speed = MIN_SHOT_SPEED + (MAX_SHOT_SPEED - MIN_SHOT_SPEED) * power;
        ball.velocity = aim_dir(ball_pos, cursor) * speed;
        ball.state = BallState::Flying;
        ball.prev_pos = ball_pos;
        aim.active = false;
        aim.charge = 0.0;
    }
}

// A power meter above the ball that fills and shifts green -> red as it charges.
fn draw_power_bar(gizmos: &mut Gizmos, ball_pos: Vec2, power: f32) {
    let w = 110.0;
    let base = ball_pos + Vec2::new(-w / 2.0, BALL_R + 22.0);
    let bg = Color::srgba(1.0, 1.0, 1.0, 0.25);
    let fill = Color::srgb(0.2 + 0.8 * power, 1.0 - 0.7 * power, 0.2);
    for o in 0..8 {
        let y = o as f32;
        gizmos.line_2d(base + Vec2::new(0.0, y), base + Vec2::new(w, y), bg);
        gizmos.line_2d(base + Vec2::new(0.0, y), base + Vec2::new(w * power, y), fill);
    }
}

// A small "+" so the predicted path reads as distinct dots, not a faint line.
fn dot(gizmos: &mut Gizmos, p: Vec2, color: Color) {
    gizmos.line_2d(p - Vec2::X * 3.5, p + Vec2::X * 3.5, color);
    gizmos.line_2d(p - Vec2::Y * 3.5, p + Vec2::Y * 3.5, color);
}

fn draw_trajectory(gizmos: &mut Gizmos, start: Vec2, vel: Vec2) {
    let dt = 1.0 / 60.0;
    let mut p = start;
    let mut v = vel;
    let color = Color::srgba(1.0, 0.95, 0.3, 0.95);
    for i in 0..150 {
        if i % 6 == 0 {
            dot(gizmos, p, color);
        }
        p += v * dt;
        v.y += GRAVITY * dt;
        if p.y < GROUND_Y + BALL_R {
            break;
        }
    }
}

/// Gravity pulls the velocity down; the velocity moves the ball.
fn physics(time: Res<Time>, mut balls: Query<(&mut Ball, &mut Transform)>) {
    let dt = time.delta_secs();
    for (mut ball, mut tf) in &mut balls {
        if ball.state != BallState::Flying {
            continue;
        }
        ball.prev_pos = tf.translation.truncate();
        ball.velocity.y += GRAVITY * dt;
        let step = ball.velocity * dt;
        tf.translation.x += step.x;
        tf.translation.y += step.y;
    }
}

fn reset(ball: &mut Ball, tf: &mut Transform) {
    ball.velocity = Vec2::ZERO;
    ball.state = BallState::Idle;
    ball.prev_pos = START;
    tf.translation = START.extend(1.0);
}

/// Gizmos are redrawn from scratch every frame, so this runs in Update.
fn draw_net(mut gizmos: Gizmos) {
    let orange = Color::srgb(0.95, 0.45, 0.15);
    let net = Color::srgba(0.85, 0.85, 0.9, 0.85);

    // Front rim nub so the front edge of the hoop opening is obvious.
    gizmos.line_2d(
        Vec2::new(RIM_FRONT_X, RIM_Y - 6.0),
        Vec2::new(RIM_FRONT_X, RIM_Y + 6.0),
        orange,
    );

    // Net: angled strands from the rim opening converging to a point below.
    let bottom = Vec2::new((RIM_FRONT_X + RIM_BACK_X) / 2.0, RIM_Y - 55.0);
    let segs = 6;
    for i in 0..=segs {
        let t = i as f32 / segs as f32;
        let top = Vec2::new(RIM_FRONT_X + (RIM_BACK_X - RIM_FRONT_X) * t, RIM_Y);
        gizmos.line_2d(top, top.lerp(bottom, 0.9), net);
    }
    // One horizontal strand so the net reads as woven, not just lines.
    gizmos.line_2d(
        Vec2::new(RIM_FRONT_X + 14.0, RIM_Y - 28.0),
        Vec2::new(RIM_BACK_X - 14.0, RIM_Y - 28.0),
        net,
    );
}
