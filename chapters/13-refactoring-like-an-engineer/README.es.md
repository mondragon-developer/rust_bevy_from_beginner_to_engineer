# Capítulo 13 — Refactorizando como ingeniero

*Léelo en: [English](README.md) | **Español***

Tu juego funciona y vive en un `main.rs` de 600 líneas. No hay nada de malo en eso — para un prototipo en solitario. Pero abre ese archivo e intenta responder "¿dónde está todo lo relacionado con la puntuación?" — está untado entre las constantes de arriba, un recurso en medio, unas líneas dentro de `collisions`, y un sistema de texto al final. En este capítulo desmontas el juego y lo vuelves a montar para que cada pregunta así tenga una respuesta de una palabra. **Ni una línea de comportamiento cambia.** Esa es la definición de refactorizar — y hacerlo sin miedo es la habilidad con más sabor a ingeniero de este curso.

**Tiempo**: ~1.5 horas.

## La estructura objetivo

```
src/
├── main.rs         ← 50 líneas: construye la App, conecta los plugins
├── constants.rs    ← todos los números del juego
├── components.rs   ← Ball, BallState, ScoreText
├── resources.rs    ← Score, Attempts, Stopped, ShotLimit, ScoreFlash, Aim
├── bridge.rs       ← el puente wasm-bindgen y sus stubs nativos
├── court.rs        ← CourtPlugin: cámara, geometría, entidades del HUD
├── shooting.rs     ← ShootingPlugin: carga, apuntado, lanzamiento, vistas previas
├── physics.rs      ← PhysicsPlugin: gravedad, colisiones, detección de canasta
├── feedback.rs     ← FeedbackPlugin: red, explosión de swish, texto del HUD
└── session.rs      ← SessionPlugin: límites de tiros y sincronización con el panel
```

Vuelve a leer la columna derecha: es el *índice del curso*. El Capítulo 7 se convirtió en `court.rs`, el 8 en `shooting.rs`, los 9–10 en `physics.rs` y `feedback.rs`, los 11–12 en `session.rs`. Una base de código bien factorizada cuenta la historia de su propia construcción.

## Paso 1 — Módulos: archivos que se conocen

El sistema de módulos de Rust es refrescantemente literal: **un archivo es un módulo.** Decláralos en `main.rs`:

```rust
mod bridge;
mod components;
mod constants;
mod court;
mod feedback;
mod physics;
mod resources;
mod session;
mod shooting;
```

Después mueve cada pieza del viejo `main.rs` a su archivo. Dos reglas mecánicas cubren casi todo:

1. **Todo lo que se use desde fuera de su archivo necesita `pub`.** `const BALL_R` pasa a `pub const BALL_R`; `struct Ball { velocity: ... }` pasa a `pub struct Ball { pub velocity: ... }` — los campos necesitan su propio `pub`. El compilador te listará cada infracción; arreglarlas es un tour guiado por tus propias dependencias.
2. **Cada uso lleva una ruta.** Dentro de `physics.rs`, las constantes viven en `crate::constants::*`, la pelota en `crate::components::Ball` (`crate::` = "desde la raíz de mi proyecto"). Cada archivo empieza con un bloque pequeño de líneas `use` que *documenta exactamente de qué depende* — información que la versión de un solo archivo mantenía en secreto.

Al mover el puente a `bridge.rs`, mejora su API gratis: las funciones pasan a ser `bridge::take_reset()`, `bridge::shot_limit()`, `bridge::publish()` — el nombre del módulo reemplaza al prefijo `js_`. Quien las llama lee mejor y el patrón de stubs no cambia.

## Paso 2 — Plugins: módulos que se instalan solos

Un módulo contiene código; un **plugin** es la manera de Bevy de dejar que ese código *se registre a sí mismo*. Este es el de `physics.rs`:

```rust
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (physics, collisions).chain().in_set(GameSet::Physics),
        );
    }
}
```

Otra implementación de trait escrita a mano (el Capítulo 11 fue la primera): `Plugin` tiene un método obligatorio, `build`, que recibe la `App` y le hace exactamente lo que antes hacía `main()`. Cada plugin trae *todo lo que su función necesita* — `ShootingPlugin` inicializa el recurso `Aim`, `FeedbackPlugin` trae `ScoreFlash`, `SessionPlugin` trae los cuatro recursos de sesión y ambos sistemas de sincronización. Borra un plugin de `main()` y su función entera desaparece limpiamente; esa es la prueba de una buena costura.

