//! Everything the player sees react: the net, the swish burst, the HUD.

use bevy::prelude::*;

use crate::components::ScoreText;
use crate::constants::*;
use crate::resources::{Attempts, Score, ScoreFlash, ShotLimit, Stopped};

pub struct FeedbackPlugin;

impl Plugin for FeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreFlash>()
            .add_systems(Update, (draw_scene, update_score_text, tick_flash));
    }
}

/// The net (green while celebrating) and the expanding swish burst.
/// Gizmos are redrawn from scratch every frame, so this runs in Update.
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
    // One horizontal strand so the net reads as woven, not just lines.
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

/// Count the celebration down to zero.
fn tick_flash(time: Res<Time>, mut flash: ResMut<ScoreFlash>) {
    if flash.0 > 0.0 {
        flash.0 = (flash.0 - time.delta_secs()).max(0.0);
    }
}

/// Rewrite the HUD only on frames where the session state actually changed.
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
