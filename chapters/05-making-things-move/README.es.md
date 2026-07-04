# Capítulo 5 — Poniendo cosas en movimiento

*Léelo en: [English](README.md) | **Español***

Un juego donde nada se mueve es un cuadro. En este capítulo escribes tu primer **sistema `Update`** — código que corre en cada frame — y animas la pelota con él. Por el camino conocerás las tres herramientas que usa todo sistema de gameplay: las **queries** (encuentra las entidades que me importan), **`Time`** (cuánto duró el último frame), y el **borrow checker** de Rust, que resulta ser la razón por la que Bevy es rápido.

**Tiempo**: ~45 minutos.

## Paso 1 — Empieza desde el Capítulo 4

Crea el proyecto (`cargo new moving_ball`), configura `Cargo.toml` como siempre, y copia dentro el `main.rs` del Capítulo 4 — cámara, suelo, pelota, marcador `Ball`. Esa escena es nuestro punto de partida.

## Paso 2 — Primer intento: solo muévela

Añade una función de sistema nueva al final del archivo:

```rust
/// Runs every frame: push the ball to the right.
fn move_ball(mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in &mut query {
        transform.translation.x += 3.0;
    }
}
```

Y regístrala en `main()`, justo después de la línea de `Startup`:

```rust
        .add_systems(Startup, setup)
        .add_systems(Update, move_ball)
```

`Update` es el calendario de cada-frame: Bevy llama a `move_ball` una vez por frame, para siempre. Ejecútalo — la pelota se desliza a la derecha y se sale de la pantalla. ¡Movimiento! Aunque con dos problemas, y arreglarlos enseña las dos grandes lecciones de este capítulo.

### ¿Qué es una Query?

`Query<&mut Transform, With<Ball>>` es el sistema pidiéndole entidades a Bevy, y se lee en dos mitades:

- **Primera mitad — a qué quiero acceder**: `&mut Transform` — "dame el `Transform` de cada entidad, y pienso *modificarlo*."
- **Segunda mitad — qué entidades cualifican**: `With<Ball>` — "solo entidades que además tengan el componente `Ball`. No necesito leer `Ball` (está vacío, de todas formas); solo tiene que estar ahí."

En términos de hoja de cálculo: *selecciona la columna `Transform`, para las filas donde la columna `Ball` esté marcada*. La cámara y el suelo también tienen Transform — el filtro es lo que los protege. Después, `for mut transform in &mut query` recorre cada coincidencia; hoy es una pelota, pero el mismo sistema movería mil.

> [!NOTE]
> **Sidebar de Rust: `&`, `&mut` y el borrow checker.** En Rust, `&cosa` es una *referencia* — permiso para **leer** `cosa` — y `&mut cosa` es permiso para **modificarla**. El compilador impone una ley sobre ellas: en cualquier momento, un valor puede tener **muchos lectores, o exactamente un escritor — nunca ambos**. Eso es el *borrow checker*, y elimina categorías enteras de bugs (datos que cambian bajo tus pies, dos escritores chocando) en tiempo de compilación.
>
> Y esta es la recompensa para los juegos: Bevy lee tus queries como declaraciones de intención. Un sistema que pide `&Transform` (lectura) puede correr sin peligro **en paralelo** con otros lectores; un sistema que pide `&mut Transform` obtiene acceso exclusivo. Tu juego usa todos los núcleos de la CPU, automáticamente, *porque* las reglas de préstamo lo hacen demostrablemente seguro. El borrow checker no es un obstáculo que superar — es el planificador del motor haciendo su trabajo.

## Paso 3 — Segundo intento: respeta el tiempo

La versión ingenua tiene un bug escondido: `+= 3.0` por **frame** significa que la velocidad de la pelota depende de la tasa de frames. En una pantalla de 60 Hz se mueve 180 píxeles/segundo; en un monitor gaming de 144 Hz, 432 — el juego va más del doble de rápido en la máquina de tu amigo. Los juegos de verdad se mueven por **segundo**, no por frame:

