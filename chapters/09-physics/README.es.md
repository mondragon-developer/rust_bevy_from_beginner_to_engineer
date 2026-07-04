# Capítulo 9 — Físicas: el mundo responde

*Léelo en: [English](README.md) | **Español***

Al final del Capítulo 8, tu pelota volaba un arco de gravedad perfecto — que atravesaba el aro, el suelo, y desaparecía de la existencia. En este capítulo la cancha se vuelve *sólida*: el tablero devuelve los banks, el aro rechaza los tiros que casi entran, las paredes contienen, y el suelo rebota, luego rueda, y finalmente deja la pelota en reposo — lista para tu siguiente tiro. Esto cierra el bucle de gameplay: tirar, mirar, recuperar, tirar otra vez.

**Tiempo**: ~1 hora.

## La idea: mover primero, corregir después

Nuestro enfoque de colisiones es el que usan de verdad la mayoría de juegos 2D:

1. `physics` mueve la pelota como si no hubiera nada en medio (Capítulo 8).
2. `collisions` corre *justo después* y comprueba: ¿acabó la pelota dentro de algo? Si es así, **la empuja hacia afuera** y **cambia su velocidad** a lo que un rebote habría producido.

Por eso la cadena de sistemas crece un eslabón, y por eso el orden es ley:

```rust
        .add_systems(Update, (aim_and_launch, physics, collisions).chain())
```

## Paso 1 — Cinco constantes nuevas: las propiedades de los materiales

```rust
const RESTITUTION: f32 = 0.6; // fraction of speed kept after a bounce
const GROUND_FRICTION: f32 = 0.75; // horizontal loss on each hard floor bounce
const ROLL_FRICTION: f32 = 2.5; // per-second slowdown while rolling on the floor
const BOUNCE_THRESHOLD: f32 = 160.0; // |vy| above this = real bounce, below = rest/roll
const STOP_SPEED: f32 = 30.0; // ball fully stops below this horizontal speed
```

Si las constantes del Capítulo 8 eran la *dificultad* del juego, estas son sus *materiales*. `RESTITUTION` (restitución) es el término clásico de la física: una pelota que conserva el 60% de su velocidad por rebote se siente como cuero sobre parqué; 95% es una superball; 10% es un saco de arena. Las otras cuatro gobiernan el comportamiento especial del suelo — verás a cada una hacer su trabajo abajo.

## Paso 2 — El sistema de colisiones, pared por pared

Esta es la función más grande del curso. Léela en sus cinco secciones:

```rust
/// The world pushes back: bank off the backboard, bounce off the rim,
/// stay inside the walls, and bounce / roll / rest on the floor.
fn collisions(time: Res<Time>, mut balls: Query<(&mut Ball, &mut Transform)>) {
    let dt = time.delta_secs();
    let half_w = WORLD_W / 2.0;
    let half_h = WORLD_H / 2.0;

    for (mut ball, mut tf) in &mut balls {
        if ball.state != BallState::Flying {
            continue;
        }
        let mut pos = tf.translation.truncate();
```

Copiamos la posición a una variable local `pos`, la corregimos con libertad, y la escribimos de vuelta una sola vez al final — más simple que toquetear `tf.translation` cinco veces.

### El tablero: una pared direccional

```rust
        // Bank off the front face of the backboard.
        if ball.velocity.x > 0.0
            && pos.x + BALL_R > BACKBOARD_FRONT
            && pos.x < BACKBOARD_X
            && pos.y < BACKBOARD_Y + BACKBOARD_H / 2.0
            && pos.y > BACKBOARD_Y - BACKBOARD_H / 2.0
        {
            pos.x = BACKBOARD_FRONT - BALL_R;
            ball.velocity.x = -ball.velocity.x * RESTITUTION;
        }
```

Cinco condiciones, cada una ganándose su sitio: la pelota se mueve hacia la derecha (`velocity.x > 0.0` — no puedes golpear el *frente* de un tablero alejándote de él), su borde derecho ha cruzado la cara frontal del tablero, no lo ha atravesado hasta pasar su centro, y está verticalmente dentro del tablero. La respuesta es el patrón que verás en cada sección: **saca la posición del objeto de golpe, invierte y amortigua la velocidad.** ¿Recuerdas `BACKBOARD_FRONT` de las constantes del Capítulo 7? Este es el momento en que el código que dibuja y el código de físicas demuestran estar de acuerdo sobre dónde está el tablero.

### El aro: un obstáculo redondo

```rust
        // Bounce off the front rim lip on a near miss.
        let rim_point = Vec2::new(RIM_FRONT_X, RIM_Y);
        let to_ball = pos - rim_point;
        if to_ball.length() < BALL_R {
            let n = to_ball.normalize_or_zero();
            pos = rim_point + n * BALL_R;
            ball.velocity = reflect(ball.velocity, n) * RESTITUTION;
        }
```

El labio frontal del aro se trata como un *punto*. Si el centro de la pelota se acerca a menos de un radio de él, se están tocando. Pero a diferencia de una pared plana, una colisión redonda puede empujar la pelota en *cualquier* dirección — así que el rebote necesita matemáticas de verdad:

```rust
/// Mirror a velocity across a surface normal (the classic bounce formula).
fn reflect(v: Vec2, n: Vec2) -> Vec2 {
    v - 2.0 * v.dot(n) * n
}
```

