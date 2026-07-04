# Chapter 0 — Before You Start

*Read this in: **English** | [Español](README.es.md)*

Welcome! Before installing anything, this short chapter shows you where you're going, what you need, and how the course works. Ten minutes here will save you hours later.

## What you're building

By the end of this course you will have built — and understood, line by line — this game:

![The finished basketball game running in a browser](../../assets/ch00-final-game.png)

It's a 2D basketball shooting game that runs **in the browser**:

- **Hold-to-charge shooting** — press and hold, and a power meter charges; aim with your cursor; release to shoot.
- **Real physics** — the ball follows a gravity arc, and bounces off the floor, the backboard, and the rim.
- **Scoring** — the game detects clean baskets and updates your score with visual feedback.
- **Game sessions** — play with a shot limit, see your results, enter your name, and reset — all from an HTML control panel that talks directly to the Rust game code.

Three technologies make this work together:

| Technology | What it is | Its job in our game |
|---|---|---|
| **Rust** | A fast, memory-safe systems programming language | The language every line of game logic is written in |
| **Bevy** | A modern, free and open-source game engine written in Rust | Windows, rendering, input, and the game loop, so we focus on *our* game |
| **WebAssembly (WASM)** | A binary format browsers can run at near-native speed | Lets our compiled Rust game run in Chrome, Firefox, Safari, or Edge — no install needed to play |

## Who this course is for

**Complete beginners.** You do not need to know Rust. You do not need to have made a game before. If you've written *any* code in *any* language (even a little), you'll be comfortable; if you haven't, you'll need patience but nothing else.

The course title says "to engineer" and it means it: the early chapters hold your hand through installation and first programs, and the final chapters cover things professional developers do — software architecture (ECS, modules, plugins), performance tuning, bundle-size optimization, and deployment.

## How the course works

**Each chapter has its own folder, and each folder contains a complete, working copy of the project as it exists at the end of that chapter.** If your code breaks and you can't figure out why, compare it against the chapter folder — or copy the chapter folder and keep going from there. You can never be permanently stuck.

As you read, you'll see four kinds of callout boxes:

> [!NOTE]
> **Rust sidebar.** Boxes like this explain a Rust language concept — ownership, enums, traits — at the exact moment the game code first uses it. No theory before you need it.

> [!WARNING]
> **Troubleshooting.** Boxes like this describe a *real* error, with the *real* message you'll see and the fix. Every one of these happened to us while building this game.

> [!TIP]
> **Tips** are optional shortcuts and quality-of-life improvements.

> [!IMPORTANT]
> **Important** boxes are not optional — usually version pins or steps that break everything if skipped.

## What you need

### A computer

- **Operating system**: Windows 10/11, macOS, or Linux. The course screenshots are from **Windows 11**, and Windows-specific steps (like the Visual Studio Build Tools) are covered in detail — but every chapter works on all three systems, and we note the differences where they exist.
- **Disk space**: about **10 GB free**. This surprises people: Rust itself is ~1.5 GB, the Windows build tools are several GB, and a Bevy project's `target/` build folder easily reaches 5+ GB. It's normal.
- **Memory**: 8 GB RAM minimum; 16 GB makes compiling noticeably more pleasant.
- **Internet**: needed to download tools and dependencies (a few GB total, mostly in Chapters 1 and 3).

### An editor

We use **Visual Studio Code** (free) with the **rust-analyzer** extension, and that's what you'll see in every screenshot. rust-analyzer gives you autocompletion, inline error messages, and go-to-definition — for a beginner, inline errors alone are worth it.

**But any editor or IDE works.** RustRover, Zed, Helix, Vim, Neovim, Sublime Text — Rust compiles from the command line, so the editor is your choice. If you already have a favorite, keep it; just look for its rust-analyzer (or built-in Rust) support.

### The tools we'll install in Chapter 1

You don't need to install these now — Chapter 1 walks through each one. This is just so you know what's coming and why:

| Tool | Version we use | Why we need it |
|---|---|---|
| **rustup** | 1.29.0 | The official Rust installer and version manager |
| **Rust (rustc + cargo)** | 1.96.0 | The compiler, and Cargo — Rust's build tool and package manager |
| **Visual Studio Build Tools** | 2022 | *(Windows only)* the linker Rust needs to produce `.exe` files |
| **wasm32-unknown-unknown target** | — | Teaches the Rust compiler to output WebAssembly instead of a Windows/macOS/Linux program |
| **Trunk** | 0.21.14 | Builds and serves Rust WASM apps for the browser with one command |
| **VS Code + rust-analyzer** | latest is fine | The editor (only tool where "latest" is safe) |

> [!IMPORTANT]
> **Versions matter in this course.** Bevy changes its API between releases — code written for Bevy 0.16 (this course) will not compile on Bevy 0.17 or later. Everywhere the course says a version, use exactly that version while following along. When you finish the course, upgrading is a great exercise — and by then you'll be able to read Bevy's migration guides yourself.

### What you *don't* need

- ❌ Prior Rust knowledge
- ❌ Game development experience
- ❌ Math beyond arithmetic — we introduce the tiny amount of vector math we use (positions and velocities are just pairs of numbers)
- ❌ A GPU or gaming PC — this 2D game runs on any recent laptop

## Checklist before Chapter 1

- [ ] I have ~10 GB of free disk space
- [ ] I have a stable internet connection for the downloads
- [ ] I've picked an editor (VS Code if unsure)
- [ ] I understand the version pins are not optional

## What's next

In **Chapter 1** you'll install the complete toolchain — Rust, the build tools, the WebAssembly target, and Trunk — and verify every piece works before writing a single line of code. It's the least glamorous chapter and the one where most tutorials lose people, so we go slowly and cover the real errors you might hit.

**[Continue to Chapter 1: Installing the toolchain →](../01-installing-the-toolchain/README.md)**
