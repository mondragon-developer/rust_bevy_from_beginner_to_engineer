# Capítulo 6 — Corriendo en el navegador

*Léelo en: [English](README.md) | **Español***

Este es el capítulo donde tu toolchain del Capítulo 1 rinde cuentas. La pelota rodante del Capítulo 5 — exactamente el mismo código Rust — está a punto de correr dentro de Chrome, Firefox o cualquier navegador moderno, con **cero cambios en la lógica del juego**. Por el camino aprenderás qué es realmente WebAssembly, qué hace `index.html` en un proyecto Rust, y los dos modos de fallo clásicos de esta tubería.

**Tiempo**: ~45 minutos, más una primera compilación WASM.

## Qué es realmente WebAssembly

Históricamente, los navegadores solo ejecutaban JavaScript. **WebAssembly (WASM)** es la segunda cosa que pueden ejecutar: un formato binario compacto de instrucciones que corre a velocidad casi nativa, dentro del mismo sandbox de seguridad que todo lo demás en una página web. No es un lenguaje que escribas — es un *destino de compilación*, igual que "Windows" o "macOS" son destinos de compilación. Rust es uno de los lenguajes que mejor compila hacia él.

Para nosotros significa: un solo código, y cualquiera con un navegador puede jugar a tu juego — sin instalador, sin "solo funciona en Windows", sin app store. Así es como se distribuye el juego de baloncesto que viste en el Capítulo 0.

## Paso 1 — El proyecto (¡sin código de juego nuevo!)

Crea `browser_ball`, configura `Cargo.toml` como siempre, y copia dentro el `main.rs` del Capítulo 5 sin cambios (puedes actualizar el título de la ventana a `"Bevy in the Browser"` — eso es cosmético). Ese es el mensaje del capítulo: **el código del juego no sabe ni le importa que va a la web.**

## Paso 2 — Una regla de dependencias nueva

Añade este bloque a `Cargo.toml`:

```toml
# WASM-only: pin wasm-bindgen to the exact version Bevy already locks,
# so Trunk's wasm-bindgen-cli matches the bindings our build generates.
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "=0.2.122"
```

Dos ideas nuevas en cinco líneas:

- **Dependencias por destino.** La cabecera `[target.'cfg(target_arch = "wasm32")']` significa: esta dependencia solo existe al compilar para WebAssembly. Las compilaciones nativas la ignoran por completo.
- **wasm-bindgen** es el puente entre el mundo WASM y el mundo del navegador. El WASM compilado no puede tocar la página, el canvas ni los eventos de input por sí mismo — `wasm-bindgen` genera el pegamento JavaScript que los conecta. Tiene dos mitades que *deben coincidir exactamente*: una biblioteca compilada dentro de tu juego, y una herramienta de línea de comandos (que ejecuta Trunk) que genera el pegamento. El `=` de `"=0.2.122"` significa *exactamente* esta versión, no "0.2.122 o más nueva" — eso es lo que mantiene las dos mitades sincronizadas.

> [!WARNING]
> **El error cuando las mitades no coinciden.** Si tu crate `wasm-bindgen` bloqueado y la herramienta CLI no están de acuerdo, la compilación (o la página) falla con un mensaje como:
>
> ```
> it looks like the Rust project used to create this wasm file was linked against
> version of wasm-bindgen that uses a different bindgen format than this binary
> ```
>
> La solución es lo que acabamos de hacer: fijar `wasm-bindgen` con una versión exacta `=` que coincida con lo que espera el ecosistema de Bevy. Este pin existe en el `Cargo.toml` del juego de baloncesto real exactamente por esta razón.

## Paso 3 — index.html: el anfitrión web

Crea `index.html` en la raíz del proyecto — **junto a** `Cargo.toml`, no dentro de `src/`:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Bevy in the Browser</title>
    <!-- This one line tells Trunk: compile this folder's Rust to WASM
         and wire it into the page. -->
    <link data-trunk rel="rust" />
    <style>
      html,
      body {
        margin: 0;
        padding: 0;
        background: #16161d;
      }
      /* Bevy creates a <canvas> and draws the game into it. */
      canvas {
        display: block;
        margin: 24px auto 0;
        outline: none;
      }
    </style>
  </head>
  <body></body>
