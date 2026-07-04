# Chapter 13 — Refactoring Like an Engineer

*Read this in: **English** | [Español](README.es.md)*

Your game works and it lives in one 600-line `main.rs`. Nothing is wrong with that — for a solo prototype. But open that file and try to answer "where is everything about scoring?" — it's smeared across constants at the top, a resource in the middle, a few lines inside `collisions`, and a text system at the bottom. In this chapter you take the game apart and reassemble it so every question like that has a one-word answer. **Not one line of behavior changes.** That's the definition of refactoring — and doing it without fear is the most engineer-flavored skill in this course.

**Time**: ~1.5 hours.

## The target structure

```
src/
├── main.rs         ← 50 lines: builds the App, wires the plugins
├── constants.rs    ← every number in the game
├── components.rs   ← Ball, BallState, ScoreText
├── resources.rs    ← Score, Attempts, Stopped, ShotLimit, ScoreFlash, Aim
├── bridge.rs       ← the wasm-bindgen bridge and its native stubs
├── court.rs        ← CourtPlugin: camera, geometry, HUD entities
├── shooting.rs     ← ShootingPlugin: charge, aim, launch, previews
├── physics.rs      ← PhysicsPlugin: gravity, collisions, scoring detection
├── feedback.rs     ← FeedbackPlugin: net, swish burst, HUD text
└── session.rs      ← SessionPlugin: shot limits and the panel sync
```

Read the right-hand column again: it's the *table of contents of the course*. Chapter 7 became `court.rs`, Chapter 8 `shooting.rs`, Chapters 9–10 `physics.rs` and `feedback.rs`, 11–12 `session.rs`. A well-factored codebase tells the story of its own construction.

## Step 1 — Modules: files that know each other

Rust's module system is refreshingly literal: **a file is a module.** Declare them in `main.rs`:

```rust
mod bridge;
mod components;
mod constants;
mod court;
mod feedback;
mod physics;
mod resources;
mod session;
mod shooting;
```

Then move each piece of the old `main.rs` into its file. Two mechanical rules cover almost everything:

1. **Anything used from outside its file needs `pub`.** `const BALL_R` becomes `pub const BALL_R`; `struct Ball { velocity: ... }` becomes `pub struct Ball { pub velocity: ... }` — fields need their own `pub`. The compiler will list every violation; fixing them is a guided tour of your own dependencies.
2. **Every use gets a path.** Inside `physics.rs`, the constants live at `crate::constants::*`, the ball at `crate::components::Ball` (`crate::` = "from my project's root"). Each file starts with a small block of `use` lines that *documents exactly what it depends on* — information the one-file version kept secret.

While moving the bridge into `bridge.rs`, improve its API for free: the functions become `bridge::take_reset()`, `bridge::shot_limit()`, `bridge::publish()` — the module name replaces the `js_` prefix. Callers read better and the stubs pattern is unchanged.

## Step 2 — Plugins: modules that install themselves

A module holds code; a **plugin** is Bevy's way of letting that code *register itself*. Here's `physics.rs`'s:

```rust
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (physics, collisions).chain().in_set(GameSet::Physics),
        );
    }
}
```

Another hand-written trait implementation (Chapter 11 was the first): `Plugin` has one required method, `build`, which receives the `App` and does to it exactly what `main()` used to do. Each plugin brings *everything its feature needs* — `ShootingPlugin` initializes the `Aim` resource, `FeedbackPlugin` brings `ScoreFlash`, `SessionPlugin` brings the four session resources and both sync systems. Delete a plugin from `main()` and its whole feature vanishes cleanly; that's the test of a good seam.

Remember Chapter 3, when `DefaultPlugins` installed windowing, rendering, and input? You're now on the other side of that API. Bevy's own engine is organized exactly like your game now is.

## Step 3 — System sets: ordering across plugin boundaries

The refactor breaks one thing, and it's the instructive one. The old `main.rs` had:

```rust
        .add_systems(Update, (sync_from_js, aim_and_launch, physics, collisions, sync_to_js).chain())
```

That `.chain()` worked because all five systems were registered *in one place*. Now they live in three different plugins that don't know each other. The fix is to name the pipeline's stages — in `main.rs`, since the frame's shape is an app-level decision:

```rust
/// The frame pipeline, named: state flows in, input acts, the world
/// simulates, state flows out. Plugins hang their systems on these.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    SyncIn,
    Input,
    Physics,
    SyncOut,
}
```

…and declare their order once:

```rust
        .configure_sets(
            Update,
            (
                GameSet::SyncIn,
                GameSet::Input,
                GameSet::Physics,
                GameSet::SyncOut,
            )
                .chain(),
        )
```

Each plugin then hangs its systems on the right hook: `aim_and_launch.in_set(GameSet::Input)`, `sync_from_js.in_set(GameSet::SyncIn)`, and physics chains its own pair *within* its set. **Sets are the contract; plugins are the implementations.** `SessionPlugin` doesn't know `ShootingPlugin` exists — both only know the frame has named stages. (The feedback systems join no set at all: they only read, so any order works, and Bevy stays free to parallelize them.)

## Step 4 — Verify: the refactor changed nothing

```
cargo build                                  ← native still compiles
cargo check --target wasm32-unknown-unknown  ← the bridge still compiles
trunk serve                                  ← play a full session
```

Shoot, bank, score, hit the limit, Save & Reset from the panel. Identical game. This is the discipline of the exercise: a refactor that "also fixed a little thing" is two changes wearing one commit, and when something breaks you won't know which one did it.

## About those slow per-chapter builds

A promise from Chapter 4 comes due. Each chapter folder compiles its own copy of Bevy into its own `target/` — minutes and gigabytes each. Two professional fixes:

- **A shared target directory.** Set the environment variable `CARGO_TARGET_DIR` to one fixed path, and every project on your machine shares one compilation cache — Bevy compiles once, ever. (This is exactly how this course's snapshots were built while writing it.)
- **A Cargo workspace.** A root `Cargo.toml` with `[workspace] members = ["chapters/*"]` makes the folders siblings in one project: shared `target/`, shared lockfile, `cargo build` from the root builds everything. Workspaces are how every multi-crate Rust project you'll ever meet is organized — including Bevy itself, which is ~40 crates in one workspace.

## Experiments before you move on

1. Comment out `feedback::FeedbackPlugin` in `main.rs`. Net, burst, and HUD updates vanish; ball, physics, and scoring don't. One line, one feature — that's the seam test.
2. Add a `DebugPlugin` in a new `debug.rs`: one system printing the ball's velocity when it's `Flying`. Notice you touch *zero* existing files except one line in `main.rs`.
3. Reorder the variants in `configure_sets` so `SyncOut` runs before `Physics`. Play — the panel now shows every value one frame late. Subtle ordering bugs are exactly what named sets protect you from.

## What you built / What's next

The same game, restructured for a team: nine single-purpose modules, five self-installing plugins, and a named frame pipeline that lets them coordinate without knowing each other. This structure is also this repo's [`final-game/`](../../final-game/) — the course's finished artifact.

Your code should now match this chapter's folder: [`chapters/13-refactoring-like-an-engineer/`](.).

One chapter left. In **Chapter 14**: release builds, making the WASM file small, and putting your game on the public internet.

**[Continue to Chapter 14: Optimizing and shipping →](../14-optimizing-and-shipping/README.md)**
