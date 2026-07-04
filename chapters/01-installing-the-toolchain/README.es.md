# Capítulo 1 — Instalando las herramientas

*Léelo en: [English](README.md) | **Español***

En este capítulo instalarás todo lo necesario para construir juegos en Rust para escritorio **y** para el navegador, verificando cada pieza antes de continuar. Nada de esto es difícil, pero es donde la mayoría de tutoriales pierde a la gente — así que iremos paso a paso e incluiremos los errores reales que puedes encontrarte.

**Tiempo**: 30–60 minutos, casi todo descargas. **Descargas**: ~8 GB en Windows (menos en macOS/Linux).

## Qué vamos a instalar, y por qué en este orden

1. **Visual Studio Build Tools** *(solo Windows)* — el linker que Rust necesita. Instalarlo primero evita el error más común de Rust en Windows.
2. **rustup** — el instalador oficial de Rust, que trae **rustc** (el compilador) y **cargo** (la herramienta de build y gestor de paquetes).
3. **El target de WebAssembly** — le enseña al compilador a producir salida ejecutable en el navegador.
4. **Trunk** — la herramienta que compila y sirve apps Rust WASM.
5. **VS Code + rust-analyzer** — el editor (sáltatelo si usas otro editor).

## Paso 1 — Visual Studio Build Tools (solo Windows)

> Usuarios de macOS: ejecutad `xcode-select --install` en la Terminal y saltad al Paso 2.
> Usuarios de Linux: instalad la toolchain de C de vuestra distro (`sudo apt install build-essential` en Ubuntu/Debian) y saltad al Paso 2.

Rust compila tu código, pero en Windows usa el **linker** de Microsoft (`link.exe`) para producir el ejecutable final — y ese linker viene con las herramientas de C++ de Visual Studio, no con Rust.

1. Ve a <https://visualstudio.microsoft.com/downloads/> y baja hasta **Tools for Visual Studio** → **Build Tools for Visual Studio 2022**. Descárgalo y ejecútalo.
2. En el instalador, marca la carga de trabajo **"Desarrollo para el escritorio con C++"** (*Desktop development with C++*). Los valores por defecto dentro de ella están bien.
3. Pulsa **Instalar**. Esta es la descarga grande (~6–7 GB) — buen momento para un café.

> [!TIP]
> Si usas `winget`, un solo comando hace lo mismo:
> `winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`

> [!WARNING]
> **El error que te sale si te saltas este paso.** Todo *parecerá* ir bien — hasta tu primera compilación, que falla con:
>
> ```
> error: linker `link.exe` not found
>   |
>   = note: program not found
>
> note: the msvc targets depend on the msvc linker but `link.exe` was not found
> note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.
> ```
>
> Nos pasó de verdad construyendo este mismo juego. La solución es exactamente este paso: instala las Build Tools con la carga de trabajo de C++, **reinicia tu terminal** y compila de nuevo. Tu código Rust nunca fue el problema.

## Paso 2 — Rust, con rustup

**rustup** es el gestor oficial de toolchains de Rust. Instala el compilador y Cargo, los mantiene actualizados y gestiona *targets* (como WebAssembly) — nunca instalarás Rust de otra manera.

### Windows

1. Ve a <https://rustup.rs> y descarga `rustup-init.exe` (64-bit).
2. Ejecútalo. Se abre una ventana de terminal que pregunta cómo proceder:

