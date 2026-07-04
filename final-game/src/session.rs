//! Session state and its sync with the HTML control panel: inbound at the
//! start of the frame, outbound at the end.

use bevy::prelude::*;

use crate::bridge;
use crate::components::Ball;
use crate::resources::{Aim, Attempts, Score, ScoreFlash, ShotLimit, Stopped};
use crate::shooting::reset;
use crate::GameSet;

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<Attempts>()
            .init_resource::<Stopped>()
            .init_resource::<ShotLimit>()
            .add_systems(Update, sync_from_js.in_set(GameSet::SyncIn))
            .add_systems(Update, sync_to_js.in_set(GameSet::SyncOut));
    }
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
    let new_limit = bridge::shot_limit();
    if limit.0 != new_limit {
        limit.0 = new_limit;
    }

    if bridge::take_reset() {
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
    bridge::publish(score.0, attempts.0, stopped.0);
}
