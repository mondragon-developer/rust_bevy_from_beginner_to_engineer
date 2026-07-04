# Capítulo 8 — La mecánica de tiro

*Léelo en: [English](README.md) | **Español***

El juego se convierte en juego: en este capítulo lees el ratón, cargas un tiro manteniendo pulsado, apuntas con el cursor, previsualizas el arco con una trayectoria punteada y lanzas la pelota con gravedad real. Es el capítulo más grande hasta ahora y se lo gana — la gestión de input, los enums, `Option` y el ordenamiento de sistemas llegan aquí, exactamente como los usa el juego terminado.

**Tiempo**: ~1.5 horas.

## Paso 1 — Constantes nuevas: el bloque de "game feel"

Empieza desde el código del Capítulo 7 (proyecto `shooting_mechanic`, mismo `Cargo.toml` e `index.html`). Añade un segundo bloque de constantes justo después de la cancha:

```rust
// ---------- Game feel — tweak these to change difficulty ----------

const GRAVITY: f32 = -1300.0; // downward acceleration in px/s^2
const CHARGE_TIME: f32 = 1.2; // seconds of holding to reach full power
const MIN_SHOT_SPEED: f32 = 500.0; // launch speed at zero charge
const MAX_SHOT_SPEED: f32 = 2200.0; // launch speed at full charge
```

Estos cuatro números *son* la dificultad del juego. La gravedad es negativa porque +y es arriba; 1.2 segundos hasta la carga completa convierte la potencia en una habilidad, no en un clic; el rango de velocidades decide si un toque perezoso avanza a trompicones o un mantenido completo cruza la cancha como un cohete. Al final del capítulo, tócalos y siente cómo cambia el juego.

## Paso 2 — La pelota recibe datos de verdad

En el Capítulo 4, `Ball` era un marcador vacío. Ahora lleva estado:

```rust
/// The ball is either resting (shootable) or in the air.
#[derive(PartialEq)]
enum BallState {
    Idle,
    Flying,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    state: BallState,
    // Position before this frame's move — scoring will need it in Chapter 10.
    prev_pos: Vec2,
}
```

Y el spawn en `setup` lo rellena:

```rust
        Ball {
            velocity: Vec2::ZERO,
            state: BallState::Idle,
            prev_pos: START,
        },
```

> [!NOTE]
> **Sidebar de Rust: enums.** Un `enum` es un tipo cuyo valor es exactamente uno de una lista fija de *variantes* con nombre — aquí, una pelota está `Idle` o `Flying`, nunca ambas, nunca ninguna. Podríamos haber usado un booleano (`is_flying`), pero el enum *nombra el concepto*: el código que lee `ball.state == BallState::Idle` se explica solo. El `#[derive(PartialEq)]` nos regala esa comparación con `==` — como `Component` en el Capítulo 4, es un trait derivado. Los enums de Rust son una de las mejores características del lenguaje (pueden incluso llevar datos dentro de cada variante), y el código Bevy se apoya en ellos por todas partes.

Una pieza más de estado — la carga — pertenece *al juego*, no a ninguna entidad. Eso la convierte en un recurso:

```rust
// While aiming: how long the mouse has been held (the charge), capped at CHARGE_TIME.
#[derive(Resource, Default)]
struct Aim {
    active: bool,
    charge: f32,
}
```

Regístralo en `main()` con `.init_resource::<Aim>()` — "crea este recurso usando su `Default` (todo ceros/false)." Ese es el segundo derive: `Default` escribe el constructor "vacío" por ti.

## Paso 3 — ¿Dónde está el ratón, en coordenadas del *mundo*?

La ventana reporta el cursor en **píxeles de pantalla** (origen arriba-izquierda, y hacia abajo). Nuestro juego vive en **coordenadas de mundo** (origen en el centro, y hacia arriba, y — desde la cámara `AutoMin` del Capítulo 7 — posiblemente escalado). La cámara sabe convertir:

```rust
/// Where is the mouse cursor, in world coordinates?
fn cursor_world(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let (camera, cam_tf) = cameras.single().ok()?;
    let cursor = window.cursor_position()?;
    camera.viewport_to_world_2d(cam_tf, cursor).ok()
}
```

Fíjate en que es una función auxiliar, no un sistema — los sistemas la llamarán. Y su tipo de retorno presenta a una celebridad de Rust:

> [!NOTE]
> **Sidebar de Rust: `Option` y el operador `?`.** Muchos valores pueden legítimamente *no existir*: la posición del cursor (el ratón puede estar fuera de la ventana), la propia ventana (podría estar cerrándose). Rust no tiene `null` — en su lugar, "quizá un valor" es un tipo propio: `Option<Vec2>` es o `Some(posición)` o `None`, y el compilador obliga a quien lo reciba a manejar ambos casos. **En Rust no puedes olvidar el chequeo de null — no compila.**
>
> El `?` tras cada llamada es el operador de salida temprana: "si esto fue `None`, para aquí y devuelve `None`; si no, desenvuelve y continúa." Cuatro pasos falibles se leen como una línea recta. (`.ok()` convierte `Result` — un primo de `Option` para operaciones que reportan errores — en un `Option` para que `?` funcione uniformemente.)

