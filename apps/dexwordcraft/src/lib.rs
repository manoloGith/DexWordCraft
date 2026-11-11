//! DexWordCraft: WordCraft's desktop interface on Android, for Samsung DeX and Android's desktop
//! windowing. The Android app (`android/`) is a `GameActivity` (craft-android's `CraftActivity`)
//! that loads this library and calls `android_main`; egui draws the whole window with wgpu
//! (Vulkan or GLES), or with OpenGL ES through glow when wgpu can't start.
//!
//! The window is WordCraft's own interface ([`WordApp`]), started as its desktop binary starts it
//! (`upstream/apps/wordcraft/src/main.rs`), in one window: the documents the desktop opens in
//! native windows of their own switch inside this one, as in WordCraft's web version, since an
//! Android activity has one window (the fork's `dexcraft/android` branch).

pub mod prefs;
pub mod tabs;

#[cfg(target_os = "android")]
pub(crate) mod android;

use wordcraft_engine::Session;
use wordcraft_ui_egui::{Services, WordApp};

/// What to start with.
#[derive(Clone, Debug, Default)]
pub struct Launch {
    /// Documents to open (paths).
    pub files: Vec<String>,
    /// Open the sample document instead of the Start screen.
    pub sample: bool,
}

/// WordCraft's interface as an eframe app: what the desktop binary's `App` does, minus the native
/// document windows and their geometry, the macOS integration, the control channel and the
/// self-update.
pub struct DexWord {
    pub app: WordApp,
}

impl DexWord {
    /// WordCraft as its desktop binary starts it, with the host's `services` and the document
    /// tabs below the ribbon ([`tabs`]).
    pub fn new(cc: &eframe::CreationContext<'_>, launch: &Launch, mut services: Services) -> DexWord {
        services.below_ribbon = Some(Box::new(tabs::show));
        prefs::load_user_fonts();
        // The system region picks the new document's paper, the units and the decimal separator.
        let region = wordcraft_ui_egui::i18n::system_region();
        let doc = if launch.sample { wordcraft_engine::sample::sample_document() } else { wordcraft_engine::blank_document(&region) };
        let mut session = Session::new(doc);
        session.system_region = region;
        let mut app = WordApp::new(session, services);
        prefs::load(&mut app);
        // After the preferences, so the reason Help › Check for Updates gives is in the interface
        // language: on Android the store or installer updates the app (the fork's message).
        app.disable_self_update();
        prefs::load_user_dictionary(&mut app);
        // Language packs from the suite's font store; an Install click is the person's own.
        wordcraft_engine::packs::use_font_store(true);
        // A region chosen in File › Options replaces the system's for the first blank document too.
        if !launch.sample && app.session.prefs.region.is_some() {
            let doc = app.session.blank_document();
            app.replace_startup_document(doc);
        }
        app.watch_desktop_theme(&cc.egui_ctx);
        // Local recovery: a copy of each changed document every few seconds, offered at the next
        // start. On Android it is what keeps the work when the system closes the app (DeX's X
        // can't be intercepted, and a background app can be killed).
        if let Some(dir) = prefs::settings_dir() {
            let ctx = cc.egui_ctx.clone();
            if let Err(e) = app.session.enable_recovery(dir.join("recovery"), std::sync::Arc::new(move || ctx.request_repaint())) {
                log::warn!("dexwordcraft: local recovery unavailable: {e}");
            }
        }
        if launch.files.is_empty() && !launch.sample {
            app.start_screen();
        }
        let mut word = DexWord { app };
        for f in &launch.files {
            word.open(f);
        }
        word
    }

    /// The window title DeX shows in its taskbar: the active document's, as WordCraft's own.
    pub fn title(&self) -> String {
        let docs = self.app.session.documents();
        match docs.iter().find(|d| d.active) {
            Some(d) => format!("{}{} - DexWordCraft", wordcraft_ui_egui::documents::display_title(d), if d.dirty { " •" } else { "" }),
            None => "DexWordCraft".to_owned(),
        }
    }

    /// Open a document as File › Open does ("Open with", shared and dropped files); a Word
    /// template starts a new document based on it.
    pub fn open(&mut self, path: &str) {
        open_document(&mut self.app, path);
    }

    /// The preferences, written now: Android may end the process without calling `on_exit`.
    pub fn save_prefs(&self) {
        prefs::save(&self.app);
    }

    /// The window lost focus: DeX's close button (which the app can't intercept), the home
    /// screen, another app. The preferences and a recovery copy of every document with unsaved
    /// changes are written now, so the next start offers to recover them.
    pub fn focus_lost(&mut self) {
        self.save_prefs();
        self.app.session.save_recovery_now();
    }
}

impl eframe::App for DexWord {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "android")]
        for path in craft_android::android::events::opened() {
            self.open(&path);
        }
        self.app.logic(ctx);
        // As on the desktop: the close (Back, through craft-android's shell) asks about unsaved
        // changes first and waits for a print job; File › Exit closes once that has been asked.
        let closing = ctx.input(|i| i.viewport().close_requested());
        if closing && (self.app.closes_a_window() || !self.app.close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        } else if self.app.quit_requested && !self.app.printing.busy() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        // (craft-android's Shell has already made Ctrl+C, X and V copy, cut and paste with the
        // system clipboard.) WordCraft opens the files dropped on the window itself.
        self.app.raw_input_hook(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.ui(ui);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.app.session.finish_recovery();
        self.save_prefs();
    }
}

