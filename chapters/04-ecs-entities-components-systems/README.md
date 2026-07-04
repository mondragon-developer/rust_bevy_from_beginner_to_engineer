# Chapter 4 — ECS: Entities, Components, Systems

*Read this in: **English** | [Español](README.es.md)*

Your window opens; now we put things in it. But first, this chapter teaches the single most important idea in the whole course: **ECS**, the architecture that organizes every Bevy game — including our basketball game, whose ball, hoop, backboard, score display, and even camera are all built from the three concepts you're about to meet.

**Time**: ~45 minutes.

## The big idea

Bevy games are made of exactly three kinds of things:

- An **Entity** is a *thing* in your game — the ball, the hoop, the camera, the score text. By itself, an entity is nothing but an ID number. What makes it a "ball" is what you attach to it:
- A **Component** is a piece of data attached to an entity. A `Transform` (where it is), a `Sprite` (what it looks like), a `Ball` tag (what it *is*). An entity is fully described by its set of components — nothing more.
- A **System** is a plain Rust function that runs on a schedule (once at startup, or every frame) and does things to entities that have particular components. "Move everything that has a `Transform` and a `Velocity`." "Check every `Ball` against every `Hoop`."

A picture that works: **a spreadsheet**. Entities are the rows. Components are the columns. Each row fills in only the columns that apply to it. Systems are formulas that operate on every row that has values in the columns they care about.

| Entity (row) | `Transform` | `Sprite` | `Ball` | `Camera2d` |
|---|---|---|---|---|
| #1 (the camera) | ✓ | | | ✓ |
| #2 (the floor) | ✓ | ✓ | | |
| #3 (the ball) | ✓ | ✓ | ✓ | |

There's no "Ball class" that inherits from "GameObject". There are just rows, columns, and functions over them. When our game gets physics in Chapter 9, we won't modify some ball object — we'll write a system that says "every frame, for every entity with a `Transform` and a `Velocity`, apply gravity." The ball qualifies; the hoop doesn't; done.

## Step 1 — The project

```
cargo new ecs_ball
cd ecs_ball
```

Set up `Cargo.toml` exactly like Chapter 3: `edition = "2021"`, `bevy = "0.16"` under `[dependencies]`, and the two `[profile.dev]` blocks. (From now on, every chapter starts this way and we won't repeat it.)

> [!TIP]
> Because Chapter 3 already compiled Bevy 0.16 once, Cargo reuses the downloaded crates — but each *project* still compiles its own copy into its own `target/` folder, so the first build of this project takes minutes again. Bear with it; there's a professional fix for this (shared build caches, workspaces) that we'll meet in Chapter 13.

## Step 2 — The code

Replace `src/main.rs`:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ECS: A Ball on the Court".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

/// Marker component: "this entity is the ball".
#[derive(Component)]
struct Ball;

fn setup(mut commands: Commands) {
    // Without a camera, nothing gets drawn.
    commands.spawn(Camera2d);

    // The floor: a long, dark strip near the bottom of the screen.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.20, 0.25), Vec2::new(1280.0, 40.0)),
        Transform::from_xyz(0.0, -320.0, 0.0),
    ));

    // The ball: an orange square for now (it becomes a circle later).
    commands.spawn((
        Ball,
        Sprite::from_color(Color::srgb(0.96, 0.55, 0.13), Vec2::splat(50.0)),
        Transform::from_xyz(0.0, -275.0, 1.0),
    ));
}
```

Everything new here, one piece at a time.

### Your first system

`.add_systems(Startup, setup)` registers the function `setup` as a system, scheduled in **`Startup`** — Bevy runs it exactly once, before the first frame. (The other schedule you'll use constantly is **`Update`**: every frame, forever. That's Chapter 5.)

Look at `setup`'s signature: `fn setup(mut commands: Commands)`. Systems are ordinary functions, but their *parameters are a request form*. Whatever you list, Bevy hands you when it calls the function. Here we request `Commands` — Bevy's tool for creating and destroying entities.

### Your first entities

Each `commands.spawn(...)` creates one entity — one new row in the spreadsheet:

- `commands.spawn(Camera2d)` — one component: `Camera2d`. **The camera is just an entity like everything else.** No camera, no picture (see the warning below).
- The floor — two components: a `Sprite` (a flat-colored rectangle, 1280×40 pixels, dark gray-blue) and a `Transform` (placed near the bottom).
- The ball — three components: our own `Ball` marker, an orange 50×50 `Sprite` (`Vec2::splat(50.0)` is shorthand for `Vec2::new(50.0, 50.0)`), and a `Transform` that sits it on the floor.

To attach multiple components, you pass them as a tuple: `spawn((a, b, c))` — note the double parentheses.

### Your first component

```rust
#[derive(Component)]
struct Ball;
```

Two lines, and they carry the whole ECS philosophy. `struct Ball;` declares a struct with **no fields** — it holds no data at all. Its only job is to *mark* an entity, so that future systems can say "give me the entity that has `Ball` on it." In Chapter 9, physics will apply to the ball and not the floor precisely because of this marker.

> [!NOTE]
> **Rust sidebar: `#[derive(...)]` and traits.** A *trait* is Rust's version of an interface: a named capability a type can have. `Component` is a Bevy trait meaning "this type may be attached to entities." Writing the implementation by hand would be boilerplate, so `#[derive(Component)]` asks the compiler to **generate it for you**. You've already used derived traits without knowing: printing with `{}` works through the `Display` trait. Deriving is everywhere in Rust — our finished game derives `Component`, `Resource`, and more, and now you know what the incantation means.

