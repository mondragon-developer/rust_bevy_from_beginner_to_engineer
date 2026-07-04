# Capítulo 12 — Hablando con la página web

*Léelo en: [English](README.md) | **Español***

**Empieza la Parte IV: de programador de juegos a ingeniero.** Tu juego corre *dentro* de una página web — pero la página ha sido un marco pasivo. En este capítulo se convierte en parte del producto: un panel de control HTML que fija el límite de tiros, muestra la puntuación en vivo, guarda resultados con nombre en un desplegable, y arranca partidas nuevas — todo hablando con tu código Rust a través de la frontera WASM. Y cuando termines, algo sobre lo que vale la pena detenerse: **tu `main.rs` coincidirá con el del juego terminado, línea por línea.**

**Tiempo**: ~1.5 horas.

## La arquitectura: un objeto compartido

Rust y JavaScript viven en mundos distintos. Nuestro puente entre ellos es deliberadamente primitivo: **un objeto JavaScript plano, `window.rustbyve`**, que ambos lados leen y escriben:

| Campo | Lo escribe | Lo lee | Significado |
|---|---|---|---|
| `shotLimit` | el panel | Rust, cada frame | Máximo de tiros (0 = ilimitado) |
| `resetRequested` | el panel (botón) | Rust, tomar-y-limpiar | "Por favor, partida nueva" |
| `score`, `attempts`, `gameOver` | Rust, cada frame | el panel | Estado del juego en vivo |

Sin callbacks hacia WASM, sin colas de mensajes, sin trucos de memoria compartida. El panel deja notas; el juego las lee cada frame, y escribe las suyas. Es humilde — y es robusto, depurable (abre la consola del navegador y escribe `window.rustbyve`), y totalmente desacoplado: cada lado funciona sin el otro.

## Paso 1 — El módulo bridge

Esta es la pieza central del capítulo — un módulo nuevo al principio de `main.rs`:

```rust
// Bridge to the HTML control panel. The two sides share a single `window.rustbyve`
// object: Rust publishes score/attempts/game-over out; the panel writes the shot
// limit and a one-shot reset request back in. On non-wasm builds these are no-ops
// so the crate still `cargo check`s on the host.
#[cfg(target_arch = "wasm32")]
mod bridge {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        function rb_ensure() {
          if (!window.rustbyve) {
            window.rustbyve = { resetRequested: false, shotLimit: 0, score: 0, attempts: 0, gameOver: false };
          }
          return window.rustbyve;
        }
        // Reads and clears the reset flag in one call so a click can't be missed or double-counted.
        export function rb_take_reset() {
          const s = rb_ensure();
          const v = s.resetRequested;
          s.resetRequested = false;
          return v;
        }
        export function rb_shot_limit() { return rb_ensure().shotLimit | 0; }
        export function rb_publish(score, attempts, gameOver) {
          const s = rb_ensure();
          s.score = score | 0;
          s.attempts = attempts | 0;
          s.gameOver = !!gameOver;
          if (typeof window.rbOnState === "function") window.rbOnState(s);
        }
    "#)]
    extern "C" {
        pub fn rb_take_reset() -> bool;
        pub fn rb_shot_limit() -> i32;
        pub fn rb_publish(score: i32, attempts: i32, game_over: bool);
    }
}
```

Tres ideas nuevas, de fuera hacia dentro:

- **`#[cfg(target_arch = "wasm32")]`** es *compilación condicional*: este módulo solo existe en las compilaciones WASM. Las compilaciones nativas nunca lo ven — no es "saltado en tiempo de ejecución", es que literalmente no se compila.
- **`extern "C" { ... }`** declara funciones que existen *en otro lugar* — Rust aprende sus firmas y confía en el enlazador. Combinado con `#[wasm_bindgen]`, "otro lugar" significa JavaScript.
- **`inline_js`** lleva el JavaScript real dentro del archivo Rust. Lee esas tres funciones JS — son el protocolo completo. Y estudia `rb_take_reset`: lee **y limpia** el flag en una sola llamada. Si Rust leyera el flag y lo limpiara por separado, un clic podría contarse doble (o perderse) entre los dos pasos. *Tomar-y-limpiar* es como se entregan eventos de un solo uso a través de cualquier frontera.

