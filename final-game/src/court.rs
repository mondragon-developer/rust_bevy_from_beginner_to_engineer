//! The stage: camera, court geometry, and the HUD text entities.

use bevy::{prelude::*, render::camera::ScalingMode};

use crate::components::{Ball, BallState, ScoreText};
use crate::constants::*;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.12)))
            .add_systems(Startup, setup);
    }
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

    // The HUD: score in the top-left corner (screen-space UI, not world-space).
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

    // How-to-play line at the bottom of the screen.
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
