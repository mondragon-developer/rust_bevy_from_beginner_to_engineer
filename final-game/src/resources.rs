//! Resources: global game state shared by all systems.

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

// Total shots taken this game (made or missed).
#[derive(Resource, Default)]
pub struct Attempts(pub u32);

// True once the optional shot limit is reached: shooting is frozen until reset.
#[derive(Resource, Default)]
pub struct Stopped(pub bool);

// Max shots before the game stops, mirrored from the HTML panel. 0 = unlimited.
#[derive(Resource, Default)]
pub struct ShotLimit(pub u32);

// Seconds of "swish" feedback remaining after a made basket.
#[derive(Resource, Default)]
pub struct ScoreFlash(pub f32);

// While aiming: how long the mouse has been held (the charge), capped at CHARGE_TIME.
#[derive(Resource, Default)]
pub struct Aim {
    pub active: bool,
    pub charge: f32,
}
