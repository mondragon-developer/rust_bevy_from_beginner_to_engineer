# Chapter 10 — Scoring and Feedback

*Read this in: **English** | [Español](README.es.md)*

At the end of Chapter 9 you sank a basket and the game just… sat there. This chapter gives it eyes and a voice: it detects the exact frame the ball drops through the hoop, counts makes and attempts, shows them in an on-screen HUD, and celebrates every swish with a green flash and an expanding burst. On the way you'll meet Bevy's UI system and one genuinely engineer-grade trick: change detection.

**Time**: ~1 hour.

## Step 1 — Three new resources

```rust
#[derive(Resource, Default)]
struct Score(u32);

// Total shots taken (made or missed).
#[derive(Resource, Default)]
struct Attempts(u32);

// Seconds of "swish" feedback remaining after a made basket.
#[derive(Resource, Default)]
struct ScoreFlash(f32);
```

Register all three in `main()` with `.init_resource::<...>()` — they start at zero, which is exactly right.

> [!NOTE]
> **Rust sidebar: tuple structs.** `struct Score(u32);` is a *tuple struct* — a struct with one unnamed field, accessed as `score.0`. Why not a plain `u32`? Because resources are looked up **by type**: `Res<Score>` and `Res<Attempts>` are different requests precisely because they're different types. Two bare `u32`s would be indistinguishable. The wrapper *is* the name.

`ScoreFlash` is a pattern worth naming: **a timer as a resource**. When a basket is made we'll set it to `0.7` (seconds); a small system counts it down; every visual that wants to celebrate just asks "is it still positive?" One number coordinates the net color and the burst animation, with no direct coupling between the systems involved.

## Step 2 — Detecting the basket

The detection is four lines in `collisions`, placed right after the rim bounce — and it's the payoff for a field we've been carrying since Chapter 8:

```rust
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
```

(The system's signature grows two parameters: `mut score: ResMut<Score>, mut flash: ResMut<ScoreFlash>`.)

Why not just "is the ball inside the hoop"? Because a fast ball moves many pixels per frame — it might be above the rim on one frame and below it the next, never *exactly* at it. So we detect the **crossing**: last frame the center was above the rim line (`prev_pos.y > RIM_Y`), this frame it's at or below (`pos.y <= RIM_Y`), and it's genuinely falling (`velocity.y < 0.0` — no scoring on the way *up* through the hoop). The x-range check with a 6-pixel inset demands a clean entry through the opening rather than a graze past its edges.

This before/after-crossing technique shows up everywhere in games (laps, checkpoints, triggers). It's why `physics` saves `prev_pos` before every move.

Also new: `Attempts` counts up in `aim_and_launch`, right at the moment of release — `attempts.0 += 1;` after the launch lines. Every release is a shot, made or missed.

## Step 3 — The HUD: Bevy UI text

Two new spawns at the end of `setup`:

```rust
    // The HUD: score in the top-left corner. UI positions are in screen
    // pixels from the window's top-left, not world coordinates.
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
```

Everything here is still just entities and components — but these components put the entity in **UI space**, not world space. A `Node` is positioned in *screen pixels from the window's top-left corner* (CSS-style, and yes — `top` instead of our usual y-up), so the HUD stays glued to the corner no matter how the camera scales the court. `Text`, `TextFont`, and `TextColor` do what they say.

The score text also gets a marker component — the same trick as `Ball` in Chapter 4:

```rust
/// Marker for the HUD text entity so update_score_text can find it.
#[derive(Component)]
struct ScoreText;
```

## Step 4 — Updating the HUD (only when something changed)

```rust
/// Rewrite the HUD only on frames where the numbers actually changed.
fn update_score_text(
    score: Res<Score>,
    attempts: Res<Attempts>,
    mut q: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() && !attempts.is_changed() {
        return;
    }
    if let Ok(mut text) = q.single_mut() {
        text.0 = format!("Made: {}   Shots: {}", score.0, attempts.0);
    }
}
```

The naive version would rebuild the string every frame — 60 allocations a second to display numbers that change a few times a minute. Instead, Bevy **tracks writes to every resource automatically**: `score.is_changed()` is true only on frames where some system actually wrote to `Score`. This is *change detection*, one of Bevy's best engineering features, and this two-line guard is its whole API. (`format!` is `println!`'s sibling that returns the string instead of printing it.)

## Step 5 — The celebration

`draw_net` grows into `draw_scene` — same net, but flash-aware, plus the burst:

```rust
/// The net (green while celebrating) and the expanding swish burst.
fn draw_scene(mut gizmos: Gizmos, flash: Res<ScoreFlash>) {
    let orange = Color::srgb(0.95, 0.45, 0.15);
    let scored = flash.0 > 0.0;
    let net = if scored {
        Color::srgb(0.3, 1.0, 0.45)
    } else {
        Color::srgba(0.85, 0.85, 0.9, 0.85)
    };

    // ...rim nub and net strands exactly as in Chapter 7, drawn in `net` color...

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
```

Read the burst's radius line closely: `flash.0` runs 0.7 → 0.0, so `(0.7 - flash.0)` runs 0.0 → 0.7, and the ring of ten spokes expands from radius 30 to 170 over the celebration — animation driven purely by the countdown, no extra state. The spokes are placed by walking angles around a circle (`TAU` is a full turn in radians) — the same `t`-walking pattern as the net strands in Chapter 7.

Register the whole feedback family in `main()` — these don't need `.chain()`, they only read:

```rust
        .add_systems(Update, (draw_scene, update_score_text, tick_flash))
```

## Run it

```
trunk serve        (or: cargo run)
```

Sink one. The instant the ball drops through the rim: **Made: 1**, the net flares green, and the burst blooms outward while the ball keeps falling through the net onto the floor — the make stays visible, which is why the scoring code deliberately doesn't reset the ball:

![A made basket: green net, expanding swish burst, and the HUD reading Made: 1](../../assets/ch10-swish.png)

## Experiments before you move on

1. Longer celebration: `flash.0 = 0.7` to `2.0` — and notice the burst now expands slower *and* farther. Why? (Look at the radius formula — then fix it so 2.0 feels right.)
2. Make bank shots worth 2: in the scoring block, `score.0 += 1` → check `ball.velocity.x < 0.0` (came off the backboard) and add 2 if so.
3. Break change detection on purpose: delete the `is_changed` guard, add `info!("rebuilt the HUD string");` inside, and watch your terminal drown — 60 lines a second. Put the guard back.

## What you built / What's next

A game that keeps score: crossing detection built on `prev_pos`, three resources including a timer, a screen-space HUD that updates only when the numbers change, and a celebration animated entirely by a countdown.

Your code should now match this chapter's folder: [`chapters/10-scoring-and-feedback/`](.).

In **Chapter 11**, shots get consequences: a shot limit, a game-over state, and a session flow — the difference between a toy and a game you can *lose*.

**[Continue to Chapter 11: Game sessions →](../11-game-sessions/README.md)**
