# El fork de WordCraft

`upstream/` es un submódulo de **[manoloGith/wordcraft](https://github.com/manoloGith/wordcraft)**,
un fork de `storytold/wordcraft`, en la rama **`dexcraft/android`**: el `main` de upstream (WordCraft
0.15.1, `060768f`) más tres commits que DexWordCraft necesita en Android. En escritorio y en la web no cambia nada:
lo de Android se compila solo para `target_os = "android"`, y lo demás son funciones que solo
usa la app Android.

| Cambio | Por qué |
|---|---|
| Carpetas de fuentes en Android (`crates/fonts/src/fontdb/names.rs`: `/system/fonts` y `/product/fonts`) y nombres de archivo de Android en las fuentes de reserva CJK y árabe de la interfaz (`crates/fonts/src/ui_fallback.rs`) | Android guarda ahí sus fuentes (Roboto, Noto Serif, Noto CJK, árabe, hebreo, tailandés…) y no tiene fontconfig. Sin esto, los documentos solo tendrían las fuentes que WordCraft incrusta. |
| Una sola ventana en Android (`crates/ui-egui/src/windows.rs`) | Una actividad de Android tiene una ventana. Los documentos se cambian dentro de ella, como en la versión web de WordCraft, en lugar de abrir una ventana nativa por documento. |
| «Ayuda ▸ Buscar actualizaciones» en Android (`crates/ui-egui/src/updates_ui.rs`) | La autoactualización de escritorio no sirve en Android. La app la desactiva con `disable_self_update()`, que ya existía, y el diálogo dice que la actualiza la tienda o el instalador. |
| Copia de recuperación inmediata (`Session::save_recovery_now`, `crates/engine/src/recovery.rs`) | La X de DeX no se puede interceptar. Al perder el foco, la app escribe ya la copia de recuperación de cada documento con cambios, en lugar de esperar a los 10 s de WordCraft. Con tests. |
| Enganche bajo la cinta (`Services::below_ribbon`, `crates/ui-egui/src/app/frame.rs`) | Para dibujar las pestañas de documentos de craft-android entre la cinta y la página. Genérico: el host dibuja lo que quiera y usa los comandos de la app. Con test (kittest). |

Como en GridCraft, **no hacen falta los shims de `rfd` y `arboard`**. La interfaz de WordCraft
recibe del binario que la arranca (`Services`) los diálogos de archivos, el portapapeles y la
impresión, así que la app Android le da los suyos (`apps/dexwordcraft/src/android/mod.rs`). Tampoco
hace falta un diálogo asíncrono nuevo: `Services::file_dialog` ya responde por un canal sin bloquear
el hilo de la interfaz.

## Trabajar en el fork

En `upstream/`, `origin` es el fork y `storytold` el proyecto original:

```bash
git -C upstream checkout dexcraft/android
```

Las normas de `upstream/AGENTS.md` siguen valiendo: nada de `unwrap` ni `panic`, `cargo fmt` con
su `rustfmt.toml` y clippy sin avisos. Después de hacer commit en `upstream/`, sube la rama y
registra aquí el nuevo commit del submódulo:

```bash
git -C upstream push origin dexcraft/android
```

```bash
git add upstream && git commit -m "upstream: …"
```

## Actualizar a una versión nueva de WordCraft

```bash
scripts/sync-upstream.sh
```

El script:

1. Rebasa `dexcraft/android` sobre el `main` de storytold.
2. Comprueba que los `[patch.crates-io]` de este workspace siguen coincidiendo con los de upstream
   y que winit sigue en la revisión de craft-android.
3. Vuelve a tomar el `Cargo.lock` de upstream.

Después hay que compilar, probar, subir la rama con `--force-with-lease` y hacer commit del
submódulo.
