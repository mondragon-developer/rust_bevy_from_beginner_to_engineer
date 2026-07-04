# Capítulo 11 — Sesiones de juego

*Léelo en: [English](README.md) | **Español***

Puedes tirar, rebotar, anotar y celebrar — pero no puedes *perder*, y un juego en el que no puedes perder es un salvapantallas. Este capítulo añade lo que está en juego: una sesión de tiros limitados, un estado de fin de partida que congela la cancha, y una forma limpia de empezar de cero. Es poco código, y casi todo es *diseño de estado* — la parte de la programación de gameplay que separa a los ingenieros de los aficionados.

**Tiempo**: ~45 minutos.

## Paso 1 — Dos recursos describen una sesión

```rust
// True once the shot limit is reached: shooting is frozen until a new game.
#[derive(Resource, Default)]
struct Stopped(bool);

// Max shots before the game stops. 0 = unlimited.
#[derive(Resource)]
struct ShotLimit(u32);

// A hand-written Default (instead of derive) so a fresh game has a real limit.
impl Default for ShotLimit {
    fn default() -> Self {
        ShotLimit(10)
    }
}
```

Registra ambos con `.init_resource::<...>()` como siempre.

> [!NOTE]
> **Sidebar de Rust: implementar un trait a mano.** Hasta ahora, todos los traits venían de `#[derive(...)]` — el compilador escribía el código. Pero el `Default` derivado para `ShotLimit` daría `0` (ilimitado), y queremos que un jugador nuevo reciba una partida real de 10 tiros. Así que por primera vez en este curso, escribimos una implementación de trait nosotros mismos: `impl Default for ShotLimit { ... }`. Esa es toda la ceremonia — un bloque `impl NombreDelTrait for NombreDelTipo` que contiene las funciones del trait. Derivar y escribir a mano producen exactamente el mismo tipo de cosa; derive es solo el atajo para los casos comunes.

Una nota de diseño en la que vale la pena pararse: ¿por qué *dos* recursos? `Stopped` parece redundante — ¿no bastaría con comprobar `attempts >= limit` en todas partes? Podríamos, pero "la partida terminó" es un *hecho con consecuencias* (el input se congela, el HUD cambia), y calcularlo con aritmética en cinco sitios invita a cinco desacuerdos sutiles. Guarda la decisión una vez, tomada en el momento en que ocurre. ¿Y la convención `0 = ilimitado` de `ShotLimit`? La alternativa de ingeniero sería `Option<u32>` — `None` para ilimitado — que es Rust más honesto. Usamos el centinela `0` a propósito: en el próximo capítulo este número vendrá de un panel de control HTML, y JavaScript habla en números, no en `Option`s. A veces la interfaz elige la representación.

## Paso 2 — Congelando el juego

Dos añadidos a `aim_and_launch` (cuya firma gana `mut stopped: ResMut<Stopped>` y `limit: Res<ShotLimit>`). Primero, la puerta — colocada *después* del manejo de la tecla R y *antes* de cualquier apuntado:

```rust
    // Game over: the last shot can still finish flying, but no new charge starts.
    if stopped.0 {
        aim.active = false;
        aim.charge = 0.0;
        return;
    }
```

Segundo, el disparador — en el momento de soltar, justo después de `attempts.0 += 1`:

```rust
        // Count the shot; if that hits the limit, this is the last one — let it
        // fly, then freeze new shots until N starts a fresh game.
        if limit.0 > 0 && attempts.0 >= limit.0 {
            stopped.0 = true;
        }
```

Lee el comentario con atención, porque esto es diseño deliberado de *game feel*: el décimo tiro activa `stopped` **mientras se lanza**, pero nada detiene la pelota — `physics` y `collisions` no consultan `Stopped` para nada. Tu último tiro traza su arco, hace bank y todavía puede anotar mientras el HUD ya dice GAME OVER. Congelar el *input* sin congelar el *mundo* es lo que hace que el final se sienta justo en vez de abrupto. (Fíjate en lo que R deliberadamente no hace: recoloca la pelota pero no devuelve nada y no des-termina nada.)

## Paso 3 — Empezar de cero: el sistema `new_game`

```rust
/// N wipes the session — score, attempts, game-over flag — and re-spots the ball.
fn new_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut attempts: ResMut<Attempts>,
    mut stopped: ResMut<Stopped>,
    mut flash: ResMut<ScoreFlash>,
    mut aim: ResMut<Aim>,
    mut balls: Query<(&mut Ball, &mut Transform)>,
) {
    if !keys.just_pressed(KeyCode::KeyN) {
        return;
    }
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
```

