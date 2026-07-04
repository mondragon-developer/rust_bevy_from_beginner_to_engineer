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