Una segunda función auxiliar convierte la posición del cursor en dirección de lanzamiento:

```rust
// Launch goes from the ball toward the cursor; if the cursor is on the ball, default up-right.
fn aim_dir(ball: Vec2, cursor: Vec2) -> Vec2 {
    let d = cursor - ball;
    if d.length() < 1.0 {
        Vec2::new(0.7, 0.7).normalize()
    } else {
        d.normalize()
    }
}
```

`normalize()` encoge un vector a longitud 1 — dirección pura, sin magnitud. Dirección y potencia se mantienen independientes: el cursor apunta, la carga decide la velocidad.

## Paso 4 — El corazón: `aim_and_launch`

Un solo sistema implementa la mecánica completa. Lee primero la forma, los detalles después:

```rust
fn aim_and_launch(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut aim: ResMut<Aim>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut gizmos: Gizmos,
) {
    let Ok((mut ball, mut tf)) = balls.single_mut() else {
        return;
    };

    // R re-spots the ball at the start line.
    if keys.just_pressed(KeyCode::KeyR) {
        reset(&mut ball, &mut tf);
        aim.active = false;
        aim.charge = 0.0;
        return;
    }

    let ball_pos = tf.translation.truncate();
    let Some(cursor) = cursor_world(&windows, &cameras) else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) && ball.state == BallState::Idle {
        aim.active = true;
        aim.charge = 0.0;
    }
    if !aim.active {
        return;
    }

    if mouse.pressed(MouseButton::Left) {
        aim.charge = (aim.charge + time.delta_secs()).min(CHARGE_TIME);
        let power = aim.charge / CHARGE_TIME;
        let speed = MIN_SHOT_SPEED + (MAX_SHOT_SPEED - MIN_SHOT_SPEED) * power;
        let launch = aim_dir(ball_pos, cursor) * speed;
        draw_power_bar(&mut gizmos, ball_pos, power);
        draw_trajectory(&mut gizmos, ball_pos, launch);
    }

    if mouse.just_released(MouseButton::Left) {
        let power = aim.charge / CHARGE_TIME;
        let speed = MIN_SHOT_SPEED + (MAX_SHOT_SPEED - MIN_SHOT_SPEED) * power;
        ball.velocity = aim_dir(ball_pos, cursor) * speed;
        ball.state = BallState::Flying;
        ball.prev_pos = ball_pos;
        aim.active = false;
        aim.charge = 0.0;
    }
}
```

La API de input es un modelo de tres fases, y es la clave de toda la mecánica:

| Método | Verdadero... | Se usa para |
|---|---|---|
| `just_pressed` | solo el frame en que el botón bajó | *empezar* la carga |
| `pressed` | cada frame mientras se mantiene | *acumular* la carga, dibujar la vista previa |
| `just_released` | solo el frame en que subió | *disparar* el tiro |

Sigue un tiro de principio a fin: pulsar sobre una pelota `Idle` → `aim.active`, carga a cero. Cada frame mantenido → la carga crece con el delta time (con tope en `CHARGE_TIME`), y `power` (0.0–1.0) fija la velocidad de salida por interpolación lineal entre mínimo y máximo. Soltar → la `velocity` de la pelota se vuelve dirección × velocidad, su estado pasa a `Flying`, el apuntado se reinicia. Ocho líneas de *mecanismo*, gobernadas por completo desde el bloque de constantes.

También nuevo aquí: `let Ok(...) = ... else { return; }` y `let Some(...) = ... else { return; }` — el patrón *let-else*, "desempaqueta esto o abandona," la manera ordenada de Rust de blindar una función. Y `truncate()` descarta la z de la translación 3D: las físicas piensan en 2D.

La función auxiliar `reset` recoloca la pelota:

```rust
fn reset(ball: &mut Ball, tf: &mut Transform) {
    ball.velocity = Vec2::ZERO;
    ball.state = BallState::Idle;
    ball.prev_pos = START;
    tf.translation = START.extend(1.0);
}
```

## Paso 5 — La vista previa: barra de potencia y trayectoria

Ambas son gizmos — redibujadas solo en los frames en los que estás cargando:

