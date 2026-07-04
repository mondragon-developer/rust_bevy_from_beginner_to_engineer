# Capítulo 7 — La cancha

*Léelo en: [English](README.md) | **Español***

**Aquí empieza la Parte III.** Se acabaron las escenas de demostración — desde este capítulo hasta el final del curso, cada línea de código que escribas es una línea del juego de baloncesto real. Hoy construyes el mundo donde ocurre: la cancha, el tablero, el aro, la red — y lo organizas como lo haría un ingeniero, con cada medida en un único lugar con nombre.

**Tiempo**: ~1 hora.

## Paso 1 — La cancha, en números

Crea el proyecto (`the_court`), con el `Cargo.toml` del Capítulo 6 (Bevy, el pin de wasm-bindgen, los perfiles) y su `index.html` (actualiza el `<title>`). Después, empieza `main.rs` con algo nuevo — un bloque de *constantes* antes de cualquier código:

```rust
use bevy::{prelude::*, render::camera::ScalingMode};

// ---------- The court, in numbers ----------

// Fixed play area so the whole court stays visible at any window/canvas size.
const WORLD_W: f32 = 1280.0;
const WORLD_H: f32 = 720.0;

const BALL_R: f32 = 26.0;
const GROUND_Y: f32 = -320.0;
// The ball starts at the free-throw spot, resting on the floor.
const START: Vec2 = Vec2::new(-420.0, GROUND_Y + BALL_R);

const BACKBOARD_X: f32 = 470.0;
const BACKBOARD_Y: f32 = 130.0;
const BACKBOARD_W: f32 = 16.0;
const BACKBOARD_H: f32 = 150.0;
const BACKBOARD_FRONT: f32 = BACKBOARD_X - BACKBOARD_W / 2.0;

const RIM_Y: f32 = 70.0;
const RIM_FRONT_X: f32 = 350.0;
const RIM_BACK_X: f32 = BACKBOARD_FRONT;
```

Estos son los números *reales* del juego terminado, y no volverán a cambiar en el resto del curso. Léelos como un plano sobre el sistema de coordenadas del Capítulo 4: la línea del suelo está en y = −320; la pelota (radio 26) empieza a la izquierda, en el punto de tiro libre; el tablero es una placa de 16×150 a la derecha; el aro cuelga en y = 70, extendiéndose desde x = 350 hasta la cara frontal del tablero.

¿Por qué constantes en vez de escribir `470.0` donde haga falta? Tres razones de ingeniería:

1. **Los nombres llevan significado.** `BACKBOARD_FRONT` te dice qué *es* un número; un `462.0` suelto, no.
2. **Una sola fuente de verdad.** Cuando las físicas (Capítulo 9) necesiten rebotar la pelota contra la cara frontal del tablero, usarán `BACKBOARD_FRONT` — el código que dibuja y el código de colisiones no pueden estar en desacuerdo.
3. **Los valores derivados se mantienen correctos.** `BACKBOARD_FRONT` está *calculado* (`BACKBOARD_X - BACKBOARD_W / 2.0`), y `RIM_BACK_X` es igual a él — así que la red siempre llega hasta el tablero, aunque muevas el aro. Prueba a cambiar `BACKBOARD_X` al final del capítulo y mira cómo todo lo sigue.

> [!NOTE]
> **Sidebar de Rust: `const`.** Una `const` es un valor fijado en tiempo de compilación: el tipo es obligatorio (`: f32`), el nombre va en `SCREAMING_SNAKE_CASE` por convención, y a diferencia de `let` puede vivir fuera de cualquier función, visible para todo el archivo. Se permite matemática simple dentro — eso es lo que hace posibles las constantes derivadas como `BACKBOARD_FRONT`.

## Paso 2 — Una cámara que siempre muestra la cancha entera

Dos mejoras en `main()`. Primero, el color de fondo se convierte en un *recurso*:

```rust
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.12)))
```

Conociste `Res<Time>` en el Capítulo 5; ahora estás *creando* un recurso. Un recurso es un dato global que comparte todo el juego — existe exactamente uno. `ClearColor` es el "pinta la pantalla de este color antes de dibujar cada frame" de Bevy. La puntuación y el estado del juego también serán recursos, empezando el próximo capítulo.

Segundo, la cámara recibe una proyección:

```rust
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WORLD_W,
                min_height: WORLD_H,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
```

Hasta ahora, redimensionar la ventana mostraba *más o menos mundo*. Para un juego eso está mal — un jugador con un monitor enorme vería más allá de la cancha. `ScalingMode::AutoMin` cambia la regla: **la cámara siempre muestra al menos 1280×720 de mundo**, escalado para caber. Redimensiona la ventana como quieras — encógela, estírala — la cancha entera sigue visible. En el navegador (donde el canvas puede tener cualquier tamaño) esto es lo que mantiene el juego jugable en todas partes.

## Paso 3 — La pelota se vuelve un círculo

Los cuadrados estaban bien para aprender; los balones de baloncesto son redondos. Dibujar *formas* (no sprites rectangulares) introduce un último concepto de renderizado:

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ...cámara...

    // The ball: a real circle at last. In front (z = 1) so it draws
    // over the rim and backboard.
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_R))),
        MeshMaterial2d(materials.add(Color::srgb(0.95, 0.5, 0.2))),
        Transform::from_translation(START.extend(1.0)),
    ));
