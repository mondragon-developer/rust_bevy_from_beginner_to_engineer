//! Every measurement and tuning value in the game, in one place.
//! Drawing, physics, and scoring all read the same blueprint.

use bevy::prelude::*;

// ---------- The court, in numbers ----------

// Fixed play area so the whole court stays visible at any window/canvas size.
pub const WORLD_W: f32 = 1280.0;
pub const WORLD_H: f32 = 720.0;

pub const BALL_R: f32 = 26.0;
pub const GROUND_Y: f32 = -320.0;
// The ball starts at the free-throw spot, resting on the floor.
pub const START: Vec2 = Vec2::new(-420.0, GROUND_Y + BALL_R);

pub const BACKBOARD_X: f32 = 470.0;
pub const BACKBOARD_Y: f32 = 130.0;
pub const BACKBOARD_W: f32 = 16.0;
pub const BACKBOARD_H: f32 = 150.0;
pub const BACKBOARD_FRONT: f32 = BACKBOARD_X - BACKBOARD_W / 2.0;

pub const RIM_Y: f32 = 70.0;
pub const RIM_FRONT_X: f32 = 350.0;
pub const RIM_BACK_X: f32 = BACKBOARD_FRONT;

// ---------- Game feel — tweak these to change difficulty ----------

pub const GRAVITY: f32 = -1300.0; // downward acceleration in px/s^2
pub const CHARGE_TIME: f32 = 1.2; // seconds of holding to reach full power
pub const MIN_SHOT_SPEED: f32 = 500.0; // launch speed at zero charge
pub const MAX_SHOT_SPEED: f32 = 2200.0; // launch speed at full charge
pub const RESTITUTION: f32 = 0.6; // fraction of speed kept after a bounce
pub const GROUND_FRICTION: f32 = 0.75; // horizontal loss on each hard floor bounce
pub const ROLL_FRICTION: f32 = 2.5; // per-second slowdown while rolling on the floor
pub const BOUNCE_THRESHOLD: f32 = 160.0; // |vy| above this = real bounce, below = rest/roll
pub const STOP_SPEED: f32 = 30.0; // ball fully stops below this horizontal speed
