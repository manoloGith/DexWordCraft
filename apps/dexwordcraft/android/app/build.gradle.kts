import javax.inject.Inject
import org.gradle.process.ExecOperations

plugins {
    alias(libs.plugins.android.application)
}

// The Rust half of the app: this Gradle project is the android/ folder of the `dexwordcraft` crate
// (apps/dexwordcraft), in the cargo workspace at the root of the repository.
val cargoWorkspaceDir: File = rootDir.parentFile.parentFile.parentFile
val rustAbis: List<String> =
    providers.gradleProperty("dexwordcraft.abis").getOrElse("arm64-v8a,x86_64").split(',').map { it.trim() }.filter { it.isNotEmpty() }
// Android 10: the first release with desktop mode on external displays; DeX itself runs on 9+.
val minApi = 29
val libName = "dexwordcraft"

/** rustup's `cargo` (Android Studio may not see rustup's PATH), else `cargo` from PATH; `CARGO` overrides. */
val cargoExecutable: String = run {
    val exe = if (System.getProperty("os.name").lowercase().startsWith("windows")) "cargo.exe" else "cargo"
    val rustup = File(System.getProperty("user.home"), ".cargo/bin/$exe")
    providers.environmentVariable("CARGO").orNull ?: if (rustup.isFile) rustup.absolutePath else exe
}

/** Builds `libdexwordcraft.so` for each ABI with cargo-ndk (release profile unless [release] is false). */
abstract class CargoNdkBuild : DefaultTask() {
    @get:Inject abstract val execOperations: ExecOperations
    @get:Input abstract val cargo: Property<String>
    @get:Input abstract val workspaceDir: Property<String>
    @get:Input abstract val ndkDir: Property<String>
    @get:Input abstract val abis: ListProperty<String>
    @get:Input abstract val apiLevel: Property<Int>
    @get:Input abstract val release: Property<Boolean>
    @get:Input abstract val library: Property<String>
    @get:OutputDirectory abstract val outputDir: DirectoryProperty

    @TaskAction
    fun build() {
        val targets = abis.get().flatMap { listOf("-t", it) }
        val out = outputDir.get().asFile.absolutePath
        val cargoHome = System.getenv("CARGO_HOME")?.takeIf { it.isNotBlank() } ?: File(System.getProperty("user.home"), ".cargo").absolutePath
        // Reproducible: the library mustn't hold the paths it was built in.
        val remap = listOf(
            "--remap-path-prefix=${File(workspaceDir.get()).absolutePath}=/dexwordcraft",
            "--remap-path-prefix=$cargoHome=/cargo",
        )
        val profile = if (release.get()) listOf("--release") else emptyList()
        // craft-i18n (scripts/fetch-build-inputs.sh): WordCraft's translations and spelling data;
        // without it the interface is English only.
        val i18n = System.getenv("CRAFT_I18N_DIR")?.takeIf { it.isNotBlank() }
            ?: File(workspaceDir.get(), ".build-inputs/craft-i18n").takeIf { it.isDirectory }?.absolutePath
        execOperations.exec {
            workingDir(workspaceDir.get())
            environment("ANDROID_NDK_HOME", ndkDir.get())
            if (i18n != null) environment("CRAFT_I18N_DIR", i18n)
            environment("CARGO_ENCODED_RUSTFLAGS", remap.joinToString("\u001f"))
            commandLine(
                listOf(cargo.get(), "ndk") + targets +
                    listOf("--platform", apiLevel.get().toString(), "-o", out, "build") + profile +
                    listOf("--locked", "-p", library.get(), "--lib"),
            )
        }
        // cargo-ndk copies every shared library cargo left behind; only the app's ships.
        File(out).walk().filter { it.isFile && it.name != "lib${library.get()}.so" }.forEach { it.delete() }
    }
}

android {
    namespace = "io.github.dexwordcraft"
    compileSdk = 36
    ndkVersion = "29.0.14206865"

    defaultConfig {
        applicationId = "io.github.dexwordcraft"
        minSdk = minApi
        targetSdk = 36
        versionCode = providers.gradleProperty("dexwordcraft.versionCode").get().toInt()
        versionName = providers.gradleProperty("dexwordcraft.version").get()
        ndk { abiFilters.addAll(rustAbis) }
    }

    // The release key, when the build is given one (the release workflow passes it from the
    // repository's secrets). Without one a release build is signed with the debug key: it installs,
    // but a later release signed with the real key can't update it.
    val releaseKeystore: String? = providers.environmentVariable("DEXWORDCRAFT_KEYSTORE").orNull?.takeIf { it.isNotBlank() }
    signingConfigs {
        if (releaseKeystore != null) {
            create("release") {
                storeFile = file(releaseKeystore)
                storePassword = providers.environmentVariable("DEXWORDCRAFT_KEYSTORE_PASSWORD").orNull
                keyAlias = providers.environmentVariable("DEXWORDCRAFT_KEY_ALIAS").orNull
                keyPassword = providers.environmentVariable("DEXWORDCRAFT_KEY_PASSWORD").orNull
            }
        }
    }

    buildTypes {
        debug {
            applicationIdSuffix = ".debug"
        }
        release {
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            // -Pdexwordcraft.unsigned=true (F-Droid's build): not signed at all, so the store signs it,
            // or checks it against the published APK.
            signingConfig = if (providers.gradleProperty("dexwordcraft.unsigned").orNull == "true") {
                null
            } else {
                signingConfigs.findByName("release") ?: signingConfigs.getByName("debug")
            }
            vcsInfo { include = false }
        }
    }
    dependenciesInfo {
        includeInApk = false
        includeInBundle = false
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

androidComponents {
    onVariants { variant ->
        val name = variant.name.replaceFirstChar { it.uppercase() }
        val cargoBuild = tasks.register<CargoNdkBuild>("cargoBuild$name") {
            group = "rust"
            description = "Builds lib$libName.so for $rustAbis"
            cargo.set(cargoExecutable)
            workspaceDir.set(cargoWorkspaceDir.absolutePath)
            ndkDir.set(androidComponents.sdkComponents.ndkDirectory.map { it.asFile.absolutePath })
            abis.set(rustAbis)
            apiLevel.set(minApi)
            // a debug APK still gets an optimised library: the renderer is unusable at opt-level 1
            release.set(providers.gradleProperty("dexwordcraft.rustDebug").orNull != "true")
            library.set(libName)
            // cargo knows what changed
            outputs.upToDateWhen { false }
        }
        variant.sources.jniLibs?.addGeneratedSourceDirectory(cargoBuild, CargoNdkBuild::outputDir)
    }
}

tasks.configureEach {
    if (name.contains("ArtProfile")) enabled = false
}

dependencies {
    // craft-android's activity (GameActivity, documents, clipboard, printing, input, lifecycle);
    // it brings GameActivity's Java side and AppCompat.
    implementation(project(":craft-activity"))
}