## Paso 2 — Los stubs lo hacen portable

Al código del juego no debería importarle en qué plataforma está, así que le damos un solo conjunto de funciones con dos implementaciones:

```rust
#[cfg(target_arch = "wasm32")]
fn js_take_reset() -> bool {
    bridge::rb_take_reset()
}
#[cfg(target_arch = "wasm32")]
fn js_shot_limit() -> u32 {
    bridge::rb_shot_limit().max(0) as u32
}
#[cfg(target_arch = "wasm32")]
fn js_publish(score: u32, attempts: u32, game_over: bool) {
    bridge::rb_publish(score as i32, attempts as i32, game_over);
}

#[cfg(not(target_arch = "wasm32"))]
fn js_take_reset() -> bool {
    false
}
#[cfg(not(target_arch = "wasm32"))]
fn js_shot_limit() -> u32 {
    0
}
#[cfg(not(target_arch = "wasm32"))]
fn js_publish(_score: u32, _attempts: u32, _game_over: bool) {}
```

En la web, `js_shot_limit()` le pregunta al panel. En escritorio, devuelve 0 — modo práctica ilimitado, sin panel, y `cargo run` sigue funcionando. Este *patrón de stubs* es como se escribe el código multiplataforma de verdad: la diferencia de plataforma queda contenida en seis funciones diminutas, y todo lo demás es ciego a la plataforma.

## Paso 3 — Dos sistemas de sincronización

El puente lo usan dos sistemas nuevos que flanquean la lógica del juego:

```rust
// Pull the shot limit from the panel every frame, and when the panel's Save & Reset
// fires, wipe the score/attempts and re-spot the ball for a brand-new game.
fn sync_from_js(
    mut score: ResMut<Score>,
    mut attempts: ResMut<Attempts>,
    mut stopped: ResMut<Stopped>,
    mut limit: ResMut<ShotLimit>,
    mut aim: ResMut<Aim>,
    mut flash: ResMut<ScoreFlash>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
) {
    let new_limit = js_shot_limit();
    if limit.0 != new_limit {
        limit.0 = new_limit;
    }

    if js_take_reset() {
        score.0 = 0;
        attempts.0 = 0;
        stopped.0 = false;
        flash.0 = 0.0;
        aim.active = false;
        aim.charge = 0.0;
        if let Ok((mut ball, mut tf)) = balls.single_mut() {
            reset(&mut ball, &mut tf);
        }
    }
}

// Push the live game state to the panel so it can show progress and, on Save & Reset,
// read the final score/attempts for the record it stores.
fn sync_to_js(score: Res<Score>, attempts: Res<Attempts>, stopped: Res<Stopped>) {
    js_publish(score.0, attempts.0, stopped.0);
}
```

Mira de cerca el bloque de reset de `sync_from_js` — es el `new_game` del Capítulo 11, literal. El *qué* de un reinicio de sesión no cambió; solo se movió el *disparador*, de la tecla N al botón del panel. Por eso la cadena se convierte en:

```rust
        .add_systems(
            Update,
            (sync_from_js, aim_and_launch, physics, collisions, sync_to_js).chain(),
        )
```

Estado entrante primero, lógica del juego en medio, estado saliente al final — una forma de bucle de frame que reconocerás en cada sistema en red o embebido que toques en tu vida. (El sistema `new_game` y su tecla N se borran; `ShotLimit` vuelve a `#[derive(Default)]` — 0 — porque el panel es ahora la fuente de verdad. La tecla `R` se queda: recolocar la pelota es gameplay, no gestión de sesión.)

Dos cambios pequeños rematan el lado Rust: la configuración de `Window` se vuelve nativa de la web —

```rust
            primary_window: Some(Window {
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
```