```
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

3. Pulsa **Enter** para aceptar la instalación estándar.
4. Al terminar verás `Rust is installed now. Great!`

### macOS / Linux

Abre una terminal y ejecuta:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pulsa **Enter** en el mismo menú para aceptar la instalación estándar.

### Verifícalo

**Cierra tu terminal por completo y abre una nueva** (importa — mira el aviso de abajo), y ejecuta:

```
rustc --version
cargo --version
```

Deberías ver algo como:

```
rustc 1.96.0 (ac68faa20 2026-05-25)
cargo 1.96.0 (30a34c682 2026-05-25)
```

> [!IMPORTANT]
> Este curso fue escrito y probado con **Rust 1.96.0**. Si rustup te instaló una versión estable *más nueva*, no pasa nada — Rust es retrocompatible, así que compiladores más nuevos construyen el código del curso sin cambios. Las versiones estrictamente fijadas del curso son las de **Bevy, Trunk y wasm-bindgen**, no la de Rust.

> [!WARNING]
> **`rustc` no se reconoce como comando.** Si los comandos de versión fallan justo después de instalar, el 90% de las veces la causa es esta: el instalador añadió Rust a tu PATH, pero *las terminales que ya estaban abiertas no ven los cambios del PATH*. Esto también nos pasó durante la construcción original de este juego. Cierra **todas** las terminales abiertas (en VS Code también: mata la terminal, no la limpies), abre una nueva y prueba otra vez. Si sigue fallando tras una terminal nueva *y* un reinicio, vuelve a ejecutar el instalador de rustup y comprueba que no reportó ningún error.

> [!NOTE]
> **Sidebar de Rust: ¿qué acabamos de instalar?**
> - `rustc` es el compilador — convierte archivos fuente `.rs` en programas ejecutables. Casi nunca lo llamarás directamente.
> - `cargo` es la herramienta en la que vivirás de verdad: crea proyectos, descarga bibliotecas (Rust las llama *crates*), compila, ejecuta y testea tu código. Si has usado `npm` (JavaScript) o `pip` (Python), Cargo cumple ese papel — y además es el sistema de build.
> - `rustup` gestiona a los otros dos, y puede instalar *targets* extra: conjuntos de instrucciones para los que el compilador puede producir salida. Lo que nos lleva a…

## Paso 3 — El target de WebAssembly

De fábrica, tu compilador produce programas para *tu* máquina (en Windows, ese target se llama `x86_64-pc-windows-msvc`). Para correr en un navegador, necesitamos que también produzca **WebAssembly**. Un solo comando:

```
rustup target add wasm32-unknown-unknown
```

Salida esperada:

```
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
```

Verifica que quedó registrado:

```
rustup target list --installed
```

Deberías ver `wasm32-unknown-unknown` en la lista (junto a tu target nativo).

> [!NOTE]
> **El nombre raro, descifrado.** Los nombres de target siguen el patrón *arquitectura-fabricante-SO*. `wasm32` = WebAssembly de 32 bits; los dos `unknown` significan "ningún fabricante concreto, ningún sistema operativo concreto" — porque WASM corre dentro del sandbox de un navegador, no sobre un SO. Escribirás este nombre muchas veces; deja de sonar raro enseguida.

## Paso 4 — Trunk

[Trunk](https://trunkrs.dev) es la herramienta de build-y-servir para apps web de Rust en WASM. Con un comando (`trunk serve`) compila tu Rust a WebAssembly, genera el pegamento JavaScript que el navegador necesita, lo empaqueta con tu HTML, lo sirve en local y recompila con cada cambio de archivo.

Instala la versión exacta que usa este curso:

```
cargo install trunk --version 0.21.14 --locked
```

> [!TIP]
> `cargo install` compila Trunk desde el código fuente, así que tarda varios minutos e imprime cientos de líneas `Compiling ...`. Es normal — déjalo correr. Acabas de usar Cargo como gestor de paquetes por primera vez.

Verifica:

```
trunk --version
```

```
trunk 0.21.14
```

## Paso 5 — VS Code y rust-analyzer

*(Sáltatelo si usas otro editor — solo instala su soporte de Rust/rust-analyzer.)*

1. Instala VS Code desde <https://code.visualstudio.com>.
2. Ábrelo, ve a Extensiones (`Ctrl+Shift+X` / `Cmd+Shift+X`), busca **rust-analyzer** e instala el publicado por *rust-lang* (millones de instalaciones).

rust-analyzer te da tres cosas sin las que un principiante no debería trabajar: errores mostrados **en línea mientras escribes** (antes incluso de compilar), autocompletado que conoce toda la API de Bevy, y documentación al pasar el cursor sobre cualquier función. Cuando empecemos a escribir código en el Capítulo 2, pasa el cursor por encima de todo lo que no reconozcas.

## Verificación final — ejecuta todos estos

Abre una terminal **nueva** y confirma que cada línea funciona:

| Comando | Esperado |
|---|---|
| `rustc --version` | `rustc 1.96.0` (o más nuevo) |
| `cargo --version` | `cargo 1.96.0` (o más nuevo) |
| `rustup target list --installed` | incluye `wasm32-unknown-unknown` |
| `trunk --version` | `trunk 0.21.14` |

¿Pasan los cuatro? **Tu máquina ya puede construir juegos en Rust para escritorio y para el navegador.** Esa es toda la batalla de este capítulo.

## Qué construiste / Qué sigue

Todavía no compilamos nada — pero montaste y verificaste una toolchain profesional y completa de juegos web en Rust, y conoces los dos fallos clásicos de instalación (linker ausente, PATH desactualizado) y sus soluciones.

En el **Capítulo 2** escribirás, compilarás y ejecutarás tu primer programa en Rust con Cargo, y conocerás la anatomía de un proyecto Rust.

**[Continuar al Capítulo 2: Hola, Cargo →](../02-hello-cargo/README.es.md)**