```

- Un **mesh** es una forma (aquí, un círculo construido con triángulos — todo el dibujo en GPU son triángulos). Un **material** es el aspecto de su superficie (aquí, naranja baloncesto plano).
- Los meshes y materiales viven en el **almacén de assets** de Bevy — ese es el parámetro `ResMut<Assets<Mesh>>`, otro recurso, esta vez prestado mutablemente porque le estamos añadiendo cosas.
- `meshes.add(...)` guarda la forma y devuelve un **handle** — un ticket ligero que apunta a los datos reales. La entidad lleva el ticket, no la forma. Si más tarde creáramos cien pelotas, compartirían un solo mesh a través de cien handles baratos.
- `START.extend(1.0)` convierte el punto 2D en 3D añadiéndole z = 1.0 — nuestra capa de "la pelota se dibuja delante" del Capítulo 4.

## Paso 4 — Tablero, poste, aro, suelo

El resto de `setup` son cuatro sprites, con cada posición derivada de las constantes:

```rust
    // The floor: extra wide so it never shows an edge on wide screens.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::new(WORLD_W * 2.0, 60.0)),
        Transform::from_xyz(0.0, GROUND_Y - 30.0, -1.0),
    ));

    // Support pole behind the hoop.
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.4, 0.42, 0.48),
            Vec2::new(12.0, BACKBOARD_Y - GROUND_Y),
        ),
        Transform::from_xyz(BACKBOARD_X + 16.0, (BACKBOARD_Y + GROUND_Y) / 2.0, -1.0),
    ));

    // The backboard.
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.9, 0.9, 0.95),
            Vec2::new(BACKBOARD_W, BACKBOARD_H),
        ),
        Transform::from_xyz(BACKBOARD_X, BACKBOARD_Y, 0.0),
    ));

    // Solid rim bar across the hoop opening (drawn behind the ball).
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.95, 0.4, 0.1),
            Vec2::new(RIM_BACK_X - RIM_FRONT_X, 7.0),
        ),
        Transform::from_xyz((RIM_FRONT_X + RIM_BACK_X) / 2.0, RIM_Y, 0.5),
    ));
}
```

Fíjate en el nivel de lectura que has alcanzado: la altura del poste es `BACKBOARD_Y - GROUND_Y` (del suelo al tablero), centrado en su punto medio; el ancho de la barra del aro es `RIM_BACK_X - RIM_FRONT_X`, centrada entre ambos. Las z apilan la escena: suelo y poste detrás (−1), tablero en 0, aro delante de él (0.5), pelota delante de todo (1).

## Paso 5 — La red, dibujada con Gizmos

Los sprites y meshes son *retenidos*: los creas una vez y persisten. Los **Gizmos** son lo contrario — líneas de modo inmediato que desaparecen cada frame y hay que redibujar. Perfectos para cosas que cambian constantemente (el próximo capítulo dibujan la trayectoria de apuntado y la barra de potencia). La red es nuestro ensayo:

```rust
/// Gizmos are redrawn from scratch every frame, so this runs in Update.
fn draw_net(mut gizmos: Gizmos) {
    let orange = Color::srgb(0.95, 0.45, 0.15);
    let net = Color::srgba(0.85, 0.85, 0.9, 0.85);

    // Front rim nub so the front edge of the hoop opening is obvious.
    gizmos.line_2d(
        Vec2::new(RIM_FRONT_X, RIM_Y - 6.0),
        Vec2::new(RIM_FRONT_X, RIM_Y + 6.0),
        orange,
    );

    // Net: angled strands from the rim opening converging to a point below.
    let bottom = Vec2::new((RIM_FRONT_X + RIM_BACK_X) / 2.0, RIM_Y - 55.0);
    let segs = 6;
    for i in 0..=segs {
        let t = i as f32 / segs as f32;
        let top = Vec2::new(RIM_FRONT_X + (RIM_BACK_X - RIM_FRONT_X) * t, RIM_Y);
        gizmos.line_2d(top, top.lerp(bottom, 0.9), net);
    }
    // One horizontal strand so the net reads as woven, not just lines.
    gizmos.line_2d(
        Vec2::new(RIM_FRONT_X + 14.0, RIM_Y - 28.0),
        Vec2::new(RIM_BACK_X - 14.0, RIM_Y - 28.0),
        net,
    );
}
```

Regístralo con `.add_systems(Update, draw_net)`. El bucle cuelga siete hilos repartidos uniformemente por la boca del aro: `t` camina de 0.0 a 1.0, colocando el extremo superior de cada hilo, y `lerp` (interpolación lineal — "camina esta fracción del camino hacia aquel punto") los inclina hacia un punto de reunión más abajo. Siete líneas de matemáticas que se leen como la descripción de una red.

## Ejecútalo

```
cargo run        (o: trunk serve)
```

![La cancha de baloncesto: pelota, tablero, aro, red y poste](../../assets/ch07-the-court.png)

Esa es la cancha de la captura del Capítulo 0 — porque *es* la cancha del juego terminado, dibujada por el código del juego terminado.

## Experimentos antes de continuar

1. Baja el aro para practicar mates: `RIM_Y` a `-50.0`. La red, el saliente y la barra del aro lo siguen — una constante, el aro entero.
2. Acerca el aro: `BACKBOARD_X` a `300.0`. Mira cómo `BACKBOARD_FRONT` y `RIM_BACK_X` se propagan tras él.
3. Alarga la red: el `-55.0` de `bottom` a `-100.0`.
4. Redimensiona la ventana agresivamente mientras corre — la cancha entera sigue encuadrada. Eso es `AutoMin` trabajando.

## Qué construiste / Qué sigue

El escenario del juego completo — construido con formas reales, apilado con z, siempre encuadrado a cualquier tamaño de ventana, y especificado por un bloque de números con nombre que los capítulos de físicas reutilizarán tal cual.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/07-the-court/`](.).

En el **Capítulo 8**, la pelota te responde: input de ratón, el medidor de potencia con carga progresiva, la trayectoria de apuntado — y el estado del juego gestionado en recursos.

**[Continuar al Capítulo 8: La mecánica de tiro →](../08-the-shooting-mechanic/README.es.md)**