/// Open a document handed over by another app ("Open with", DexFiles, "Share", the start
/// command line) as File › Open does, and leave the Start screen (Backstage) for it, as WordCraft
/// does with documents Finder hands over on macOS: the app starts on the Start screen before the
/// document arrives. A document that can't be opened says why in the status bar, also out of
/// Backstage, where that shows.
pub fn open_document(app: &mut WordApp, path: &str) {
    if let Err(e) = app.open_from_platform(path, None) {
        app.report_open_failure(path, &e);
    }
    app.ui.backstage = false;
}

/// The window's options, shared by Android and the desktop binary.
pub fn native_options(renderer: eframe::Renderer) -> eframe::NativeOptions {
    eframe::NativeOptions {
        renderer,
        viewport: egui::ViewportBuilder::default()
            .with_title("DexWordCraft")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 420.0])
            .with_app_id("io.github.dexwordcraft")
            .with_drag_and_drop(true),
        ..Default::default()
    }
}

/// Run the window with wgpu, then with glow if wgpu can't start (old or unusual GPU drivers).
/// `create` builds the app once a renderer has started.
pub fn run(
    mut options: impl FnMut(eframe::Renderer) -> eframe::NativeOptions,
    mut create: impl FnMut(&eframe::CreationContext<'_>) -> Box<dyn eframe::App>,
) {
    let mut last_error = None;
    for renderer in [eframe::Renderer::Wgpu, eframe::Renderer::Glow] {
        let result = eframe::run_native("DexWordCraft", options(renderer), Box::new(|cc| Ok(create(cc))));
        match result {
            Ok(()) => return,
            Err(e) => {
                log::error!("dexwordcraft: the {renderer} renderer could not start: {e}");
                #[cfg(target_os = "android")]
                craft_android::android::diag(&format!("the {renderer} renderer could not start: {e}"));
                last_error = Some(e);
            }
        }
    }
    if let Some(e) = last_error {
        log::error!("dexwordcraft: no renderer could start: {e}");
    }
}

/// The WordCraft release `upstream/` is at (its workspace version).
pub fn wordcraft_version() -> &'static str {
    const MANIFEST: &str = include_str!("../../../upstream/Cargo.toml");
    MANIFEST.lines().find_map(|l| l.strip_prefix("version = \"").and_then(|v| v.strip_suffix('"'))).unwrap_or("unknown")
}

/// A POSIX locale (`es_ES.UTF-8`, `sr_RS@latin`, as craft-android puts in `LANG`) as the BCP 47
/// tag WordCraft's `WORDCRAFT_LANGUAGE` takes (`es-ES`); `None` for `C`, `POSIX` or nothing.
pub fn language_tag(posix: &str) -> Option<String> {
    let base = posix.split(['.', '@']).next().unwrap_or_default().trim();
    if base.is_empty() || base.eq_ignore_ascii_case("C") || base.eq_ignore_ascii_case("POSIX") {
        return None;
    }
    Some(base.replace('_', "-"))
}

/// The renderer that started, for the diagnostics report.
pub fn renderer_line(cc: &eframe::CreationContext<'_>) -> String {
    match &cc.wgpu_render_state {
        Some(state) => {
            let adapter = state.adapter.get_info();
            format!("renderer wgpu, {} ({:?}, {:?})", adapter.name, adapter.device_type, adapter.backend)
        }
        None if cc.gl.is_some() => "renderer glow (OpenGL ES)".to_owned(),
        None => "renderer unknown".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_android_locale_becomes_a_language_tag() {
        assert_eq!(language_tag("es_ES.UTF-8").as_deref(), Some("es-ES"));
        assert_eq!(language_tag("pt_BR").as_deref(), Some("pt-BR"));
        assert_eq!(language_tag("sr_RS@latin").as_deref(), Some("sr-RS"));
        assert_eq!(language_tag("de").as_deref(), Some("de"));
        assert_eq!(language_tag("C.UTF-8"), None);
        assert_eq!(language_tag("POSIX"), None);
        assert_eq!(language_tag(""), None);
    }

    /// The app starts on the Start screen; a document another app hands over a moment later
    /// (DexFiles, "Open with") shows instead of it.
    #[test]
    fn a_document_handed_over_after_start_leaves_the_start_screen() {
        let dir = std::env::temp_dir().join(format!("dexwordcraft-open-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("Letter.txt");
        std::fs::write(&path, "Dear editor").unwrap();
        let mut app = WordApp::new(Session::new(wordcraft_engine::doc::Document::new()), Services::default());
        app.start_screen();
        assert!(app.ui.backstage, "the app starts on the Start screen");
        open_document(&mut app, path.to_str().unwrap());
        assert!(!app.ui.backstage, "the opened document shows");
        assert_eq!(app.session.doc.plain_text(wordcraft_engine::doc::StoryRef::Body).trim(), "Dear editor");
        // One that can't be opened leaves it too, so the status bar can say why.
        app.start_screen();
        open_document(&mut app, dir.join("Missing.docx").to_str().unwrap());
        assert!(!app.ui.backstage);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn upstream_has_a_version() {
        assert!(wordcraft_version().split('.').count() == 3, "{}", wordcraft_version());
    }
}