```rust
fn move_ball(time: Res<Time>, mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in &mut query {
        transform.translation.x += 200.0 * time.delta_secs();
    }
}
```

Dos cambios:

- El sistema ahora también pide **`Res<Time>`** — el reloj de Bevy. `Res` significa *resource* (recurso): datos globales que pertenecen al juego entero y no a ninguna entidad. (Nuestro juego terminado guarda la puntuación en un recurso; esa historia empieza en el Capítulo 8.)
- `time.delta_secs()` es la duración del último frame en segundos — unos 0.016 a 60 FPS, unos 0.007 a 144 FPS. Multiplicar por él hace que las cuentas salgan idénticas en ambas máquinas: **200 píxeles por segundo, en todas partes.**

> [!IMPORTANT]
> Esta es la regla de oro del movimiento en juegos: **todo lo que cambia con el tiempo se multiplica por el delta time.** Cada velocidad, cada rotación, cada temporizador del resto de este curso lo hace. Cuando llegue la gravedad en el Capítulo 9, es el delta time lo que hace correctas las físicas.

## Paso 4 — Versión final: de un lado a otro, y rodando

Independiente del frame rate — pero la pelota sigue saliéndose de la pantalla. Reemplaza `move_ball` con la versión con la que este capítulo termina:

```rust
/// Runs every frame: slide the ball back and forth along the floor,
/// rolling it in the direction it travels.
fn move_ball(time: Res<Time>, mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in &mut query {
        // A smooth wave between -400 and +400 as time passes.
        let x = (time.elapsed_secs() * 0.8).sin() * 400.0;

        // How far we moved this frame decides how much the ball rolls.
        let dx = x - transform.translation.x;
        transform.translation.x = x;
        transform.rotate_z(-dx * 0.02);
    }
}
```

- `time.elapsed_secs()` es el tiempo *total* desde que arrancó el juego (frente a `delta_secs()`, solo el último frame).
- `.sin()` convierte el tiempo, que crece sin parar, en una onda suave que oscila para siempre entre −1 y +1; por 400, la pelota patrulla entre x = −400 y x = +400, frenando con gracia en los bordes. El `0.8` es el dial de velocidad.
- `transform.rotate_z(...)` gira el sprite (en radianes). Rotamos según lo que la pelota se movió este frame, así que *rueda* como una pelota en vez de deslizarse como una caja — y rueda al revés cuando cambia de sentido.

```
cargo run
```

![La pelota naranja rodando, inclinada y lejos del centro](../../assets/ch05-moving-ball.png)

La pelota patrulla la cancha, rodando por el camino, cambiando de sentido con una suavidad natural. Eso es una onda seno haciendo trabajo de animación gratis.

## Experimentos antes de continuar

1. Acelérala: cambia `0.8` por `3.0`. Amplía la patrulla: `400.0` por `600.0`.
2. Borra `With<Ball>` de la query y ejecuta. El *suelo* también patrulla — y la cámara también, lo cual es desorientante y maravilloso. Los filtros importan. Devuélvelo.
3. Haz que ruede al revés: quita el signo menos de `rotate_z`. Ahora hace el moonwalk — tu ojo lo pilla al instante.
4. Añade una segunda pelota en otra x inicial (experimento del Capítulo 4). Las dos se mueven — un sistema, cada entidad que coincida.

## Qué construiste / Qué sigue

Una escena viva: tu primer sistema de cada-frame, guiado por una query que elige exactamente las entidades correctas y con matemáticas que corren a la misma velocidad en cualquier máquina.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/05-making-things-move/`](.).

En el **Capítulo 6**, esta ventana aprende un truco nuevo: correr dentro de un navegador. WebAssembly, Trunk y tu toolchain del Capítulo 1 por fin se encuentran.

**[Continuar al Capítulo 6: Corriendo en el navegador →](../06-running-in-the-browser/README.es.md)**
