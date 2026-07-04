use bevy::{prelude::*, render::camera::ScalingMode};

// Bridge to the HTML control panel. The two sides share a single `window.rustbyve`
// object: Rust publishes score/attempts/game-over out; the panel writes the shot
// limit and a one-shot reset request back in. On non-wasm builds these are no-ops
// so the crate still `cargo check`s on the host.
#[cfg(target_arch = "wasm32")]
mod bridge {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        function rb_ensure() {
          if (!window.rustbyve) {
            window.rustbyve = { resetRequested: false, shotLimit: 0, score: 0, attempts: 0, gameOver: false };
          }
          return window.rustbyve;
        }
        // Reads and clears the reset flag in one call so a click can't be missed or double-counted.
        export function rb_take_reset() {
          const s = rb_ensure();
          const v = s.resetRequested;
          s.resetRequested = false;
          return v;
        }
        export function rb_shot_limit() { return rb_ensure().shotLimit | 0; }
        export function rb_publish(score, attempts, gameOver) {
          const s = rb_ensure();
          s.score = score | 0;
          s.attempts = attempts | 0;
          s.gameOver = !!gameOver;
          if (typeof window.rbOnState === "function") window.rbOnState(s);
        }
    "#)]
    extern "C" {
        pub fn rb_take_reset() -> bool;
        pub fn rb_shot_limit() -> i32;
        pub fn rb_publish(score: i32, attempts: i32, game_over: bool);
    }
}

#[cfg(target_arch = "wasm32")]
fn js_take_reset() -> bool {
    bridge::rb_take_reset()
}
#[cfg(target_arch = "wasm32")]
fn js_shot_limit() -> u32 {
    bridge::rb_shot_limit().max(0) as u32
}
#[cfg(target_arch = "wasm32")]
fn js_publish(score: u32, attempts: u32, game_over: bool) {
    bridge::rb_publish(score as i32, attempts as i32, game_over);
}

#[cfg(not(target_arch = "wasm32"))]
fn js_take_reset() -> bool {
    false
}
#[cfg(not(target_arch = "wasm32"))]
fn js_shot_limit() -> u32 {
    0
}
#[cfg(not(target_arch = "wasm32"))]
fn js_publish(_score: u32, _attempts: u32, _game_over: bool) {}

// Fixed play area so the whole court stays visible at any browser/canvas size.
const WORLD_W: f32 = 1280.0;
const WORLD_H: f32 = 720.0;

// Game feel — tweak these to change difficulty.
const GRAVITY: f32 = -1300.0; // downward acceleration in px/s^2
const CHARGE_TIME: f32 = 1.2; // seconds of holding to reach full power
const MIN_SHOT_SPEED: f32 = 500.0; // launch speed at zero charge
const MAX_SHOT_SPEED: f32 = 2200.0; // launch speed at full charge
const RESTITUTION: f32 = 0.6; // fraction of speed kept after a bounce
const GROUND_FRICTION: f32 = 0.75; // horizontal loss on each hard floor bounce
const ROLL_FRICTION: f32 = 2.5; // per-second slowdown while rolling on the floor
const BOUNCE_THRESHOLD: f32 = 160.0; // |vy| above this = real bounce, below = rest/roll
const STOP_SPEED: f32 = 30.0; // ball fully stops below this horizontal speed
const BALL_R: f32 = 26.0;

const GROUND_Y: f32 = -320.0;
const START: Vec2 = Vec2::new(-420.0, GROUND_Y + BALL_R);

const BACKBOARD_X: f32 = 470.0;
const BACKBOARD_Y: f32 = 130.0;
const BACKBOARD_W: f32 = 16.0;
const BACKBOARD_H: f32 = 150.0;
const BACKBOARD_FRONT: f32 = BACKBOARD_X - BACKBOARD_W / 2.0;

const RIM_Y: f32 = 70.0;
const RIM_FRONT_X: f32 = 350.0;
const RIM_BACK_X: f32 = BACKBOARD_FRONT;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.12)))
        .init_resource::<Aim>()
        .init_resource::<Score>()
        .init_resource::<Attempts>()
        .init_resource::<Stopped>()
        .init_resource::<ShotLimit>()
        .init_resource::<ScoreFlash>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (sync_from_js, aim_and_launch, physics, collisions, sync_to_js).chain(),
        )
        .add_systems(Update, (draw_scene, update_score_text, tick_flash))
        .run();
}