```rust
// A power meter above the ball that fills and shifts green -> red as it charges.
fn draw_power_bar(gizmos: &mut Gizmos, ball_pos: Vec2, power: f32) {
    let w = 110.0;
    let base = ball_pos + Vec2::new(-w / 2.0, BALL_R + 22.0);
    let bg = Color::srgba(1.0, 1.0, 1.0, 0.25);
    let fill = Color::srgb(0.2 + 0.8 * power, 1.0 - 0.7 * power, 0.2);
    for o in 0..8 {
        let y = o as f32;
        gizmos.line_2d(base + Vec2::new(0.0, y), base + Vec2::new(w, y), bg);
        gizmos.line_2d(base + Vec2::new(0.0, y), base + Vec2::new(w * power, y), fill);
    }
}

// A small "+" so the predicted path reads as distinct dots, not a faint line.
fn dot(gizmos: &mut Gizmos, p: Vec2, color: Color) {
    gizmos.line_2d(p - Vec2::X * 3.5, p + Vec2::X * 3.5, color);
    gizmos.line_2d(p - Vec2::Y * 3.5, p + Vec2::Y * 3.5, color);
}

fn draw_trajectory(gizmos: &mut Gizmos, start: Vec2, vel: Vec2) {
    let dt = 1.0 / 60.0;
    let mut p = start;
    let mut v = vel;
    let color = Color::srgba(1.0, 0.95, 0.3, 0.95);
    for i in 0..150 {
        if i % 6 == 0 {
            dot(gizmos, p, color);
        }
        p += v * dt;
        v.y += GRAVITY * dt;
        if p.y < GROUND_Y + BALL_R {
            break;
        }
    }
}
```

La barra de potencia son ocho líneas apiladas (los gizmos no tienen grosor, así que lo simulamos), con el color del relleno deslizándose de verde a rojo según sube `power`. La trayectoria es la parte ingeniosa: **ejecuta las físicas del vuelo por adelantado** — misma gravedad, mismo paso de tiempo — dibujando una de cada seis posiciones como un `+`. La vista previa nunca miente, porque son las mismas matemáticas que usará el vuelo real. Que son…

## Paso 6 — La gravedad

```rust
/// Gravity pulls the velocity down; the velocity moves the ball.
fn physics(time: Res<Time>, mut balls: Query<(&mut Ball, &mut Transform)>) {
    let dt = time.delta_secs();
    for (mut ball, mut tf) in &mut balls {
        if ball.state != BallState::Flying {
            continue;
        }
        ball.prev_pos = tf.translation.truncate();
        ball.velocity.y += GRAVITY * dt;
        let step = ball.velocity * dt;
        tf.translation.x += step.x;
        tf.translation.y += step.y;
    }
}
```

Dos líneas de física, dignas de leerse en voz alta: **la gravedad cambia la velocidad; la velocidad cambia la posición** — cada una escalada por el delta time (la regla de oro del Capítulo 5). Eso es movimiento de proyectil, las mismas matemáticas que la luna y las balas de cañón. El resto es contabilidad: solo las pelotas `Flying` se mueven, y `prev_pos` recuerda dónde estaba la pelota (el Capítulo 10 lo necesitará para detectar el cruce del aro).

Registra ambos sistemas con garantía de orden:

```rust
        .add_systems(Update, (aim_and_launch, physics).chain())
```

`.chain()` significa "ejecuta estos en este orden exacto." Sin él, Bevy puede ejecutarlos en paralelo o en cualquier orden (ese es el superpoder del borrow checker del Capítulo 5) — pero un lanzamiento debe ser *visto* por las físicas en el mismo frame, así que aquí el orden importa.

## Ejecútalo

```
trunk serve        (o: cargo run)
```

Mantén el ratón sobre la pelota, arrastra hacia el aro, mira cómo se llena la barra y se extiende el arco:

![Cargando un tiro: barra de potencia llena y arco de trayectoria punteado](../../assets/ch08-charge.png)

Suelta:

![La pelota en pleno vuelo sobre el aro](../../assets/ch08-flight.png)

Y entonces… la pelota atraviesa el aro, atraviesa el suelo, y cae fuera del mundo para siempre. (Pulsa **R**.) Nada en el mundo la frena todavía — el tablero es un dibujo, el suelo es un dibujo. **Hacerlos sólidos es el Capítulo 9.**

## Experimentos antes de continuar

1. `CHARGE_TIME` a `0.3` — un tiro arcade nervioso. A `3.0` — un swing de golf.
2. `GRAVITY` a `-300.0` — baloncesto lunar. La vista previa de trayectoria se ajusta automáticamente (¡mismas matemáticas!).
3. Mantén un tiro y mueve el cursor: la dirección se actualiza en vivo mientras cargas, exactamente como promete `aim_dir`.

## Qué construiste / Qué sigue

Una mecánica de tiro completa y ajustable: input de tres fases, la carga como recurso, conversión pantalla-a-mundo, una vista previa honesta de la trayectoria, y gravedad — el mismo código, línea por línea, que el juego terminado.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/08-the-shooting-mechanic/`](.).

En el **Capítulo 9**, el mundo se vuelve sólido: el suelo rebota, el tablero devuelve los banks, el aro rechaza — detección de colisiones, reflexión, restitución y fricción.

**[Continuar al Capítulo 9: Físicas →](../09-physics/README.es.md)**