### The coordinate system

`Transform::from_xyz(0.0, -320.0, 0.0)` places an entity. Bevy's 2D coordinates:

- **The origin (0, 0) is the center of the window** — not a corner.
- **+x goes right, +y goes up** (math-style, not screen-style: y grows *upward*).
- In our 1280×720 window, x spans −640…+640 and y spans −360…+360. So the floor at y = −320 hugs the bottom, and the ball at y = −275 sits right on top of it.
- **z is the layering order** for 2D: higher z draws on top. The ball has z = 1.0 so it can never vanish behind the floor.

Colors: `Color::srgb(0.96, 0.55, 0.13)` is red/green/blue with each channel from 0.0 to 1.0 — this one is basketball orange.

## Step 3 — Run it

```
cargo run
```

![A window showing an orange square resting on a dark floor strip](../../assets/ch04-ecs-ball.png)

A floor, and a ball resting on it. Not moving yet — no system runs after startup — but this scene is *structured like a real game*: three entities, each defined purely by its components.

> [!WARNING]
> **Window opens but it's completely empty?** You forgot the camera — delete `commands.spawn(Camera2d);` and you'll see it: the app runs happily, renders nothing, and reports no error, because "no camera" is a valid state. It's the classic Bevy beginner trap. If the screen is ever unexpectedly empty, check the camera first.

## Experiments before you move on

Each takes one line — change it, `cargo run`, see it:

1. Move the ball to the top-left: `Transform::from_xyz(-500.0, 250.0, 1.0)`.
2. Make it huge: `Vec2::splat(200.0)`.
3. Spawn a second ball at a different x — copy the whole `commands.spawn((...))` block. Two rows, same columns.
4. Set the ball's z to `-1.0` and move it down to y = −320: it slides *behind* the floor.

## What you built / What's next

A structured game scene — camera, floor, ball — and the mental model the rest of the course builds on: entities are rows, components are columns, systems are functions over them.

Your code should now match this chapter's folder: [`chapters/04-ecs-entities-components-systems/`](.).

In **Chapter 5**, we write our first `Update` system and make the ball *move* — which means meeting queries, delta time, and Rust's most famous feature: the borrow checker.

**[Continue to Chapter 5: Making things move →](../05-making-things-move/README.md)**
