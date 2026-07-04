# Chapter 3 — Your First Window

*Read this in: **English** | [Español](README.es.md)*

In this chapter, one line in `Cargo.toml` turns your Rust skeleton into a game-engine project, and `cargo run` opens a real, GPU-rendered game window. You'll also learn the two facts of life about compiling Bevy — the first build is slow, every build after is fast — and the standard trick that keeps it that way.

**Time**: ~30 minutes of reading and typing, plus one long compile you only pay once (see Step 4 — it's coffee-worthy).

## Step 1 — Create the project

```
cargo new first_window
cd first_window
```

Open `Cargo.toml` and set `edition = "2021"`, as we standardized in Chapter 2.

## Step 2 — Add Bevy

Still in `Cargo.toml`, add one line under `[dependencies]`:

```toml
[dependencies]
bevy = "0.16"
```

That's the whole installation of a game engine. When you next build, Cargo will download Bevy and everything Bevy itself depends on, compile it all, and link it into your program.

> [!IMPORTANT]
> The version matters: **`"0.16"`**, exactly as written. It means "any 0.16.x release" — bug-fix updates are fine, but Cargo will never silently jump you to 0.17, whose API changes would break every code block in this course.

> [!NOTE]
> **Rust sidebar: what's a dependency, really?** Rust libraries are called *crates*, and they're published to a public registry at [crates.io](https://crates.io). The line `bevy = "0.16"` tells Cargo: fetch the `bevy` crate, version 0.16-something, from the registry. The exact versions Cargo picks get recorded in `Cargo.lock`, so your build and ours use identical code.

## Step 3 — The build-speed trick

Before the first build, add this to the bottom of `Cargo.toml`:

```toml
# Build our own code lightly but dependencies fully optimized, so dev
# rebuilds stay fast while Bevy itself (the slow part) runs at full speed.
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

Here's the reasoning, because this trick is standard practice in every serious Bevy project:

- Your **dependencies** (Bevy and friends) are compiled **once**, then reused for every build after. So we optimize them fully (`opt-level = 3`) — pay once, and the engine runs at real speed even during development.
- **Your own code** recompiles every time you change it — which is constantly. So we keep its optimization light (`opt-level = 1`), making each rebuild fast.

Without this, you'd either wait ages on every rebuild (everything optimized) or get a game that runs like a slideshow (nothing optimized, and an unoptimized game engine is *really* slow).

## Step 4 — The code

Replace `src/main.rs` with:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My First Bevy Window".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
```

Fourteen lines. Before we run it, let's read it:

- `use bevy::prelude::*;` — imports Bevy's *prelude*: the curated set of names (like `App`, `Window`, `default`) that almost every Bevy program needs. One `use` line instead of fifty.
- `App::new()` — creates an empty Bevy application.
- `.add_plugins(DefaultPlugins...)` — installs Bevy's standard equipment: window creation, GPU rendering, input handling, audio, time, logging. In Bevy, *everything* is a plugin — including the engine's own core features. Our finished game adds its own plugins the same way.
- `.set(WindowPlugin { ... })` — customizes one of those default plugins: we give the window a title and a 1280×720 size instead of the defaults.
- `.run()` — hands control to Bevy, which opens the window and starts the *game loop*: an infinite cycle of "process input → update the world → draw the frame", repeating ~60 times per second until you close the window. Every game you've ever played runs on this loop; from now on, so do yours.

> [!NOTE]
> **Rust sidebar: the builder pattern.** `App::new().add_plugins(...).run()` is a *method chain*: each call returns the object back, so you keep calling methods on the result, reading top to bottom like a recipe. Rust APIs use this style everywhere, and Bevy's `App` is a classic example — you'll extend this exact chain with your own systems in the next chapter.

> [!NOTE]
> **Rust sidebar: structs and `..default()`.** `Window { title: ..., resolution: ..., ..default() }` creates a *struct* — a bundle of named fields, like an object literal. A `Window` has dozens of fields (cursor options, transparency, vsync…), and we only care about two. The magic `..default()` at the end means: "fill in every field I didn't mention with its default value." You'll use this constantly in Bevy, which favors big configurable structs. (And `Some(...)` around it? That wraps a value that's allowed to be absent — we'll meet it properly when the game needs it.)

## Step 5 — Run it (and go get that coffee)

```
cargo run
```

The first build compiles Bevy's entire dependency tree — hundreds of crates. On our machine it compiled **324 crates in 4 minutes 36 seconds** (a machine that has never built Rust before also spends a few extra minutes downloading them first). You'll see a long parade of `Compiling ...` lines; this is normal, and you only pay it once per project.

When it finishes, a 1280×720 window opens with the title "My First Bevy Window":

![An empty Bevy window with a dark background](../../assets/ch03-first-window.png)

It's empty and dark — no camera, no sprites, nothing to draw yet — but look at what's actually running: a native window, a GPU rendering pipeline (that background color is being redrawn by your graphics card ~60 times a second), keyboard and mouse input capture, and a live game loop. Close it with the window's ✕ button, and the program exits cleanly.

Now the second fact of life. Change the window title to anything you like, save, and `cargo run` again:

```
   Compiling first_window v0.1.0 (...)
    Finished `dev` profile [optimized + debuginfo] target(s) in 8.33s
```

**Eight seconds.** Only *your* crate recompiled; Bevy was already built. This is your development rhythm from here on: slow once, fast forever.

> [!WARNING]
> **`error: linker 'link.exe' not found`** on this build means the Visual Studio Build Tools from Chapter 1 are missing or incomplete — this is exactly the error we hit building the original game. Go back to [Chapter 1, Step 1](../01-installing-the-toolchain/README.md#step-1--visual-studio-build-tools-windows-only), install the "Desktop development with C++" workload, restart your terminal, and run again.

> [!TIP]
> Seeing warnings in the terminal while the window is open — mentions of `wgpu`, graphics adapters, or missing features? As long as the window opened, ignore them. Bevy's renderer probes your GPU's capabilities and chats about what it finds; it's diagnostics, not errors.

## What you built / What's next

A real game window with a running game loop, in fourteen lines — plus the two pieces of Bevy craft everyone learns first: the exact-version pin and the dev-profile trick.

Your code should now match this chapter's folder: [`chapters/03-first-window/`](.).

In **Chapter 4** we put something *in* the window: a camera, a sprite — and the big idea that organizes every Bevy game, including ours: **ECS** — entities, components, and systems.

**[Continue to Chapter 4: ECS — entities, components, systems →](../04-ecs-entities-components-systems/README.md)**
