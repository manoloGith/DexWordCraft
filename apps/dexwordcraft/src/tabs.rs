//! The document tabs below WordCraft's ribbon: craft-android's tab strip
//! ([`craft_android::tabs`]) drawn through the fork's `Services::below_ribbon`, one tab per open
//! document. What a tab asks for runs as WordCraft's own commands, so closing a tab with unsaved
//! changes asks Save / Don't Save / Cancel exactly as File › Close does.

use craft_android::tabs::{Tab, TabAction, TabStrip};
use serde_json::{Value, json};
use wordcraft_ui_egui::WordApp;

/// Draw the tabs and run what was clicked. No tabs in a window with no document open.
pub fn show(app: &mut WordApp, ui: &mut egui::Ui) {
    let docs = app.session.documents();
    if docs.is_empty() {
        return;
    }
    let tabs: Vec<Tab> = docs.iter().map(|d| Tab::new(d.id, wordcraft_ui_egui::documents::display_title(d), d.dirty, d.active)).collect();
    let t = wordcraft_ui_egui::i18n::t;
    let out = TabStrip::new(&tabs).new_tooltip(t("Blank document")).close_tooltip(t("Close")).show_panel(ui);
    let Some(action) = out.action else { return };
    let active = docs.iter().find(|d| d.active).map(|d| d.id);
    for (command, params) in commands(action, active) {
        if let Err(e) = app.run(command, params) {
            log::warn!("dexwordcraft: tab {action:?}: {command}: {e}");
            break;
        }
    }
}

/// The WordCraft commands for a tab's action, given the shown document. Closing another
/// document's tab shows it first, so File › Close (and its Save prompt) is about that one.
pub fn commands(action: TabAction, active: Option<u64>) -> Vec<(&'static str, Value)> {
    match action {
        TabAction::Activate(id) => vec![("view.switchWindows", json!({"id": id}))],
        TabAction::Close(id) if active == Some(id) => vec![("file.close", json!({}))],
        TabAction::Close(id) => vec![("view.switchWindows", json!({"id": id})), ("file.close", json!({}))],
        TabAction::New => vec![("file.new", json!({}))],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wordcraft_engine::Session;
    use wordcraft_ui_egui::Services;

    #[test]
    fn each_action_is_a_wordcraft_command() {
        assert_eq!(commands(TabAction::Activate(4), Some(2)), [("view.switchWindows", json!({"id": 4}))]);
        assert_eq!(commands(TabAction::Close(2), Some(2)), [("file.close", json!({}))]);
        assert_eq!(commands(TabAction::Close(4), Some(2)), [("view.switchWindows", json!({"id": 4})), ("file.close", json!({}))]);
        assert_eq!(commands(TabAction::New, None), [("file.new", json!({}))]);
    }

    fn text(app: &WordApp) -> String {
        app.session.doc.plain_text(wordcraft_engine::doc::StoryRef::Body).trim().to_owned()
    }

    fn run(app: &mut WordApp, action: TabAction) {
        let active = app.session.documents().iter().find(|d| d.active).map(|d| d.id);
        for (command, params) in commands(action, active) {
            app.run(command, params).unwrap();
        }
    }

    /// The commands do what the tabs promise on WordCraft itself: switch, new, and close a
    /// document without changes at once while one with changes asks first.
    #[test]
    fn the_commands_switch_open_and_close_documents() {
        let mut app = WordApp::new(Session::new(wordcraft_engine::doc::Document::new()), Services::default());
        app.run("text.insert", json!({"text": "First draft"})).unwrap();
        let first = app.session.document_id();
        run(&mut app, TabAction::New);
        let second = app.session.document_id();
        assert_ne!(first, second);
        assert_eq!(app.session.documents().len(), 2);
        run(&mut app, TabAction::Activate(first));
        assert_eq!(app.session.document_id(), first);
        assert_eq!(text(&app), "First draft");
        // The untouched second document closes from its tab without a question.
        run(&mut app, TabAction::Close(second));
        assert_eq!(app.session.documents().len(), 1);
        assert_eq!(app.session.document_id(), first);
        // The first one has unsaved changes: closing it asks, and leaves it open meanwhile.
        run(&mut app, TabAction::Close(first));
        assert_eq!(app.session.documents().len(), 1, "a document with changes is not closed without asking");
        assert!(app.session.dirty);
        assert!(app.dialog.is_some(), "the Save / Don't Save / Cancel prompt is up");
    }
}