#[derive(PartialEq)]
enum BallState {
    Idle,
    Flying,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    state: BallState,
    // Position before this frame's move, so scoring can detect the exact frame
    // the ball crosses down through the rim line.
    prev_pos: Vec2,
}

#[derive(Component)]
struct ScoreText;

#[derive(Resource, Default)]
struct Score(u32);

// Total shots taken this game (made or missed).
#[derive(Resource, Default)]
struct Attempts(u32);

// True once the optional shot limit is reached: shooting is frozen until reset.
#[derive(Resource, Default)]
struct Stopped(bool);

// Max shots before the game stops, mirrored from the HTML panel. 0 = unlimited.
#[derive(Resource, Default)]
struct ShotLimit(u32);

// Seconds of "swish" feedback remaining after a made basket.
#[derive(Resource, Default)]
struct ScoreFlash(f32);

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

    // Ball sits in front (z = 1) so it draws over the rim and backboard.
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

    commands.spawn((
        Text::new("Made: 0   Shots: 0"),
        TextFont {
            font_size: 34.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("Hold on the ball to charge power, aim with the mouse, release to shoot. R = reset ball."),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        },
    ));
}

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
    mut attempts: ResMut<Attempts>,
    mut stopped: ResMut<Stopped>,
    limit: Res<ShotLimit>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut gizmos: Gizmos,
) {
    let Ok((mut ball, mut tf)) = balls.single_mut() else {
        return;
    };

    // R re-spots the ball at the start line; it doesn't refund a shot or end the game.
    if keys.just_pressed(KeyCode::KeyR) {
        reset(&mut ball, &mut tf);
        aim.active = false;
        aim.charge = 0.0;
        return;
    }

    // Game over: the last shot can still finish flying, but no new charge starts.
    if stopped.0 {
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

        // Count the shot; if that hits the limit, this is the last one — let it fly,
        // then freeze new shots until the panel's Save & Reset starts a fresh game.
        attempts.0 += 1;
        if limit.0 > 0 && attempts.0 >= limit.0 {
            stopped.0 = true;
        }
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

fn collisions(
    time: Res<Time>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut score: ResMut<Score>,
    mut flash: ResMut<ScoreFlash>,
) {
    let dt = time.delta_secs();
    let half_w = WORLD_W / 2.0;
    let half_h = WORLD_H / 2.0;

    for (mut ball, mut tf) in &mut balls {
        if ball.state != BallState::Flying {
            continue;
        }
        let mut pos = tf.translation.truncate();

        // Bank off the front face of the backboard.
        if ball.velocity.x > 0.0
            && pos.x + BALL_R > BACKBOARD_FRONT
            && pos.x < BACKBOARD_X
            && pos.y < BACKBOARD_Y + BACKBOARD_H / 2.0
            && pos.y > BACKBOARD_Y - BACKBOARD_H / 2.0
        {
            pos.x = BACKBOARD_FRONT - BALL_R;
            ball.velocity.x = -ball.velocity.x * RESTITUTION;
        }

        // Bounce off the front rim lip on a near miss.
        let rim_point = Vec2::new(RIM_FRONT_X, RIM_Y);
        let to_ball = pos - rim_point;
        if to_ball.length() < BALL_R {
            let n = to_ball.normalize_or_zero();
            pos = rim_point + n * BALL_R;
            ball.velocity = reflect(ball.velocity, n) * RESTITUTION;
        }

        // Score: ball center drops through the opening. The ball is NOT reset — it
        // keeps falling through the net and bounces on, so the make is visible.
        if ball.prev_pos.y > RIM_Y
            && pos.y <= RIM_Y
            && ball.velocity.y < 0.0
            && pos.x > RIM_FRONT_X + 6.0
            && pos.x < RIM_BACK_X - 6.0
        {
            score.0 += 1;
            flash.0 = 0.7;
        }

        // Side walls keep the ball wandering inside the court.
        if pos.x - BALL_R < -half_w {
            pos.x = -half_w + BALL_R;
            ball.velocity.x = ball.velocity.x.abs() * RESTITUTION;
        }
        if pos.x + BALL_R > half_w {
            pos.x = half_w - BALL_R;
            ball.velocity.x = -ball.velocity.x.abs() * RESTITUTION;
        }
        // Ceiling.
        if pos.y + BALL_R > half_h {
            pos.y = half_h - BALL_R;
            ball.velocity.y = -ball.velocity.y.abs() * RESTITUTION;
        }

        // Floor: bounce while losing energy, then roll, then come to rest in place.
        if pos.y - BALL_R <= GROUND_Y {
            pos.y = GROUND_Y + BALL_R;
            if ball.velocity.y < -BOUNCE_THRESHOLD {
                ball.velocity.y = -ball.velocity.y * RESTITUTION;
                ball.velocity.x *= GROUND_FRICTION;
            } else {
                ball.velocity.y = 0.0;
                ball.velocity.x *= (1.0 - ROLL_FRICTION * dt).max(0.0);
                if ball.velocity.x.abs() < STOP_SPEED {
                    ball.velocity = Vec2::ZERO;
                    ball.state = BallState::Idle;
                }
            }
        }

        tf.translation.x = pos.x;
        tf.translation.y = pos.y;
    }
}

fn reflect(v: Vec2, n: Vec2) -> Vec2 {
    v - 2.0 * v.dot(n) * n
}

fn reset(ball: &mut Ball, tf: &mut Transform) {
    ball.velocity = Vec2::ZERO;
    ball.state = BallState::Idle;
    ball.prev_pos = START;
    tf.translation = START.extend(1.0);
}

// Pull the shot limit from the panel every frame, and when the panel's Save & Reset
// fires, wipe the score/attempts and re-spot the ball for a brand-new game.
fn sync_from_js(
    mut score: ResMut<Score>,
    mut attempts: ResMut<Attempts>,
    mut stopped: ResMut<Stopped>,
    mut limit: ResMut<ShotLimit>,
    mut aim: ResMut<Aim>,
    mut flash: ResMut<ScoreFlash>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
) {
    let new_limit = js_shot_limit();
    if limit.0 != new_limit {
        limit.0 = new_limit;
    }

    if js_take_reset() {
        score.0 = 0;
        attempts.0 = 0;
        stopped.0 = false;
        flash.0 = 0.0;
        aim.active = false;
        aim.charge = 0.0;
        if let Ok((mut ball, mut tf)) = balls.single_mut() {
            reset(&mut ball, &mut tf);
        }
    }
}

// Push the live game state to the panel so it can show progress and, on Save & Reset,
// read the final score/attempts for the record it stores.
fn sync_to_js(score: Res<Score>, attempts: Res<Attempts>, stopped: Res<Stopped>) {
    js_publish(score.0, attempts.0, stopped.0);
}

fn draw_scene(mut gizmos: Gizmos, flash: Res<ScoreFlash>) {
    let orange = Color::srgb(0.95, 0.45, 0.15);
    let scored = flash.0 > 0.0;
    let net = if scored {
        Color::srgb(0.3, 1.0, 0.45)
    } else {
        Color::srgba(0.85, 0.85, 0.9, 0.85)
    };

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
    gizmos.line_2d(
        Vec2::new(RIM_FRONT_X + 14.0, RIM_Y - 28.0),
        Vec2::new(RIM_BACK_X - 14.0, RIM_Y - 28.0),
        net,
    );

    // Swish burst: a ring of spokes that expands while the flash is active.
    if scored {
        let center = Vec2::new((RIM_FRONT_X + RIM_BACK_X) / 2.0, RIM_Y - 10.0);
        let r = 30.0 + (0.7 - flash.0) * 200.0;
        let green = Color::srgb(0.3, 1.0, 0.45);
        let spokes = 10;
        for k in 0..spokes {
            let a = k as f32 / spokes as f32 * std::f32::consts::TAU;
            let dir = Vec2::new(a.cos(), a.sin());
            gizmos.line_2d(center + dir * r, center + dir * (r + 16.0), green);
        }
    }
}

fn tick_flash(time: Res<Time>, mut flash: ResMut<ScoreFlash>) {
    if flash.0 > 0.0 {
        flash.0 = (flash.0 - time.delta_secs()).max(0.0);
    }
}

fn update_score_text(
    score: Res<Score>,
    attempts: Res<Attempts>,
    stopped: Res<Stopped>,
    limit: Res<ShotLimit>,
    mut q: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() && !attempts.is_changed() && !stopped.is_changed() && !limit.is_changed()
    {
        return;
    }
    let shots = if limit.0 > 0 {
        format!("{}/{}", attempts.0, limit.0)
    } else {
        format!("{}", attempts.0)
    };
    if let Ok(mut text) = q.single_mut() {
        text.0 = if stopped.0 {
            format!("Made: {}   Shots: {}     GAME OVER", score.0, shots)
        } else {
            format!("Made: {}   Shots: {}", score.0, shots)
        };
    }
}