Nada nuevo en la mecánica — lo interesante es la forma. Un reinicio de sesión toca *seis piezas de estado*, y esta función es la lista completa y exhaustiva de lo que significa "partida nueva". Cuando un reporte de bug diga "después de una partida nueva, X quedó viejo," este es el único sitio donde mirar. Regístralo al *frente* de la cadena, para que una partida nueva surta efecto antes de que se interprete el input de este frame:

```rust
        .add_systems(Update, (new_game, aim_and_launch, physics, collisions).chain())
```

(También vale la pena notar: `aim_and_launch` lee el teclado por la R, y `new_game` lo lee por la N. Los recursos no se consumen — cualquier número de sistemas puede leer `ButtonInput<KeyCode>` en el mismo frame.)

Actualiza la línea de instrucciones en `setup` para que los jugadores lo sepan:

```rust
        Text::new("Hold on the ball to charge, aim with the mouse, release to shoot. R = reset ball. N = new game."),
```

## Paso 4 — Un HUD que cuenta la historia completa

`update_score_text` crece hasta su forma final:

```rust
/// Rewrite the HUD only on frames where the session state actually changed.
fn update_score_text(
    score: Res<Score>,
    attempts: Res<Attempts>,
    stopped: Res<Stopped>,
    limit: Res<ShotLimit>,
    mut q: Query<&mut Text, With<ScoreText>>,
) {
    if !score.is_changed() && !attempts.is_changed() && !stopped.is_changed() && !limit.is_changed()
    {
        return;
    }
    let shots = if limit.0 > 0 {
        format!("{}/{}", attempts.0, limit.0)
    } else {
        format!("{}", attempts.0)
    };
    if let Ok(mut text) = q.single_mut() {
        text.0 = if stopped.0 {
            format!("Made: {}   Shots: {}     GAME OVER", score.0, shots)
        } else {
            format!("Made: {}   Shots: {}", score.0, shots)
        };
    }
}
```

Dos cosas pequeñas y una bonita: con límite, los tiros se muestran como `3/10`; sin él, solo `3`. El fin de partida añade el veredicto. Y la bonita — `if`/`else` usados como *expresiones*, su resultado asignado directamente a `shots` y `text.0`. En Rust, casi todo es una expresión; dejarás de escribir el baile de `let x; if ... { x = a } else { x = b }` por completo.

## Ejecútalo

```
trunk serve        (o: cargo run)
```

![El HUD de sesión: Made 0, Shots 0/10, con la tecla de partida nueva en las instrucciones](../../assets/ch11-session-hud.png)

Juega una sesión completa: diez tiros, mira el `7/10` subir, mete las que puedas — y GAME OVER. Apunta todo lo que quieras: la pelota te ignora (pero tu último tiro terminó su vuelo, y si entró, contó). Pulsa **N**. Cancha limpia, `0/10`, otra vez. Ese bucle — jugar, terminar, reiniciar — es la diferencia entre una demo y un juego.

## Experimentos antes de continuar

1. Sesiones speedrun: cambia el `Default` escrito a mano a `ShotLimit(3)`.
2. Modo práctica: `ShotLimit(0)` — el HUD cambia a conteo simple y la partida no termina nunca. Ambos formatos, un solo `if`.
3. Borra la puerta de `stopped.0` en `aim_and_launch` y juega más allá del límite — el HUD grita GAME OVER mientras tú sigues anotando. Siente cómo *guardar* la decisión pero *ignorarla* es peor que no decidir nunca. Devuélvela.

## Qué construiste / Qué sigue

Una máquina de estados de sesión completa: un límite con modo "ilimitado", un final congelado-pero-justo, un reinicio exhaustivo en un solo lugar, y un HUD que lo narra — más tu primera implementación de trait escrita a mano.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/11-game-sessions/`](.).

**La Parte III está completa — el juego es totalmente jugable.** La Parte IV te hace ingeniero: en el **Capítulo 12**, la página web alrededor del juego cobra vida — un panel de control HTML que fija el límite de tiros, muestra resultados en vivo y reinicia la partida, hablando con Rust a través de `wasm-bindgen`.

**[Continuar al Capítulo 12: Hablando con la página web →](../12-talking-to-the-web-page/README.es.md)**
