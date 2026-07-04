# Chapter 14 — Optimizing and Shipping

*Read this in: **English** | [Español](README.es.md)*

Your game is finished. This chapter is about the distance between *finished* and *shipped* — and it opens with a number that explains why the distance exists: the development build of your game is a **66.4 MB** WASM file. Nobody's phone is downloading that. By the end of this chapter it will be a fraction of the size, live on a public URL you can put in your CV.

**Time**: ~1 hour (plus one long release build).

## Step 1 — The release profile

Cargo has had two personalities all along: `cargo build` uses the `dev` profile (fast builds, big output — what we've tuned since Chapter 3), and `--release` uses the `release` profile. Ours goes in `Cargo.toml`, and it's the last block the file will ever need:

```toml
# Release profile tuned for small WASM bundles shipped to the browser.
[profile.release]
opt-level = "z"
lto = "thin"
codegen-units = 1
panic = "abort"
```

Line by line, because each is a real engineering trade-off:

- **`opt-level = "z"`** — optimize for *size*, not speed. Levels 1–3 make code faster; `"s"` and `"z"` make it smaller. For a 2D game that already runs at full speed, every megabyte matters more than every microsecond — a web game's biggest performance problem is its *download*.
- **`lto = "thin"`** — link-time optimization: the compiler optimizes *across* crate boundaries, which above all means **deleting code**. Bevy ships an entire engine; your game uses a slice of it. LTO is how the unused animation, audio, and glTF machinery falls out of the binary.
- **`codegen-units = 1`** — normally Rust splits each crate into chunks compiled in parallel (fast builds, missed optimizations across chunk borders). One unit = slowest build, best output. Fine for release: you build it once per ship.
- **`panic = "abort"`** — when a Rust program panics it normally *unwinds*, carefully dismantling the call stack; that machinery is code you ship. Abort means "just stop" — the right answer for a game, and free bytes back.

Every line trades **build time for output quality**. That's why it's the release profile and not the dev one.

## Step 2 — The optimizer after the compiler

Remember this line in `index.html`, planted in Chapter 12?

```html
<link data-trunk rel="rust" data-wasm-opt="z" />
```

`wasm-opt` is a *second* optimizer (from the WebAssembly toolkit Binaryen) that works directly on the compiled `.wasm` — shrinking instruction encodings, deduplicating, dead-code-eliminating what the compiler missed. The `data-wasm-opt="z"` attribute tells Trunk to run it, at size setting, on release builds only. Two optimizers in a row is standard practice on the web target — the compiler thinks in Rust, `wasm-opt` thinks in WASM.

## Step 3 — Build and measure

```
trunk build --release
```

Go make the Chapter 3 coffee — LTO with one codegen unit rebuilds every dependency and then `wasm-opt` chews the result (5 minutes 44 seconds total on our machine: 3m 08s of compiling, the rest is `wasm-opt`). Then look inside `dist/`:

| Build | .wasm size |
|---|---|
| `trunk build` (dev) | 66.4 MB |
| `trunk build --release` | **17.3 MB** |

**74% smaller.** Three-quarters of the binary was optimization headroom — debug scaffolding, unwinding machinery, and engine code your game never calls, all deleted by the profile you just wrote.

Two honest notes to calibrate expectations. First: yes, that's still big for a little basketball game — you're shipping a real game engine. Trimming further is real engineering (Bevy's *cargo features* let you compile out subsystems you don't use — sound, 3D, scenes; that's your first stop if you chase megabytes). Second: the wire is kinder than the disk — web servers compress WASM with gzip or brotli automatically, so what a player actually downloads is typically a third to half of the file size.

And notice what `dist/` *is*: an `index.html`, one `.js` glue file, one `.wasm`. **Three static files.** No server code, no database, no build step on the host. Anything that can serve files can serve your game — which is why shipping it is this easy:

## Step 4 — Put it on the internet

**Option A — GitHub Pages** (free, and your code is probably already on GitHub):

1. Build with the repo name as the base path — Pages serves you from a subdirectory:
   ```
   trunk build --release --public-url /YOUR-REPO-NAME/
   ```
2. Push `dist/` to the repo (e.g. on a `gh-pages` branch, or `/docs` on main — remove `dist/` from `.gitignore` for that branch).
3. In the repo's **Settings → Pages**, point Pages at that branch/folder.
4. Your game is live at `https://YOUR-USERNAME.github.io/YOUR-REPO-NAME/`.

**Option B — Vercel / Netlify** (free tiers, nicer URLs): both serve a static folder. Install their CLI or drag-and-drop the `dist/` folder in their dashboard; tell it the output directory is `dist`; done. (This is how the course's reference game is deployed.)

Either way — **send the link to someone.** A game nobody else has played is a build artifact; a game with one other player has shipped.

## Step 5 — Show it off

Your repo's README deserves proof the game exists: a screenshot and a short capture of a made basket. On Windows, `Win+Alt+R` (Game Bar) records a clip; ScreenToGif or LICEcap make GIFs directly. Fifteen seconds of charge → arc → swish → green burst tells more than any paragraph — it's exactly how this course's own README works.

## You did it

Take stock of the distance covered. In Chapter 0 you had no compiler. You now have a physics game **you understand down to every line** — you wrote the gravity, derived the bounce formula, designed the state machine, bridged two languages, refactored it like a team would, and shipped a size-optimized build to a public URL.

The "to engineer" checklist, in retrospect: named constants as one source of truth · frame-rate-independent simulation · ECS architecture · borrow-checker-driven parallelism · change detection · cross-boundary protocols with take-and-clear semantics · conditional compilation and platform stubs · modules, plugins, and system sets · build profiles and two-stage optimization · deployment. That's not a toy list. That's a working vocabulary.

**Where to go next:**

- **Add to this game**: sound on the swish (`bevy_audio` is already in your binary), touch input for phones, a moving hoop, a two-player mode, a shot-arc replay.
- **Trim it**: explore Bevy's cargo features and see how small you can get `dist/`.
- **Upgrade it**: Bevy moves fast. Bumping this game to the next Bevy version with the official migration guide is the single most instructive exercise left in this repo — now that every line that breaks is a line you wrote.
- **Join in**: the [Bevy examples](https://github.com/bevyengine/bevy/tree/main/examples) folder, [The Rust Book](https://doc.rust-lang.org/book/) for the language depth this course deferred, and the Bevy Discord, where beginners are welcome.

Thanks for playing. 🏀

**[Back to the course index →](../../README.md)**
