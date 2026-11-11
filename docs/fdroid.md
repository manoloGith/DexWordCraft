# Distribución: GitHub, Obtainium y F-Droid

## GitHub y Obtainium (ya disponible)

Cada versión se publica como release de GitHub con el APK firmado (`docs/releasing.md`).
[Obtainium](https://github.com/ImranR98/Obtainium) puede seguir el repositorio
`https://github.com/manoloGith/DexWordCraft` e instalar las actualizaciones según salen.

## F-Droid (preparado, sin enviar)

El repositorio tiene los metadatos en el formato que F-Droid lee
(`fastlane/metadata/android/{en-US,es-ES}`: título, descripciones, notas por `versionCode`, icono y
capturas). Para pedir la inclusión hay que abrir una *merge request* en
[fdroiddata](https://gitlab.com/fdroid/fdroiddata) con un archivo `metadata/io.github.dexwordcraft.yml`.
Este es el borrador:

```yaml
Categories:
  - Office
License: MIT OR Apache-2.0
SourceCode: https://github.com/manoloGith/DexWordCraft
IssueTracker: https://github.com/manoloGith/DexWordCraft/issues
Changelog: https://github.com/manoloGith/DexWordCraft/releases

AutoName: DexWordCraft

RepoType: git
Repo: https://github.com/manoloGith/DexWordCraft

Builds:
  - versionName: 0.1.0
    versionCode: 1
    commit: v0.1.0
    subdir: apps/dexwordcraft/android/app
    submodules: true
    srclibs:
      - craft-i18n@1e515422afdda2f317af430c1f9d68a5f83935b4
    sudo:
      - apt-get update
      - apt-get install -y cmake
    gradle:
      - yes
    output: build/outputs/apk/release/app-release-unsigned.apk
    prebuild:
      - curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
      - source $HOME/.cargo/env
      - rustup toolchain install $(sed -n 's/^channel = "\(.*\)"/\1/p' ../../../../rust-toolchain.toml)
      - cargo install cargo-ndk --version 4.1.2 --locked
    gradleprops:
      - dexwordcraft.abis=arm64-v8a
      - dexwordcraft.unsigned=true
    ndk: 29.0.14206865
    build:
      - export CRAFT_I18N_DIR=$$craft-i18n$$ CRAFT_I18N_REQUIRED=1

AutoUpdateMode: Version
UpdateCheckMode: Tags ^v[0-9.]+$
CurrentVersion: 0.1.0
CurrentVersionCode: 1
```

Puntos que la revisión de F-Droid mirará:

- **Entradas de compilación:** solo [craft-i18n](https://github.com/storytold/craft-i18n)
  (traducciones, ortografía y guiones de WordCraft), en la revisión de `upstream/craft-i18n.rev`.
  F-Droid no deja descargar nada durante la compilación, así que entra como `srclib`: hay que
  añadir `srclibs/craft-i18n.yml` (repositorio git `https://github.com/storytold/craft-i18n`) en
  la misma *merge request* y actualizar la revisión del borrador cuando upstream la mueva.
  craft-fonts no se usa: las fuentes las pone Android (`/system/fonts`).
- **Firma.** F-Droid firma con su propia clave, salvo que se active la verificación de builds
  reproducibles contra el APK de GitHub. La biblioteca nativa ya se compila sin rutas locales
  (`--remap-path-prefix`) y el APK no lleva información de dependencias ni del control de versiones.
- **Dependencias no libres:** no hay ninguna. GameActivity y AndroidX son Apache-2.0; no hay
  servicios de Google, analítica ni anuncios.
- **Sin firmar para F-Droid:** `-Pdexwordcraft.unsigned=true` produce `app-release-unsigned.apk`.
  Sin esa opción ni las variables `DEXWORDCRAFT_KEYSTORE*`, el release se firma con la clave de
  depuración.
