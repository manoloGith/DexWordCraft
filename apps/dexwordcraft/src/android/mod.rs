//! The Android side: the entry point and WordCraft's [`Services`] on craft-android (the Storage
//! Access Framework's pickers, the clipboard, printing). Everything else (the JNI bridge, the
//! environment, the activity's input, the lifecycle) is craft-android's.

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, channel};

use craft_android::android::{self as platform, AndroidApp, clipboard::Clip, pickers::PickKind};
use wordcraft_ui_egui::paste_picture::ClipboardPicture;
use wordcraft_ui_egui::{FileDialogAnswer, FileDialogRequest, Services};

use crate::{DexWord, Launch};

/// Entry point, called by android-activity on the activity's own thread.
#[allow(unsafe_code)] // `no_mangle` is how android-activity's glue finds the entry point
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let config = platform::Config { log_tag: "dexwordcraft", asset_dirs: Vec::new() };
    let Some(started) = platform::start(&app, &config) else { return };
    set_language();
    platform::diag(&format!("DexWordCraft {} (WordCraft {})", env!("CARGO_PKG_VERSION"), crate::wordcraft_version()));
    let print_dir = Arc::new(started.data_dir.join("cache/print"));
    let launch = Launch::default();
    crate::run(
        |renderer| platform::native_options(&app, crate::native_options(renderer)),
        |cc| {
            platform::events::set_context(cc.egui_ctx.clone());
            platform::diag(&crate::renderer_line(cc));
            let word = DexWord::new(cc, &launch, services(print_dir.clone()));
            // craft-android's shell: the activity's mouse, keyboard and insets, Back as the close
            // button (WordCraft asks about unsaved documents), the title for DeX's taskbar.
            Box::new(craft_android::Shell::new(word).on_focus_lost(|w| w.focus_lost()).title(DexWord::title))
        },
    );
    // The window closed (WordCraft allowed it, after asking about unsaved changes).
    platform::finish();
}

/// WordCraft's interface language follows `WORDCRAFT_LANGUAGE`, then the system's. craft-android
/// has put the system's in `LANG` (`es_ES.UTF-8`); hand it over in the form WordCraft reads.
/// Before the interface starts its threads.
#[allow(unsafe_code)] // `set_var` before any other thread reads the environment
fn set_language() {
    if std::env::var_os("WORDCRAFT_LANGUAGE").is_some() {
        return;
    }
    let Some(tag) = std::env::var("LANG").ok().and_then(|l| crate::language_tag(&l)) else { return };
    platform::diag(&format!("language {tag}"));
    // SAFETY: called on `android_main`'s thread right after craft-android's `start` (which sets
    // `HOME`, the XDG folders and `LANG` the same way), before eframe or WordCraft start threads.
    unsafe { std::env::set_var("WORDCRAFT_LANGUAGE", tag) };
}

/// Wait for a picker's answer on a worker thread, then hand it to `then`. The picker itself must
/// be started on the interface thread, which can't wait: the activity's pause waits for it.
fn when_picked(picked: platform::pickers::Picked, then: impl FnOnce(Vec<PathBuf>) + Send + 'static) {
    let spawned = std::thread::Builder::new().name("picker".into()).spawn(move || then(block_on(picked)));
    if let Err(e) = spawned {
        log::error!("dexwordcraft: picker thread: {e}");
    }
}

/// craft-android's picker futures block inside `poll` until the answer comes, so one poll is
/// the whole wait.
fn block_on<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
        std::thread::yield_now();
    }
}

/// WordCraft's filters (`("Documents", ["docx", …])`) as craft-android takes them. A filter with
/// `*` (All files) makes the picker offer every file.
fn filters(list: &[(&str, &[&str])]) -> Vec<(String, Vec<String>)> {
    list.iter().map(|(name, exts)| ((*name).to_owned(), exts.iter().map(|e| (*e).to_owned()).collect())).collect()
}

/// WordCraft's file dialogs as the system's pickers. The answer comes later through the channel
/// WordCraft reads on each frame; cancelled is an empty list.
fn file_dialog(request: FileDialogRequest) -> Receiver<FileDialogAnswer> {
    let (tx, rx) = channel();
    let picked = match &request {
        FileDialogRequest::Open { purpose } => {
            platform::pickers::pick(PickKind::OpenFile, None, &filters(wordcraft_ui_egui::open_filters(purpose)), None)
        }
        FileDialogRequest::OpenMany { purpose } => {
            platform::pickers::pick(PickKind::OpenFiles, None, &filters(wordcraft_ui_egui::open_filters(purpose)), None)
        }
        // Save As, the exports and Save as Word Template: the system's Save picker, offering the
        // suggested name's type. (The folder to start in is the picker's own choice.)
        FileDialogRequest::Save { name, .. } => {
            let ext = std::path::Path::new(name).extension().and_then(std::ffi::OsStr::to_str).unwrap_or("docx").to_ascii_lowercase();
            platform::pickers::pick(PickKind::Save, None, &[(ext.to_uppercase(), vec![ext])], Some(name))
        }
    };
    when_picked(picked, move |paths| {
        let paths = paths.iter().filter_map(|p| p.to_str().map(str::to_owned)).filter(|p| !p.is_empty()).collect();
        let _ = tx.send(Ok(paths));
        platform::events::wake();
    });
    rx
}

fn services(print_dir: Arc<PathBuf>) -> Services {
    Services {
        file_dialog: Some(Box::new(file_dialog)),
        // File › Print: WordCraft composes the PDF; Android's print dialog takes it from there
        // (printer, copies, two-sided), as the browser's does for the web version.
        print: Some(Box::new(move |pdf: &[u8]| platform::print::print_pdf(pdf, "WordCraft", &print_dir).map(|_| ()))),
        clipboard_text: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Text(text) | Clip::Html { text, .. } => Some(text),
            _ => None,
        })),
        clipboard_html: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Html { html, .. } => Some(html),
            _ => None,
        })),
        // A formatted copy: its HTML beside its text, so other apps keep the formatting. Android's
        // clipboard has no RTF or MathML.
        set_clipboard_rich: Some(Box::new(|text: &str, html: &str, _rtf: &str, _mathml: &str| {
            let clip = if html.is_empty() { Clip::Text(text.to_owned()) } else { Clip::Html { html: html.to_owned(), text: text.to_owned() } };
            match platform::clipboard::set(clip) {
                Ok(()) => true,
                Err(e) => {
                    log::warn!("dexwordcraft: clipboard: {e}");
                    false
                }
            }
        })),
        clipboard_picture: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Image { width, height, rgba } => Some(ClipboardPicture::Pixels { width, height, rgba }),
            Clip::Files(files) => Some(ClipboardPicture::Files(files)),
            _ => None,
        })),
        save_prefs: Some(Box::new(crate::prefs::try_save)),
        // Personal templates live in `Templates` in the settings folder, made on first save.
        templates_dir: crate::prefs::settings_dir().map(|d| d.join("Templates")),
        ..Default::default()
    }
}
