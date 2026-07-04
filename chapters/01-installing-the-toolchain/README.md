# Chapter 1 — Installing the Toolchain

*Read this in: **English** | [Español](README.es.md)*

In this chapter you'll install everything needed to build Rust games for the desktop **and** the browser, and verify each piece before moving on. Nothing here is hard, but it's where most tutorials lose people — so we go step by step and include the real errors you might hit.

**Time**: 30–60 minutes, mostly downloads. **Downloads**: ~8 GB on Windows (less on macOS/Linux).

## What we're installing, and why in this order

1. **Visual Studio Build Tools** *(Windows only)* — the linker Rust needs. Installing it first avoids the most common Windows-Rust error.
2. **rustup** — the official Rust installer, which brings **rustc** (the compiler) and **cargo** (the build tool and package manager).
3. **The WebAssembly target** — teaches the compiler to produce browser-runnable output.
4. **Trunk** — the tool that builds and serves Rust WASM apps.
5. **VS Code + rust-analyzer** — the editor (skip if you're using another editor).

## Step 1 — Visual Studio Build Tools (Windows only)

> macOS users: run `xcode-select --install` in Terminal instead, then jump to Step 2.
> Linux users: install your distro's C toolchain (`sudo apt install build-essential` on Ubuntu/Debian), then jump to Step 2.

Rust compiles your code, but on Windows it uses Microsoft's **linker** (`link.exe`) to produce the final executable — and that linker ships with Visual Studio's C++ tools, not with Rust.

1. Go to <https://visualstudio.microsoft.com/downloads/> and scroll to **Tools for Visual Studio** → **Build Tools for Visual Studio 2022**. Download and run it.
2. In the installer, check the workload **"Desktop development with C++"**. The defaults within it are fine.
3. Click **Install**. This is the big download (~6–7 GB) — good moment for a coffee.

> [!TIP]
> If you use `winget`, one command does the same:
> `winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`

> [!WARNING]
> **The error you get if you skip this step.** Everything will *seem* fine — until your first build, which fails with:
>
> ```
> error: linker `link.exe` not found
>   |
>   = note: program not found
>
> note: the msvc targets depend on the msvc linker but `link.exe` was not found
> note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.
> ```
>
> This happened to us building this very game. The fix is exactly this step: install the Build Tools with the C++ workload, then **restart your terminal** and build again. Your Rust code was never the problem.

## Step 2 — Rust, via rustup

**rustup** is Rust's official toolchain manager. It installs the compiler and Cargo, keeps them updated, and manages *targets* (like WebAssembly) — you'll never install Rust any other way.

### Windows

1. Go to <https://rustup.rs> and download `rustup-init.exe` (64-bit).
2. Run it. A terminal window opens and asks how to proceed:

```
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

3. Press **Enter** to accept the standard installation.
4. When it finishes you'll see `Rust is installed now. Great!`

### macOS / Linux

Open a terminal and run:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Press **Enter** at the same prompt to accept the standard installation.

### Verify it

**Close your terminal completely and open a new one** (this matters — see the warning below), then run:

```
rustc --version
cargo --version
```

You should see something like:

```
rustc 1.96.0 (ac68faa20 2026-05-25)
cargo 1.96.0 (30a34c682 2026-05-25)
```

> [!IMPORTANT]
> This course was written and tested with **Rust 1.96.0**. If rustup installed a *newer* stable version, that's fine — Rust itself is backwards-compatible, so newer compilers build this course's code without changes. The strict version pins in this course are for **Bevy, Trunk, and wasm-bindgen**, not for Rust.

> [!WARNING]
> **`rustc` is not recognized as a command.** If the version commands fail right after installing, 90% of the time the cause is this: the installer added Rust to your PATH, but *terminals that were already open don't see PATH changes*. This also bit us during the original build of this game. Close **every** open terminal (in VS Code too: kill the terminal, don't just clear it), open a fresh one, and try again. If it still fails after a fresh terminal *and* a reboot, re-run the rustup installer and check it didn't report an error.

> [!NOTE]
> **Rust sidebar: what did we just install?**
> - `rustc` is the compiler — it turns `.rs` source files into executable programs. You'll almost never call it directly.
> - `cargo` is the tool you'll actually live in: it creates projects, downloads libraries (Rust calls them *crates*), builds, runs, and tests your code. If you've used `npm` (JavaScript) or `pip` (Python), Cargo plays that role — plus the build system.
> - `rustup` manages the other two, and can install extra *targets*: instruction sets the compiler can produce output for. Which brings us to…

## Step 3 — The WebAssembly target

Out of the box, your compiler produces programs for *your* machine (on Windows, that target is called `x86_64-pc-windows-msvc`). To run in a browser, we need it to also produce **WebAssembly**. One command:

```
rustup target add wasm32-unknown-unknown
```

Expected output:

```
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
```

Verify it's registered:

```
rustup target list --installed
```

You should see `wasm32-unknown-unknown` in the list (alongside your native target).

> [!NOTE]
> **The weird name, decoded.** Target names follow the pattern *architecture-vendor-OS*. `wasm32` = 32-bit WebAssembly; the two `unknown`s mean "no particular vendor, no particular operating system" — because WASM runs inside a browser sandbox, not on an OS. You'll type this name a lot; it stops feeling strange quickly.

## Step 4 — Trunk

[Trunk](https://trunkrs.dev) is the build-and-serve tool for Rust WASM web apps. In one command (`trunk serve`) it compiles your Rust to WebAssembly, generates the JavaScript glue the browser needs, bundles it with your HTML, serves it locally, and rebuilds on every file change.

Install the exact version this course uses:

```
cargo install trunk --version 0.21.14 --locked
```

> [!TIP]
> `cargo install` compiles Trunk from source, so this takes several minutes and prints hundreds of `Compiling ...` lines. That's normal — let it run. You just used Cargo as a package manager for the first time.

Verify:

```
trunk --version
```

```
trunk 0.21.14
```

## Step 5 — VS Code and rust-analyzer

*(Skip if you're using another editor — just install its Rust/rust-analyzer support.)*

1. Install VS Code from <https://code.visualstudio.com>.
2. Open it, go to Extensions (`Ctrl+Shift+X` / `Cmd+Shift+X`), search for **rust-analyzer**, and install the one published by *rust-lang* (millions of installs).

rust-analyzer gives you three things a beginner shouldn't work without: errors shown **inline as you type** (before you even build), autocompletion that knows the whole Bevy API, and hover-documentation on any function. When we start writing code in Chapter 2, hover over anything you don't recognize.

## Final verification — run all of these

Open a **new** terminal and confirm every line works:

| Command | Expected |
|---|---|
| `rustc --version` | `rustc 1.96.0` (or newer) |
| `cargo --version` | `cargo 1.96.0` (or newer) |
| `rustup target list --installed` | includes `wasm32-unknown-unknown` |
| `trunk --version` | `trunk 0.21.14` |

All four pass? **Your machine can now build Rust games for the desktop and the browser.** That's the whole battle of this chapter.

## What you built / What's next

Nothing compiled yet — but you assembled and verified a complete, professional Rust web-game toolchain, and you know the two classic setup failures (missing linker, stale PATH) and their fixes.

In **Chapter 2** you'll write, build, and run your first Rust program with Cargo, and learn your way around a Rust project's anatomy.

**[Continue to Chapter 2: Hello, Cargo →](../02-hello-cargo/README.md)**
