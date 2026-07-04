# Capítulo 14 — Optimizando y publicando

*Léelo en: [English](README.md) | **Español***

Tu juego está terminado. Este capítulo trata de la distancia entre *terminado* y *publicado* — y abre con un número que explica por qué esa distancia existe: la compilación de desarrollo de tu juego es un archivo WASM de **66.4 MB**. Ningún teléfono va a descargar eso. Al final de este capítulo será una fracción de ese tamaño, en vivo en una URL pública que puedes poner en tu CV.

**Tiempo**: ~1 hora (más una compilación release larga).

## Paso 1 — El perfil release

Cargo ha tenido dos personalidades todo este tiempo: `cargo build` usa el perfil `dev` (compilaciones rápidas, salida grande — el que afinamos desde el Capítulo 3), y `--release` usa el perfil `release`. El nuestro va en `Cargo.toml`, y es el último bloque que el archivo necesitará jamás:

```toml
# Release profile tuned for small WASM bundles shipped to the browser.
[profile.release]
opt-level = "z"
lto = "thin"
codegen-units = 1
panic = "abort"
```

Línea por línea, porque cada una es un trade-off real de ingeniería:

- **`opt-level = "z"`** — optimiza para *tamaño*, no velocidad. Los niveles 1–3 hacen el código más rápido; `"s"` y `"z"` lo hacen más pequeño. Para un juego 2D que ya corre a toda velocidad, cada megabyte importa más que cada microsegundo — el mayor problema de rendimiento de un juego web es su *descarga*.
- **`lto = "thin"`** — optimización en tiempo de enlazado: el compilador optimiza *a través* de las fronteras entre crates, lo que sobre todo significa **borrar código**. Bevy trae un motor entero; tu juego usa una rebanada. LTO es como la maquinaria de animación, audio y glTF que no usas se cae del binario.
- **`codegen-units = 1`** — normalmente Rust parte cada crate en trozos compilados en paralelo (compilaciones rápidas, optimizaciones perdidas entre fronteras de trozos). Una unidad = la compilación más lenta, la mejor salida. Bien para release: se compila una vez por publicación.
- **`panic = "abort"`** — cuando un programa Rust entra en pánico, normalmente hace *unwinding*, desmontando la pila con cuidado; esa maquinaria es código que envías. Abort significa "simplemente para" — la respuesta correcta para un juego, y bytes gratis de vuelta.

Cada línea intercambia **tiempo de compilación por calidad de salida**. Por eso es el perfil release y no el dev.

## Paso 2 — El optimizador después del compilador

¿Recuerdas esta línea de `index.html`, plantada en el Capítulo 12?

```html
<link data-trunk rel="rust" data-wasm-opt="z" />
```

`wasm-opt` es un *segundo* optimizador (del toolkit de WebAssembly, Binaryen) que trabaja directamente sobre el `.wasm` compilado — encogiendo codificaciones de instrucciones, deduplicando, eliminando código muerto que el compilador no vio. El atributo `data-wasm-opt="z"` le dice a Trunk que lo ejecute, en ajuste de tamaño, solo en compilaciones release. Dos optimizadores en fila es práctica estándar en el destino web — el compilador piensa en Rust, `wasm-opt` piensa en WASM.

## Paso 3 — Compila y mide

```
trunk build --release
```

Ve a por el café del Capítulo 3 — LTO con una sola unidad de código recompila todas las dependencias y después `wasm-opt` mastica el resultado (5 minutos y 44 segundos en total en nuestra máquina: 3m 08s de compilación, el resto es `wasm-opt`). Después mira dentro de `dist/`:

| Compilación | Tamaño del .wasm |
|---|---|
| `trunk build` (dev) | 66.4 MB |
| `trunk build --release` | **17.3 MB** |

**Un 74% más pequeño.** Tres cuartas partes del binario eran margen de optimización — andamiaje de depuración, maquinaria de unwinding y código del motor que tu juego nunca llama, todo borrado por el perfil que acabas de escribir.