</html>
```

Este archivo es a la vez la página web donde vive tu juego *y* la configuración de Trunk. La línea mágica es `<link data-trunk rel="rust" />` — le dice a Trunk "el proyecto Rust de esta carpeta es la carga útil." Cuando el juego arranca, Bevy crea un elemento `<canvas>` (su ventana, edición web) y dibuja cada frame dentro; el CSS centra ese canvas sobre una página oscura. No hay etiqueta `<script>` porque Trunk inyecta el pegamento generado por ti.

## Paso 4 — trunk serve

```
trunk serve
```

Esta es la tubería que Trunk ejecuta por ti — tu toolchain del Capítulo 1 entera disparándose en secuencia:

1. `cargo build --target wasm32-unknown-unknown` — el compilador de Rust produce un archivo `.wasm` en vez de un `.exe` (para esto instalaste aquel target),
2. `wasm-bindgen` genera el pegamento JavaScript,
3. todo se empaqueta con tu `index.html` en una carpeta `dist/`,
4. arranca un servidor web local, que recompila y recarga la página cada vez que guardas un archivo.

La primera compilación WASM construye el árbol de dependencias completo otra vez para el nuevo destino — en nuestra máquina tardó **3 minutos y 33 segundos**. Coste de una sola vez, como en el Capítulo 3. Después:

```
INFO 📡 serving static assets at -> /
INFO 🏠 server listening at:
INFO     🏠 http://127.0.0.1:8080/
```

Abre **<http://127.0.0.1:8080>** en tu navegador:

![La pelota naranja rodando dentro de una página del navegador](../../assets/ch06-browser-ball.png)

La misma pelota, rodando de un lado a otro sobre el mismo suelo — dentro de una página web. El juego es Rust compilado, corriendo a toda velocidad en el sandbox del navegador, redibujado por tu GPU a través de WebGL.

> [!WARNING]
> **`error: error binding to 127.0.0.1:8080 ... Address already in use`** — algo más en tu máquina ya es dueño del puerto 8080 (muy a menudo: otro `trunk serve` que olvidaste en otra terminal). Nos pasó durante la construcción original. O cierras el otro programa, o eliges otro puerto: `trunk serve --port 8081`. Este error parece grave y no significa casi nada.

> [!TIP]
> Deja `trunk serve` corriendo y cambia algo — el color de la pelota, la velocidad de la onda — y guarda. Trunk recompila (segundos, gracias al truco de perfiles del Capítulo 3) y el navegador se recarga solo. Este bucle de editar-guardar-ver es como está pensado que se viva el resto del curso.

## Nativo y web, lado a lado

Nada de la versión de escritorio se rompió: `cargo run` en esta misma carpeta sigue abriendo la ventana nativa. Un código, dos plataformas — durante el resto del curso desarrollamos en el navegador con `trunk serve`, porque ahí es donde vive el juego terminado, pero cada capítulo sigue corriendo nativo también.

Una nota de expectativas: este `.wasm` en modo desarrollo es grande (decenas de MB) y sin optimizar. Publicar un build web pequeño y rápido es un tema de ingeniería de verdad — y es exactamente de lo que trata el Capítulo 14.

## Qué construiste / Qué sigue

Tu juego corre en la web. Y más importante: entiendes cada pieza móvil: el target `wasm32` produce el binario, `wasm-bindgen` lo conecta al navegador (con un apretón de manos de versión exacta), `index.html` lo hospeda, y Trunk dirige la orquesta.

Tu código debería coincidir ahora con la carpeta de este capítulo: [`chapters/06-running-in-the-browser/`](.).

**La Parte II está completa.** En el **Capítulo 7** empezamos a construir el juego de verdad: la cancha, el tablero, el aro — dibujados con formas reales en vez de cuadrados, y organizados con constantes como lo haría un ingeniero.

**[Continuar al Capítulo 7: La cancha →](../07-the-court/README.es.md)**
