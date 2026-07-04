//! Components: data attached to entities.

use bevy::prelude::*;

/// The ball is either resting (shootable) or in the air.
#[derive(PartialEq)]
pub enum BallState {
    Idle,
    Flying,
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub state: BallState,
    // Position before this frame's move, so scoring can detect the exact frame
    // the ball crosses down through the rim line.
    pub prev_pos: Vec2,
}

/// Marker for the HUD text entity so the feedback systems can find it.
#[derive(Component)]
pub struct ScoreText;
