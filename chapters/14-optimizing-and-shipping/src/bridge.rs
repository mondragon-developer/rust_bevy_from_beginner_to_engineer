//! Bridge to the HTML control panel. The two sides share a single
//! `window.rustbyve` object: Rust publishes score/attempts/game-over out; the
//! panel writes the shot limit and a one-shot reset request back in. On
//! non-wasm builds these are no-ops so the crate still builds on the host.

#[cfg(target_arch = "wasm32")]
mod js {
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
pub fn take_reset() -> bool {
    js::rb_take_reset()
}
#[cfg(target_arch = "wasm32")]
pub fn shot_limit() -> u32 {
    js::rb_shot_limit().max(0) as u32
}
#[cfg(target_arch = "wasm32")]
pub fn publish(score: u32, attempts: u32, game_over: bool) {
    js::rb_publish(score as i32, attempts as i32, game_over);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn take_reset() -> bool {
    false
}
#[cfg(not(target_arch = "wasm32"))]
pub fn shot_limit() -> u32 {
    0
}
#[cfg(not(target_arch = "wasm32"))]
pub fn publish(_score: u32, _attempts: u32, _game_over: bool) {}
