# Capítulo 2 — Hola, Cargo

*Léelo en: [English](README.md) | **Español***

Hora de escribir y ejecutar tu primer programa en Rust. Al final de este capítulo conocerás la estructura de un proyecto Rust — la misma que usará nuestro juego de baloncesto — y habrás conocido el rasgo de personalidad más famoso del compilador de Rust: se niega a compilar código con ciertos tipos de bugs, y te dice exactamente cómo arreglarlos.

**Tiempo**: ~30 minutos.

## Paso 1 — Crea un proyecto

Abre una terminal en la carpeta donde guardas tus proyectos y ejecuta:

```
cargo new hello_cargo
cd hello_cargo
```

`cargo new` crea una carpeta con un proyecto listo para ejecutar:

```
hello_cargo/
├── .gitignore      ← le dice a git que ignore la salida de compilación
├── Cargo.toml      ← el "DNI" del proyecto
└── src/
    └── main.rs     ← tu código vive aquí
```

Ese es el esqueleto *completo* de un proyecto Rust. Nuestro juego de baloncesto terminado tiene las mismas tres piezas — solo que con más código en `src/`.

## Paso 2 — El DNI del proyecto: Cargo.toml

Abre la carpeta en tu editor (`code .` abre VS Code en el directorio actual) y mira `Cargo.toml`:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- **`[package]`** describe tu proyecto: su nombre, su versión y su *edición*.
- **`edition`** es el "dialecto anual" de Rust en que está escrito el código. Las ediciones permiten que el lenguaje evolucione sin romper código antiguo.
- **`[dependencies]`** es donde listarás las bibliotecas que usa tu proyecto. Ahora está vacío; en el Capítulo 3, `bevy` irá aquí — y esa única línea es lo que convierte este esqueleto en un proyecto de motor de juegos.

**Cambia la línea de edición a `"2021"`:**

```toml
edition = "2021"
```

> [!IMPORTANT]
> Todo el código de este curso — incluido el juego final — usa la **edición 2021**, así que la estandarizamos desde ya. Las versiones nuevas de `cargo` generan `edition = "2024"` por defecto; ambas funcionan, pero mantener todos los capítulos idénticos significa que siempre podrás comparar tu código con las carpetas de los capítulos sin ruido.

> [!NOTE]
> **Sidebar de Rust: este formato de archivo es TOML** ("Tom's Obvious Minimal Language") — secciones entre `[corchetes]`, pares `clave = "valor"` debajo. Solo editarás cosas pequeñas en él a mano. La extensión `.toml` es la razón por la que el archivo se llama *Cargo-punto-toml*.

## Paso 3 — Ejecútalo

`cargo new` ya escribió un programa diminuto en `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Ejecútalo:

```
cargo run
```

```
   Compiling hello_cargo v0.1.0 (C:\...\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
     Running `target\debug\hello_cargo.exe`
Hello, world!
```

Acaban de pasar tres cosas: Cargo **compiló** tu código fuente a un ejecutable real, **lo colocó** en una nueva carpeta `target/`, y **lo ejecutó**. También apareció un archivo `Cargo.lock` — el registro exacto de Cargo de cada versión de dependencia usada, para que las compilaciones sean reproducibles.

> [!NOTE]
> **Sidebar de Rust: tus dos primeras construcciones de Rust.**
> - `fn main()` declara una *función* llamada `main` — la especial donde empieza todo programa Rust.
> - `println!(...)` imprime una línea de texto. El `!` significa que es una *macro*, no una función normal — código que escribe código en tiempo de compilación. Por ahora, lo práctico es solo esto: algunas cosas que llamas terminan en `!`, y `println!` es la que usarás constantemente.

> [!TIP]
> `target/` se hace grande (gigabytes, cuando llegue Bevy) y siempre es regenerable — por eso `.gitignore` la excluye. Nunca la subas a git, nunca la respaldes, y bórrala con libertad si necesitas espacio (`cargo clean` hace exactamente eso).

## Paso 4 — Un programa de verdad: tiros libres de práctica

Reemplaza el contenido de `src/main.rs` con esto — escríbelo, no lo pegues:

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

*(Los comentarios del código están en inglés en todo el curso — igual que los mensajes del compilador y la documentación de Bevy que leerás a diario. Acostumbrarte a ellos es parte del entrenamiento.)*

Ejecútalo con `cargo run`:

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

Línea por línea, esto es lo nuevo:

> [!NOTE]
> **Sidebar de Rust: las variables son inmutables por defecto.**
> `let player = "Rusty"` crea una variable — y salvo que digas lo contrario, es *inmutable*: su valor no puede cambiar nunca. Este es el movimiento estrella de Rust. Una variable que va a cambiar, como nuestro contador de canastas, debe declararse `let mut made = 0` (*mut* de *mutable*). ¿Por qué inmutable por defecto? Porque la mayoría de los valores de la mayoría de los programas nunca cambian tras crearse, y a los bugs les encantan las variables que cambiaron cuando nadie lo esperaba. Rust convierte "esto puede cambiar" en algo que declaras a propósito, y el compilador te lo hace cumplir.

> [!NOTE]
> **Sidebar de Rust: `for`, rangos, `if` y `%`.**
> - `for shot in 1..=total_shots` ejecuta el cuerpo del bucle una vez por cada número del 1 al 10. El rango `1..=10` *incluye* el 10; escribir `1..10` pararía en el 9.
> - `if` no necesita paréntesis alrededor de la condición — `if shot % 3 == 0 {` — pero las llaves `{}` son siempre obligatorias.
> - `%` es el operador de resto: `shot % 3 == 0` es verdadero cuando `shot` es divisible entre 3 — los tiros 3, 6 y 9.
> - `println!("{player} made {made}...")` — poner el nombre de una variable dentro de `{}` inserta su valor en el texto.

## Paso 5 — Rómpelo a propósito

Este es el paso más importante del capítulo. Borra el `mut` para que la línea 10 diga `let made = 0;`, y ejecuta `cargo run`:

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

Léelo despacio, porque así es trabajar en Rust: el compilador te dice **qué** está mal (asignar a una variable inmutable), **dónde** — archivo, línea, columna, citando el código culpable — y **cómo arreglarlo**, hasta los caracteres exactos que añadir. Los mensajes de error de Rust están considerados los mejores de cualquier lenguaje. Cuando aparezca uno, no entres en pánico — léelo; la respuesta suele estar dentro.

Devuelve el `mut` a su sitio y confirma que vuelve a funcionar.

## Los tres comandos de Cargo que usarás a diario

| Comando | Qué hace | Cuándo usarlo |
|---|---|---|
| `cargo check` | Comprueba que el código compila — no construye nada | Constantemente mientras escribes; es el más rápido |
| `cargo run` | Compila (si hace falta) y ejecuta | Para probar tu programa de verdad |
| `cargo build` | Compila sin ejecutar | Rara vez a mano; `run` lo hace por ti |

## Qué construiste / Qué sigue

Creaste un proyecto Rust desde cero, aprendiste el papel de cada archivo, escribiste un programa con variables, un bucle y una condición — y provocaste tu primer error de compilador a propósito, que es la mejor forma de conocerlo.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/02-hello-cargo/`](.).

En el **Capítulo 3**, una línea en `Cargo.toml` trae a Bevy, y `cargo run` abre una ventana de juego de verdad. (Aviso justo: esa primera compilación de Bevy tarda un buen rato. Explicaremos por qué, y cómo hacer rápidas todas las siguientes.)

**[Continuar al Capítulo 3: Tu primera ventana →](../03-first-window/README.es.md)**
