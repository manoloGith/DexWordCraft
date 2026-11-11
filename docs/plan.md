# Plan y estado

DexWordCraft sigue el [prompt de portado de DexPdfCraft](https://github.com/manoloGith/DexPdfCraft/blob/main/docs/PORTING-PROMPT.md),
con lo que se aprendió después en [DexGridCraft](https://github.com/manoloGith/DexGridCraft).
WordCraft usa el mismo egui (`6d518fc`) y el mismo winit (`a517b38`) que PdfCraft y GridCraft, así
que toda la capa Android viene hecha en craft-android y las fases se han hecho de una vez. **Nada se
ha probado aún en un dispositivo.** La lista de pruebas está en [pruebas.md](pruebas.md).

## Lo que se encontró al estudiar WordCraft (sección 6 del prompt)

| Punto | En WordCraft | En DexWordCraft |
|---|---|---|
| Arranque (`apps/wordcraft/src/main.rs`) | `WordApp::new(Session, Services)`, `ui.json`, diccionario personal, fuentes del usuario, recuperación local, ventanas nativas por documento, canal de control, geometría de la ventana, Apple events, autoactualización | `DexWord` (`src/lib.rs`) hace lo mismo sin ventanas nativas, sin canal de control, sin geometría, sin Apple events y sin autoactualización |
| `rfd` / `arboard` | Solo en el binario: la interfaz recibe los diálogos y el portapapeles por `Services` | No hacen falta shims: `src/android/mod.rs` rellena `Services` con craft-android |
| Diálogos | `Services::file_dialog` ya es asíncrono: la respuesta llega por un canal | El selector se abre en el hilo de la interfaz y la respuesta se espera en otro hilo. No hace falta cambiar el fork |
| Compilación para Android | — | `cargo ndk -t arm64-v8a check -p wordcraft-ui-egui` pasó **sin cambios** |
| `Command::new` | `speech-dispatcher`/`espeak` (Leer en voz alta en Linux), `xdg-mime`, `dbus-send` y `sysctl` (en el binario) | Leer en voz alta no encuentra voz en Android (ver límites). Lo del binario no se usa |
| Viewports secundarios | Un documento, una ventana nativa (craft-window) | Una sola ventana, como en la versión web (cambio en el fork) |
| Entradas de compilación | `CRAFT_I18N_DIR` (traducciones, ortografía y guiones) y `CRAFT_FONTS_DIR` opcional | craft-i18n sí (`scripts/fetch-build-inputs.sh`; el CI lo exige). craft-fonts no: añadiría unos 45 MB y Android ya trae esas familias en `/system/fonts` |
| Fuentes | Inter, Source Sans/Serif y JetBrains Mono incrustadas; las del sistema en `/usr/share/fonts` | Las incrustadas, más `/system/fonts` y `/product/fonts` (cambio en el fork) |
| Marcas | Logotipos de ArtCraft solo en `docs/brand/` (la interfaz no los incrusta); el icono de WordCraft solo en la ventana y en el diálogo de actualizaciones | Icono propio; la ventana de Android no usa el icono de egui |
| Datos y configuración | `$XDG_CONFIG_HOME/wordcraft` en Linux (`wordcraft-control-key`) | craft-android apunta XDG al almacenamiento privado |
| Formatos | docx, docm, dotx, dotm, doc, dot, odt, ott, fodt, rtf, txt, md, html, htm, mht, mhtml, tex | Filtros de intents `VIEW`/`EDIT`/`SEND` con sus tipos MIME |
| Autoguardado y recuperación | AutoGuardado de Word para archivos docx; una copia de recuperación de cada documento cambiado cada 10 s, que se ofrece al arrancar | Igual. Si Android cierra la app (la X de DeX no se puede interceptar), se pierden como mucho 10 s. Las preferencias se guardan al perder el foco |
| Guardado atómico | Escribe `.wordcraft-save-….tmp` y lo renombra | El `DocumentStore` de craft-android ignora los nombres con punto y escribe de vuelta al renombrar |
| Idioma | `WORDCRAFT_LANGUAGE`, luego `sys-locale` | `android_main` convierte el `LANG` de craft-android (`es_ES.UTF-8`) en `WORDCRAFT_LANGUAGE=es-ES`. La región (papel, unidades) sale de `LANG` |
| Hardware | Leer en voz alta (TTS del sistema) | Sin voz por ahora |

## Fases

| Fase | Hecho |
|---|---|
| 0 | Repositorio, fork con rama `dexcraft/android`, submódulos `upstream` y `craft-android`, workspace con los `[patch]` de upstream copiados (krilla, egui) y el winit de craft-android, Gradle + cargo-ndk, CI, NOTICE, licencias, icono propio |
| 1 | eframe en GameActivity (CraftActivity), wgpu con glow de reserva, el motor en arm64, diagnóstico de craft-android (renderizador, versión, idioma) |
| 2 | La interfaz real de WordCraft, arrancada como en escritorio; entorno HOME/XDG/LANG (craft-android); idioma de Android; traducciones y ortografía de craft-i18n; fuentes de `/system/fonts` (fork) |
| 3 | Selectores del SAF con escritura de vuelta: Abrir, Guardar como, Exportar, Insertar imagen, Añadir fuente, lista de destinatarios; portapapeles (texto, HTML, imagen, archivos) y Ctrl+C/X/V; «Abrir con» y «Compartir»; ratón, teclado y ciclo de vida (craft-android); informe de diagnóstico (Ctrl+Mayús+D) |
| 4 | Cursores, título de la tarea («Documento1 - DexWordCraft»), arrastrar y soltar documentos (WordCraft los abre; `droppableMimeTypes`) |
| 5 | Imprimir: WordCraft compone el PDF y lo pasa al diálogo de impresión de Android, como hace con el del navegador en la versión web; enlaces con Intent (craft-android); sin autoactualización |
| 6 | Teclado en pantalla e insets (craft-android) |
| 7 | Perfil de release (LTO, `codegen-units = 1`), firma y workflow de release, metadatos de F-Droid y Obtainium |

## Decisiones y límites conocidos

- **Una sola ventana.** «Nueva ventana» y «Cambiar ventanas» funcionan dentro de la app, como en la
  versión web de WordCraft.
- **Imprimir.** Se usa la vía de la versión web (`Services::print`): el panel Imprimir no lista
  impresoras propias, y la impresora, las copias y la doble cara se eligen en el diálogo de Android.
- **Guardar como.** El selector de Android ofrece el tipo del nombre sugerido y empieza en la
  carpeta que él elija (WordCraft pide la de plantillas al guardar una plantilla).
- **Leer en voz alta** no tiene voz: WordCraft busca `speech-dispatcher` o `espeak`, que Android no
  tiene. Hacerlo con el `TextToSpeech` de Android necesitaría un puente en craft-android y un
  backend nuevo en el motor.
- **Ortografía.** Con craft-i18n, la misma que en escritorio. Los paquetes de idioma que se
  instalan desde la tienda de fuentes de la suite van al almacenamiento privado.
- **Hacer de WordCraft la app predeterminada** (Opciones ▸ General) no aparece: en Android eso lo
  decide el sistema.
