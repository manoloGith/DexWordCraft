//! DexWordCraft in a desktop window, to iterate without a device:
//! `cargo run -p dexwordcraft --features desktop -- [--sample] [document.docx…]`. No system
//! pickers or clipboard here (the Android build provides them): WordCraft shows its own file
//! browser instead.

fn main() {
    let mut launch = dexwordcraft::Launch::default();
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--sample" => launch.sample = true,
            _ => launch.files.push(a),
        }
    }
    dexwordcraft::run(dexwordcraft::native_options, |cc| {
        log::info!("dexwordcraft: {}", dexwordcraft::renderer_line(cc));
        let services = wordcraft_ui_egui::Services { save_prefs: Some(Box::new(dexwordcraft::prefs::try_save)), ..Default::default() };
        let word = dexwordcraft::DexWord::new(cc, &launch, services);
        Box::new(craft_android::Shell::new(word).on_focus_lost(|w| w.save_prefs()).title(dexwordcraft::DexWord::title))
    });
}