Dos notas honestas para calibrar expectativas. Primera: sí, sigue siendo grande para un jueguito de baloncesto — estás enviando un motor de juegos de verdad. Recortar más es ingeniería real (las *cargo features* de Bevy permiten compilar fuera los subsistemas que no usas — sonido, 3D, escenas; esa es tu primera parada si persigues megabytes). Segunda: el cable es más amable que el disco — los servidores web comprimen el WASM con gzip o brotli automáticamente, así que lo que un jugador descarga de verdad es típicamente entre un tercio y la mitad del tamaño del archivo.

Y fíjate en qué *es* `dist/`: un `index.html`, un archivo `.js` de pegamento, un `.wasm`. **Tres archivos estáticos.** Sin código de servidor, sin base de datos, sin build en el host. Cualquier cosa que sirva archivos puede servir tu juego — y por eso publicarlo es así de fácil:

## Paso 4 — Ponlo en internet

**Opción A — GitHub Pages** (gratis, y tu código probablemente ya está en GitHub):

1. Compila con el nombre del repo como ruta base — Pages te sirve desde un subdirectorio:
   ```
   trunk build --release --public-url /TU-REPO/
   ```
2. Sube `dist/` al repo (p. ej. en una rama `gh-pages`, o `/docs` en main — quita `dist/` del `.gitignore` para esa rama).
3. En **Settings → Pages** del repo, apunta Pages a esa rama/carpeta.
4. Tu juego está en vivo en `https://TU-USUARIO.github.io/TU-REPO/`.

**Opción B — Vercel / Netlify** (planes gratuitos, URLs más bonitas): ambos sirven una carpeta estática. Instala su CLI o arrastra la carpeta `dist/` en su panel; dile que el directorio de salida es `dist`; listo. (Así está desplegado el juego de referencia de este curso.)

Sea cual sea — **mándale el enlace a alguien.** Un juego que nadie más ha jugado es un artefacto de compilación; un juego con un jugador más, se ha publicado.

## Paso 5 — Presúmelo

El README de tu repo merece una prueba de que el juego existe: una captura y un clip corto de una canasta. En Windows, `Win+Alt+R` (Game Bar) graba un clip; ScreenToGif o LICEcap hacen GIFs directamente. Quince segundos de carga → arco → swish → explosión verde cuentan más que cualquier párrafo — es exactamente como funciona el README de este curso.

## Lo lograste

Haz balance de la distancia recorrida. En el Capítulo 0 no tenías ni compilador. Ahora tienes un juego con físicas que **entiendes hasta la última línea** — escribiste la gravedad, derivaste la fórmula del rebote, diseñaste la máquina de estados, tendiste un puente entre dos lenguajes, refactorizaste como lo haría un equipo, y publicaste una compilación optimizada de tamaño en una URL pública.

La lista del "a ingeniero", en retrospectiva: constantes con nombre como fuente única de verdad · simulación independiente del frame rate · arquitectura ECS · paralelismo guiado por el borrow checker · detección de cambios · protocolos entre fronteras con semántica de tomar-y-limpiar · compilación condicional y stubs de plataforma · módulos, plugins y system sets · perfiles de compilación y optimización en dos etapas · despliegue. Esa no es una lista de juguete. Es un vocabulario de trabajo.

**A dónde ir después:**

- **Añádele a este juego**: sonido en el swish (`bevy_audio` ya está en tu binario), input táctil para móviles, un aro móvil, modo de dos jugadores, una repetición del arco del tiro.
- **Recórtalo**: explora las cargo features de Bevy y mira cuán pequeño puedes dejar `dist/`.
- **Actualízalo**: Bevy avanza rápido. Subir este juego a la siguiente versión de Bevy con la guía oficial de migración es el ejercicio más instructivo que queda en este repo — ahora que cada línea que se rompa es una línea que escribiste tú.
- **Únete**: la carpeta de [ejemplos de Bevy](https://github.com/bevyengine/bevy/tree/main/examples), [The Rust Book](https://doc.rust-lang.org/book/) para la profundidad del lenguaje que este curso aplazó, y el Discord de Bevy, donde los principiantes son bienvenidos.

Gracias por jugar. 🏀

**[Volver al índice del curso →](../../README.es.md)**
