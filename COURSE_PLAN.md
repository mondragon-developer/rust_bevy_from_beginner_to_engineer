# Course Plan (internal)

This file is the production blueprint for the course. It is not a lesson — learners should start at [README.md](README.md).

## Locked decisions

| Decision | Choice |
|---|---|
| Repo | Standalone course repo; the original `rustbyve` project is the private reference implementation |
| Chapter code | Folder per chapter — every chapter ends with a complete, compilable snapshot in `chapters/NN-name/` |
| Language | Bilingual: `README.md` (English) + `README.es.md` (Spanish) in every folder |
| Rust depth | Teach Rust as needed, via "Rust sidebar" callouts where a concept first appears in game code |
| Editor | VS Code + rust-analyzer shown in screenshots; every chapter states any editor/IDE works |

## Pinned versions — never write "latest" anywhere in the course

| Tool | Version |
|---|---|
| rustc / cargo | 1.96.0 |
| rustup | 1.29.0 |
| Rust edition | 2021 |
| bevy | 0.16 |
| wasm-bindgen | =0.2.122 (exact pin — must match Trunk's wasm-bindgen-cli) |
| trunk | 0.21.14 |

## Chapter outline

### Part I — Getting ready (beginner)

- **00 — Before you start**: what we're building (screenshots + GIF), who the course is for, how the course works, hardware/OS requirements, tool overview, editor choice.
- **01 — Installing the toolchain**: rustup on Windows (+ macOS/Linux notes), Visual Studio Build Tools, verifying `cargo`, installing the `wasm32-unknown-unknown` target, installing Trunk, VS Code + rust-analyzer. Troubleshooting: linker `link.exe` not found, rustup not on PATH in the current shell.
- **02 — Hello, Cargo**: `cargo new`, anatomy of a Rust project, `cargo run`/`check`/`build`. Rust sidebars: `fn main`, `println!`, variables & mutability.

### Part II — First steps with Bevy

- **03 — Your first window**: add Bevy, `App`, `DefaultPlugins`, why the first compile is slow, dev-profile trick (`opt-level = 1` app / `3` deps). Rust sidebars: structs, methods, the builder pattern.
- **04 — ECS: entities, components, systems**: spawn a camera and a sprite; what ECS is and why Bevy uses it. Rust sidebars: `#[derive]`, traits.
- **05 — Making things move**: a system with `Query` and `Time`, transform math, frame independence. Rust sidebars: ownership & borrowing (as seen in queries), `&mut`.
- **06 — Running in the browser (WASM)**: what WebAssembly is, `index.html` + Trunk, `trunk serve`, canvas sizing. Troubleshooting: wasm-bindgen version mismatch, port already in use.

### Part III — Building the basketball game

- **07 — The court**: coordinate system, drawing court, backboard, hoop and ball from sprites/shapes; constants module.
- **08 — The shooting mechanic**: mouse/keyboard input, hold-to-charge power, aiming; `Resource` for game state. Rust sidebars: enums & `match`.
- **09 — Physics**: velocity, gravity, integration each frame, bouncing off floor/backboard/rim. Rust sidebars: `Vec2`/`Vec3` math, `f32`.
- **10 — Scoring and feedback**: hoop collision detection, score `Resource`, on-screen UI text, visual feedback on make/miss.
- **11 — Game sessions**: shot limits, results tracking, reset flow. Rust sidebars: `Option`, `Vec`, iterators.

### Part IV — From beginner to engineer

- **12 — Talking to the web page**: `wasm-bindgen` bridge between Bevy and the HTML control panel (player name field, results dropdown, reset button, shot-limit toggle); why the exact-version pin exists.
- **13 — Refactoring like an engineer**: split the one-file game into modules and Bevy plugins; where systems, components, resources each live; states.
- **14 — Optimizing and shipping**: release profile for small WASM (`opt-level = "z"`, LTO, `panic = "abort"`), measuring bundle size, `trunk build --release`, deploying (GitHub Pages / Vercel), recording your own gameplay video.

## Production workflow (how each chapter is written)

1. Actually execute every step in the chapter folder — every command in the prose has been run, every code block compiles at that point in the course.
2. Capture real screenshots (Windows 11, VS Code, Chrome) into `assets/chNN/`; reference them from both language versions.
3. Write `README.md` (EN) first, then mirror to `README.es.md` (ES). The two must stay in sync — a change to one is a change to both.
4. Callout conventions (GitHub alerts):
   - `> [!NOTE]` — Rust sidebar (language concept explained at first use)
   - `> [!TIP]` — shortcuts, quality-of-life
   - `> [!WARNING]` — Troubleshooting: a real error, its real cause, its fix
   - `> [!IMPORTANT]` — version pins and things that break if skipped
5. Each chapter ends with: "What you built / What's next" + a checkpoint note ("your code should now match `chapters/NN-name/`").

## Media plan

- Hero: animated GIF (autoplays in README) + `.mp4` (better quality, click to play) of a full game session — recorded from the final game in the browser.
- Per-chapter screenshots of the running state, plus VS Code screenshots for setup chapters.
- All media in `assets/`, named `chNN-short-description.png`.