> [!NOTE]
> **Sidebar de matemáticas: la fórmula de reflexión.** `n` es la *normal* — la dirección desde el punto del aro hacia la pelota, es decir, "directamente lejos de la superficie." `v.dot(n)` (el producto escalar) mide cuánta de la velocidad apunta *hacia dentro* de la superficie. Restar ese componente dos veces lo invierte: entra en un ángulo, sale en el ángulo espejado — un rebote de billar. Esta línea está en todos los motores de juego jamás escritos; ahora la has escrito tú. Es exactamente la razón por la que una pelota que roza el labio del aro se desvía hacia arriba y afuera en vez de simplemente invertirse.

### Paredes y techo: la caja invisible

```rust
        // Side walls keep the ball wandering inside the court.
        if pos.x - BALL_R < -half_w {
            pos.x = -half_w + BALL_R;
            ball.velocity.x = ball.velocity.x.abs() * RESTITUTION;
        }
        if pos.x + BALL_R > half_w {
            pos.x = half_w - BALL_R;
            ball.velocity.x = -ball.velocity.x.abs() * RESTITUTION;
        }
        // Ceiling.
        if pos.y + BALL_R > half_h {
            pos.y = half_h - BALL_R;
            ball.velocity.y = -ball.velocity.y.abs() * RESTITUTION;
        }
```

Un detalle sutil de oficio: la pared izquierda pone la velocidad en `.abs()` (definitivamente hacia la derecha) en vez de negarla. Si un bug o un frame raro dejara alguna vez la pelota *ya* solapada con la pared mientras se aleja, la negación la atraparía — devolviéndola hacia la pared para siempre. `abs` declara el *resultado* ("ahora te estás alejando") en vez de la *operación* ("invierte"). Un seguro barato, y un hábito que vale la pena robar.

### El suelo: rebota, rueda, descansa

El suelo es más rico que una pared, porque los balones de baloncesto no rebotan para siempre — rebotan, luego ruedan, luego se paran:

```rust
        // Floor: bounce while losing energy, then roll, then come to rest in place.
        if pos.y - BALL_R <= GROUND_Y {
            pos.y = GROUND_Y + BALL_R;
            if ball.velocity.y < -BOUNCE_THRESHOLD {
                ball.velocity.y = -ball.velocity.y * RESTITUTION;
                ball.velocity.x *= GROUND_FRICTION;
            } else {
                ball.velocity.y = 0.0;
                ball.velocity.x *= (1.0 - ROLL_FRICTION * dt).max(0.0);
                if ball.velocity.x.abs() < STOP_SPEED {
                    ball.velocity = Vec2::ZERO;
                    ball.state = BallState::Idle;
                }
            }
        }

        tf.translation.x = pos.x;
        tf.translation.y = pos.y;
    }
}
```

`BOUNCE_THRESHOLD` es la bifurcación del camino. ¿Cae rápido (`velocity.y < -160`)? Rebote de verdad: invierte la y con restitución, recorta la x con `GROUND_FRICTION`. Cada rebote alcanza el 60% de la altura del anterior, así que la pelota cruza *naturalmente* por debajo del umbral tras unos pocos — y entra en rodadura: la y clavada a cero, la x decayendo suavemente por segundo (`ROLL_FRICTION * dt` — delta time otra vez). Y cuando la rodadura cae por debajo de `STOP_SPEED`, el remate:

**`ball.state = BallState::Idle;`**

La pelota vuelve a ser lanzable, allá donde se detuvo. Sin reset, sin reaparición — la máquina de estados del Capítulo 8 acaba de cerrar el bucle. Tira desde donde quedó, o pulsa R para devolverla al punto de tiro libre.

## Ejecútalo

```
trunk serve        (o: cargo run)
```

Tira. La pelota hace bank en el tablero, besa el aro, bota en el parqué en saltos cada vez más pequeños, rueda, se para, y te espera:

![La pelota cayendo a través del aro](../../assets/ch09-bounce.png)

Esa captura es del tiro que lanzamos probando este capítulo — un bank en el tablero que cayó limpio a través del aro. Lo cual expone lo único que falta: *el juego no se enteró*. Ni punto, ni celebración, nada. La pelota cayó a través de un aro que no sabe que es una canasta.

## Experimentos antes de continuar

1. `RESTITUTION` a `0.95` — superball de patio; mírala traquetear por la cancha una eternidad.
2. `BOUNCE_THRESHOLD` a `2000.0` — la pelota nunca rebota, solo golpea seco y rueda. Siente cómo una constante cambia el material.
3. `GROUND_FRICTION` a `0.99` — suelo helado; rueda sin parar. Fíjate en qué constante la detiene de todas formas (`STOP_SPEED`).
4. Apunta recto hacia arriba a plena potencia. Rebote en el techo, botes menguantes en el suelo, rodadura, reposo, Idle. Todo el sistema de materiales en un solo tiro.

## Qué construiste / Qué sigue

Un sistema completo de respuesta física 2D: paredes direccionales, un obstáculo circular con reflexión de verdad, y un suelo con tres regímenes (rebote → rodadura → reposo) — todo gobernado por cinco constantes de material con nombre, todo de acuerdo con el código de dibujo porque ambos leen el mismo plano.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/09-physics/`](.).

En el **Capítulo 10**, el juego aprende a *darse cuenta*: detectar la pelota cruzando el aro hacia abajo, llevar la puntuación en un recurso, poner texto en pantalla, y celebrar con un destello verde.

**[Continuar al Capítulo 10: Puntuación y feedback →](../10-scoring-and-feedback/README.es.md)**