— que significa "renderiza en el `<canvas id="bevy">` de la página y ajústate a su tamaño" (la página es dueña del layout ahora; `AutoMin` mantiene la cancha encuadrada dentro de la forma que sea).

## Paso 4 — El panel en sí

El `index.html` completo de este capítulo está en [la carpeta del capítulo](index.html) — son ~230 líneas y todas son desarrollo web corriente: un `<div>` flotante con un checkbox y un input numérico para el límite, una línea de estado, un campo de nombre, un botón Save & Reset, un `<select>` de resultados, y CSS para que se sienta parte del juego. Cópialo, y lee el `<script>` del final contra la tabla del principio de esta lección:

- **Crea `window.rustbyve` primero** (antes de que cargue el WASM — quien llega primero lo crea, el otro lo reutiliza; el `rb_ensure` del lado Rust es la misma jugada defensiva).
- `applyLimit()` refleja el checkbox/número en `rb.shotLimit` — incluida la convención `0 = ilimitado` del Capítulo 11, ahora revelada como el idioma nativo del panel.
- `window.rbOnState` es llamado por `rb_publish` cada frame — reescribe la línea de estado y la pone en verde al terminar la partida.
- El botón Save & Reset guarda `{name, made, shots}` en el desplegable (auto-nombrando las entradas en blanco), y pone `rb.resetRequested = true` — la nota que Rust tomará-y-limpiará en su próximo frame.

Un detalle de Trunk cambió en el `<head>`: `<link data-trunk rel="rust" data-wasm-opt="z" />` — el atributo nuevo le pide a Trunk que pase el optimizador de tamaño `wasm-opt` en las compilaciones release. Es una semilla plantada para el Capítulo 14.

## Ejecútalo

```
trunk serve
```

![El juego con su panel de control: límite de tiros, estado en vivo, campo de nombre y desplegable de resultados](../../assets/ch12-panel.png)

Juega una sesión de 10 tiros. Mira el estado del panel avanzar en sincronía con el HUD (mismo estado, dos pantallas, una fuente de verdad). Alcanza el límite — la línea del panel se pone verde: *Game over — 3 made in 10*. Escribe tu nombre, pulsa **Save & Reset**: tu partida aparece en el desplegable y empieza una nueva. Cambia el límite a mitad de partida; el `x/10` del HUD se convierte en `x/5` al instante. Después abre la consola del navegador y escribe `window.rustbyve` — ahí está tu protocolo entero, en vivo.

Y con eso: **compara tu `main.rs` con el del juego terminado — son el mismo archivo.** Todo lo que viste en la captura del Capítulo 0, ya lo has escrito.

## Experimentos antes de continuar

1. En la consola: `window.rustbyve.resetRequested = true` — puedes manejar el juego desde JavaScript a mano. Ese es todo el puente, desmitificado.
2. Añade un campo: publica la velocidad actual de la pelota vía `rb_publish` (un argumento extra a través del puente) y muéstrala en vivo en el panel. Tocar las cuatro capas — llamada Rust, pegamento JS, objeto compartido, DOM del panel — una vez, a propósito, es la mejor forma de hacer tuyo el patrón.
3. `cargo run` en el escritorio: sin panel, tiros ilimitados, todo lo demás intacto. Los stubs trabajando.

## Qué construiste / Qué sigue

Un puente Rust↔JavaScript de verdad: compilación condicional, externs de `wasm-bindgen` con JS inline, el protocolo del objeto compartido con semántica de tomar-y-limpiar, y sistemas de sincronización de entrada/salida flanqueando el frame — más un juego terminado, controlado por panel, idéntico a la implementación de referencia.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/12-talking-to-the-web-page/`](.).

En el **Capítulo 13**, desmontamos este juego de un solo archivo y lo volvemos a montar como lo publicaría un equipo: módulos, plugins y un workspace.

**[Continuar al Capítulo 13: Refactorizando como ingeniero →](../13-refactoring-like-an-engineer/README.es.md)**
