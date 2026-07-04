# Chapter 2 — Hello, Cargo

*Read this in: **English** | [Español](README.es.md)*

Time to write and run your first Rust program. By the end of this chapter you'll know your way around a Rust project — the same layout our basketball game will use — and you'll have met the Rust compiler's most famous personality trait: it refuses to compile code with certain kinds of bugs, and tells you exactly how to fix them.

**Time**: ~30 minutes.

## Step 1 — Create a project

Open a terminal in the folder where you keep your projects and run:

```
cargo new hello_cargo
cd hello_cargo
```

`cargo new` creates a folder with a ready-to-run project inside:

```
hello_cargo/
├── .gitignore      ← tells git to ignore build output
├── Cargo.toml      ← the project's "ID card"
└── src/
    └── main.rs     ← your code lives here
```

That's the *entire* skeleton of a Rust project. Our finished basketball game has the same three pieces — just with more code in `src/`.

## Step 2 — The project's ID card: Cargo.toml

Open the folder in your editor (`code .` opens VS Code in the current directory) and look at `Cargo.toml`:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- **`[package]`** describes your project: its name, its version, and its *edition*.
- **`edition`** is which "dialect year" of Rust the code is written in. Editions let the language evolve without breaking old code.
- **`[dependencies]`** is where you'll list the libraries your project uses. It's empty now; in Chapter 3, `bevy` goes here — and that one line is what turns this skeleton into a game engine project.

**Change the edition line to `"2021"`:**

```toml
edition = "2021"
```

> [!IMPORTANT]
> All code in this course — including the final game — uses **edition 2021**, so we standardize on it now. Newer `cargo` versions generate `edition = "2024"` by default; both work, but keeping every chapter identical means you can always diff your code against the chapter snapshots without noise.

> [!NOTE]
> **Rust sidebar: this file format is TOML** ("Tom's Obvious Minimal Language") — sections in `[brackets]`, `key = "value"` pairs below them. You'll only ever hand-edit small things in it. The `.toml` extension is why the file is called *Cargo-dot-toml*.

## Step 3 — Run it

`cargo new` already wrote a tiny program in `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Run it:

```
cargo run
```

```
   Compiling hello_cargo v0.1.0 (C:\...\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
     Running `target\debug\hello_cargo.exe`
Hello, world!
```

Three things just happened: Cargo **compiled** your source into a real executable, **placed it** in a new `target/` folder, and **ran it**. You also gained a `Cargo.lock` file — Cargo's exact record of every dependency version used, so builds are reproducible.

> [!NOTE]
> **Rust sidebar: your first two Rust constructs.**
> - `fn main()` declares a *function* named `main` — the special one where every Rust program starts.
> - `println!(...)` prints a line of text. The `!` means it's a *macro*, not a regular function — code that writes code at compile time. For now, the practical takeaway is just: some things you call end in `!`, and `println!` is the one you'll use constantly.

> [!TIP]
> `target/` gets big (gigabytes, once Bevy arrives) and is always regenerable — that's why `.gitignore` excludes it. Never commit it, never back it up, and delete it freely if you need disk space (`cargo clean` does exactly that).

## Step 4 — A real program: practice free throws

Replace the contents of `src/main.rs` with this — type it, don't paste it:

```rust
fn main() {
    println!("Hello, basketball!");

    // A player and their stats. `let` creates a variable.
    let player = "Rusty";
    let total_shots = 10;

    // `mut` makes a variable changeable. Without it, Rust
    // refuses to let you modify the value. Try removing it!
    let mut made = 0;

    // Practice free throws: shots 1 to 10 (the `=` includes the 10).
    for shot in 1..=total_shots {
        // Our imaginary player sinks every 3rd shot.
        if shot % 3 == 0 {
            made += 1;
            println!("Shot {shot}: SWISH!");
        } else {
            println!("Shot {shot}: rim out...");
        }
    }

    println!("{player} made {made} of {total_shots} shots.");
}
```

Run it with `cargo run`:

```
Hello, basketball!
Shot 1: rim out...
Shot 2: rim out...
Shot 3: SWISH!
Shot 4: rim out...
Shot 5: rim out...
Shot 6: SWISH!
Shot 7: rim out...
Shot 8: rim out...
Shot 9: SWISH!
Shot 10: rim out...
Rusty made 3 of 10 shots.
```

Line by line, here's what's new:

> [!NOTE]
> **Rust sidebar: variables are immutable by default.**
> `let player = "Rusty"` creates a variable — and unless you say otherwise, it's *immutable*: its value can never change. This is Rust's signature move. A variable that will change, like our shot counter, must be declared `let mut made = 0` (*mut* for *mutable*). Why default to unchangeable? Because most values in most programs never change after creation, and bugs love variables that changed when nobody expected them to. Rust makes "this can change" something you declare on purpose, and the compiler holds you to it.

> [!NOTE]
> **Rust sidebar: `for`, ranges, `if`, and `%`.**
> - `for shot in 1..=total_shots` runs the loop body once for each number from 1 to 10. The range `1..=10` *includes* 10; writing `1..10` would stop at 9.
> - `if` needs no parentheses around the condition — `if shot % 3 == 0 {` — but the braces `{}` are always required.
> - `%` is the remainder operator: `shot % 3 == 0` is true when `shot` divides evenly by 3 — shots 3, 6, and 9.
> - `println!("{player} made {made}...")` — putting a variable name inside `{}` embeds its value in the text.

## Step 5 — Break it on purpose

This is the most important step of the chapter. Delete the `mut` so line 10 reads `let made = 0;`, then run `cargo run`:

```
error[E0384]: cannot assign twice to immutable variable `made`
  --> src\main.rs:16:13
   |
10 |     let made = 0;
   |         ---- first assignment to `made`
...
16 |             made += 1;
   |             ^^^^^^^^^ cannot assign twice to immutable variable
   |
help: consider making this binding mutable
   |
10 |     let mut made = 0;
   |         +++
```

Read it slowly, because this is what working in Rust is like: the compiler tells you **what's** wrong (assigning to an immutable variable), **where** — file, line, column, with the offending code quoted — and **how to fix it**, down to the exact characters to add. Rust error messages are widely considered the best of any language. When one appears, don't panic — read it; the answer is usually inside.

Put the `mut` back and confirm it runs again.

## The three Cargo commands you'll use daily

| Command | What it does | When to use it |
|---|---|---|
| `cargo check` | Checks the code compiles — builds nothing | Constantly while writing; it's the fastest |
| `cargo run` | Compiles (if needed) and runs | To actually try your program |
| `cargo build` | Compiles without running | Rarely by hand; `run` does it for you |

## What you built / What's next

You created a Rust project from scratch, learned the role of every file in it, wrote a program with variables, a loop, and a condition — and got your first compiler error on purpose, which is the best way to meet it.

Your code should now match this chapter's folder: [`chapters/02-hello-cargo/`](.).

In **Chapter 3**, one line in `Cargo.toml` brings in Bevy, and `cargo run` opens a real game window. (Fair warning: that first Bevy compile takes a while. We'll explain why, and how to make every later compile fast.)

**[Continue to Chapter 3: Your first window →](../03-first-window/README.md)**