¿Recuerdas el Capítulo 3, cuando `DefaultPlugins` instaló ventanas, renderizado e input? Ahora estás al otro lado de esa API. El propio motor de Bevy está organizado exactamente como tu juego ahora.

## Paso 3 — System sets: orden a través de las fronteras de plugins

La refactorización rompe una cosa, y es la instructiva. El viejo `main.rs` tenía:

```rust
        .add_systems(Update, (sync_from_js, aim_and_launch, physics, collisions, sync_to_js).chain())
```

Ese `.chain()` funcionaba porque los cinco sistemas se registraban *en un mismo sitio*. Ahora viven en tres plugins distintos que no se conocen. La solución es ponerle nombre a las etapas de la tubería — en `main.rs`, porque la forma del frame es una decisión a nivel de aplicación:

```rust
/// The frame pipeline, named: state flows in, input acts, the world
/// simulates, state flows out. Plugins hang their systems on these.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    SyncIn,
    Input,
    Physics,
    SyncOut,
}
```

…y declarar su orden una sola vez:

```rust
        .configure_sets(
            Update,
            (
                GameSet::SyncIn,
                GameSet::Input,
                GameSet::Physics,
                GameSet::SyncOut,
            )
                .chain(),
        )
```

Cada plugin cuelga entonces sus sistemas del gancho correcto: `aim_and_launch.in_set(GameSet::Input)`, `sync_from_js.in_set(GameSet::SyncIn)`, y physics encadena su propio par *dentro* de su set. **Los sets son el contrato; los plugins son las implementaciones.** `SessionPlugin` no sabe que `ShootingPlugin` existe — ambos solo saben que el frame tiene etapas con nombre. (Los sistemas de feedback no entran en ningún set: solo leen, así que cualquier orden vale, y Bevy queda libre para paralelizarlos.)

## Paso 4 — Verifica: la refactorización no cambió nada

```
cargo build                                  ← lo nativo sigue compilando
cargo check --target wasm32-unknown-unknown  ← el puente sigue compilando
trunk serve                                  ← juega una sesión completa
```

Tira, haz bank, anota, alcanza el límite, Save & Reset desde el panel. Juego idéntico. Esta es la disciplina del ejercicio: una refactorización que "de paso arregló una cosita" son dos cambios vistiendo un solo commit, y cuando algo se rompa no sabrás cuál de los dos fue.

## Sobre aquellas compilaciones lentas por capítulo

Una promesa del Capítulo 4 vence hoy. Cada carpeta de capítulo compila su propia copia de Bevy en su propio `target/` — minutos y gigabytes cada una. Dos soluciones profesionales:

- **Un directorio target compartido.** Define la variable de entorno `CARGO_TARGET_DIR` a una ruta fija, y todos los proyectos de tu máquina comparten una caché de compilación — Bevy se compila una vez, para siempre. (Así exactamente se construyeron los snapshots de este curso mientras se escribía.)
- **Un workspace de Cargo.** Un `Cargo.toml` raíz con `[workspace] members = ["chapters/*"]` hace a las carpetas hermanas dentro de un proyecto: `target/` compartido, lockfile compartido, `cargo build` desde la raíz lo compila todo. Los workspaces son la organización de todo proyecto Rust multi-crate que conocerás — incluido el propio Bevy, que es ~40 crates en un workspace.

## Experimentos antes de continuar

1. Comenta `feedback::FeedbackPlugin` en `main.rs`. La red, la explosión y el HUD desaparecen; la pelota, las físicas y la puntuación no. Una línea, una función — esa es la prueba de la costura.
2. Añade un `DebugPlugin` en un `debug.rs` nuevo: un sistema que imprima la velocidad de la pelota cuando esté `Flying`. Fíjate en que tocas *cero* archivos existentes salvo una línea de `main.rs`.
3. Reordena las variantes en `configure_sets` para que `SyncOut` corra antes que `Physics`. Juega — el panel ahora muestra cada valor un frame tarde. Los bugs sutiles de orden son exactamente aquello de lo que los sets con nombre te protegen.

## Qué construiste / Qué sigue

El mismo juego, reestructurado para un equipo: nueve módulos de propósito único, cinco plugins auto-instalables, y una tubería de frame con nombre que les permite coordinarse sin conocerse. Esta estructura es también el [`final-game/`](../../final-game/) de este repo — el artefacto terminado del curso.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/13-refactoring-like-an-engineer/`](.).

Queda un capítulo. En el **Capítulo 14**: compilaciones release, hacer pequeño el archivo WASM, y poner tu juego en el internet público.

**[Continuar al Capítulo 14: Optimizando y publicando →](../14-optimizing-and-shipping/README.es.md)**
