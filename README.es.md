# 🏀 Rust + Bevy: de principiante a ingeniero

**Construye un juego de baloncesto con físicas reales que corre en el navegador — empezando desde cero.**

*Léelo en: [English](README.md) | **Español***

![Partida del juego de baloncesto terminado](assets/hero-gameplay.gif)

*▶ [Mira una sesión de juego completa de un minuto (video)](assets/rust-bevy-video.mp4)*

Este es un curso práctico y gratuito. Empiezas sin nada instalado y terminas con un juego completo — tiro con carga progresiva, gravedad y rebotes reales, puntuación y sesiones de juego — escrito en **Rust** con el motor de juegos **Bevy**, compilado a **WebAssembly** y jugable en cualquier navegador moderno.

No necesitas experiencia previa con Rust ni con desarrollo de videojuegos. Cada concepto del lenguaje (ownership, enums, traits…) se explica la primera vez que el código del juego lo necesita.

## Por qué existe este curso

Construí este juego de baloncesto mientras aprendía Rust y Bevy, y por el camino me encontré con todo lo que hace frustrantes los tutoriales de desarrollo de juegos para un principiante: guías escritas para versiones del motor que ya no existen, comandos que asumen herramientas que nadie te dijo que instalaras, y errores que el autor aparentemente nunca vio — porque nunca ejecutó sus propios pasos.

Así que este curso se apoya en tres promesas:

1. **Todo lo que hay aquí se ejecutó de verdad.** Cada comando se corrió, cada bloque de código compila exactamente en el punto del curso donde aparece, y cada captura es una captura real de la compilación real. Cuando el curso cita un tiempo de compilación o el tamaño de un archivo, es una medición, no una suposición.
2. **Los errores son reales.** Las cajas de troubleshooting documentan fallos que ocurrieron de verdad construyendo este juego — el linker ausente, el PATH desactualizado, el conflicto de puerto, el muro de versiones incompatibles. Probablemente te encuentres con alguno; la solución te estará esperando.
3. **Las versiones están fijadas, siempre.** Nada aquí dice "la última." Todo está probado contra las versiones exactas listadas abajo, para que el curso que lees sea el curso que funciona.

Y es **bilingüe** — cada capítulo existe en inglés y en español — porque el buen material de Rust y Bevy en español escasea, y aprender algo tan denso en un segundo idioma no debería ser el precio de la entrada.

Si este curso te ayuda, una ⭐ en el repo ayuda al siguiente estudiante a encontrarlo.

## Qué vas a construir

Un juego 2D de tiros de baloncesto donde:

- Mantienes pulsado para cargar la potencia del tiro y apuntas antes de soltar
- Ves la pelota volar con gravedad real y rebotar en el aro y el tablero
- Anotas puntos y registras tus resultados en sesiones con límite de tiros
- Controlas el juego desde un panel HTML (nombre del jugador, resultados, reinicio) que habla directamente con el código Rust

Y por el camino aprenderás lo que hace que este curso llegue "a ingeniero": la arquitectura Entity-Component-System, refactorizar un prototipo de un solo archivo en módulos y plugins, ajustar perfiles de compilación para bundles WASM diminutos, y desplegar en la web.

## Los capítulos

Cada carpeta de capítulo contiene la lección **y** una copia completa y ejecutable del proyecto en ese punto. ¿Te perdiste? Entra en la carpeta de cualquier capítulo y continúa desde ahí.

### Parte I — Preparación
| # | Capítulo | Vas a |
|---|---|---|
| 00 | [Antes de empezar](chapters/00-before-you-start/README.es.md) | Ver qué vas a construir y revisar los requisitos |
| 01 | [Instalando las herramientas](chapters/01-installing-the-toolchain/README.es.md) | Instalar Rust, las build tools, Trunk y VS Code |
| 02 | [Hola, Cargo](chapters/02-hello-cargo/README.es.md) | Crear y ejecutar tu primer programa en Rust |

### Parte II — Primeros pasos con Bevy
| # | Capítulo | Vas a |
|---|---|---|
| 03 | [Tu primera ventana](chapters/03-first-window/README.es.md) | Abrir una ventana de juego con Bevy |
| 04 | [ECS: entidades, componentes, sistemas](chapters/04-ecs-entities-components-systems/README.es.md) | Dibujar tu primer sprite y entender la arquitectura de Bevy |
| 05 | [Poniendo cosas en movimiento](chapters/05-making-things-move/README.es.md) | Animar con sistemas, queries y delta time |
| 06 | [Corriendo en el navegador](chapters/06-running-in-the-browser/README.es.md) | Compilar a WebAssembly y servir con Trunk |

### Parte III — Construyendo el juego de baloncesto
| # | Capítulo | Vas a |
|---|---|---|
| 07 | [La cancha](chapters/07-the-court/README.es.md) | Dibujar la cancha, el aro, el tablero y la pelota |
| 08 | [La mecánica de tiro](chapters/08-the-shooting-mechanic/README.es.md) | Implementar la carga progresiva y el apuntado |
| 09 | [Físicas](chapters/09-physics/README.es.md) | Añadir gravedad, velocidad y rebotes |
| 10 | [Puntuación y feedback](chapters/10-scoring-and-feedback/README.es.md) | Detectar canastas y mostrar la puntuación |
| 11 | [Sesiones de juego](chapters/11-game-sessions/README.es.md) | Añadir límite de tiros, resultados y reinicio |

### Parte IV — De principiante a ingeniero
| # | Capítulo | Vas a |
|---|---|---|
| 12 | [Hablando con la página web](chapters/12-talking-to-the-web-page/README.es.md) | Conectar Rust ↔ HTML con wasm-bindgen |
| 13 | [Refactorizando como ingeniero](chapters/13-refactoring-like-an-engineer/README.es.md) | Dividir el juego en módulos y plugins de Bevy |
| 14 | [Optimizando y publicando](chapters/14-optimizing-and-shipping/README.es.md) | Reducir el bundle WASM y desplegar en la web |

## Requisitos de un vistazo

- **Sistema operativo**: Windows 10/11, macOS o Linux (las capturas del curso usan Windows 11)
- **Disco**: ~10 GB libres (la toolchain de Rust y los artefactos de compilación ocupan mucho)
- **Editor**: usamos **VS Code**, pero cualquier editor o IDE sirve
- **Experiencia**: ninguna — este curso empieza desde cero

> [!IMPORTANT]
> Este curso fija versiones exactas para que cada bloque de código compile tal cual está escrito: **Rust 1.96.0**, **Bevy 0.16**, **Trunk 0.21.14**, **wasm-bindgen 0.2.122**. Las versiones nuevas de Bevy cambian su API — usa las versiones fijadas mientras sigues el curso.

## Cómo usar este curso

1. Lee los capítulos en orden — cada uno se apoya en el anterior.
2. Escribe el código tú mismo en vez de copiar y pegar; ahí es donde se aprende.
3. ¿Atascado o compilación rota? Compara tu código con la carpeta del capítulo.
4. Fíjate en las cajas de aviso:
   - **Note** — un concepto del lenguaje Rust, explicado la primera vez que aparece
   - **Warning** — un error real que puedes encontrarte, con su solución real
   - **Tip** — atajos y mejoras de calidad de vida

**Empieza aquí → [Capítulo 0: Antes de empezar](chapters/00-before-you-start/README.es.md)**

## Licencia

[MIT](LICENSE) — usa el código y las lecciones libremente, para aprender o para lo que quieras.
