//! Enter on a dialog whose command fails shows the error, as clicking OK does (#2189). The Enter
//! that ran OK used to close the message box in the same frame, before it was ever drawn.

use egui::{Event, Key, vec2};
use gridcraft_engine::Session;
use gridcraft_ui_egui::SheetApp;
use serde_json::json;

fn harness() -> egui_kittest::Harness<'static, SheetApp> {
    let mut session = Session::new();
    session.new_workbook();
    session.execute("home.insertSheet", json!({"name": "Taken"})).unwrap();
    let mut h = egui_kittest::Harness::builder().with_size(vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        SheetApp::new(session, Default::default()),
    );
    h.run_steps(6);
    h
}

fn type_and_enter(h: &mut egui_kittest::Harness<'static, SheetApp>, text: &str) {
    for c in text.chars() {
        h.input_mut().events.push(Event::Text(c.to_string()));
        h.run_steps(1);
    }
    h.key_press(Key::Enter);
    h.run_steps(6);
}

fn other_sheet(h: &egui_kittest::Harness<'static, SheetApp>) -> String {
    let wb = &h.state().session.active().unwrap().wb;
    let active = wb.active().unwrap().name.clone();
    wb.sheets.iter().map(|s| s.name.clone()).find(|n| *n != active).unwrap()
}

#[test]
fn enter_on_rename_sheet_with_a_taken_name_shows_why() {
    let mut h = harness();
    let taken = other_sheet(&h);
    h.state_mut().open_dialog("renameSheet", json!({}));
    h.run_steps(6);
    type_and_enter(&mut h, &taken);
    assert!(h.state().dialog.is_some(), "the dialog stays open for another name");
    let message = h.state().message.clone();
    assert!(message.as_ref().is_some_and(|(_, m)| m.contains("already taken")), "{message:?}");
}

#[test]
fn enter_on_rename_sheet_with_a_free_name_still_renames() {
    let mut h = harness();
    h.state_mut().open_dialog("renameSheet", json!({}));
    h.run_steps(6);
    type_and_enter(&mut h, "Budget");
    assert!(h.state().dialog.is_none(), "Enter confirmed the dialog");
    assert!(h.state().message.is_none(), "{:?}", h.state().message);
    let wb = &h.state().session.active().unwrap().wb;
    assert!(wb.sheets.iter().any(|s| s.name == "Budget"), "{:?}", wb.sheets.iter().map(|s| &s.name).collect::<Vec<_>>());
}

#[test]
fn enter_on_find_next_with_nothing_to_find_shows_why() {
    let mut h = harness();
    h.state_mut().open_dialog("find", json!({}));
    h.run_steps(6);
    type_and_enter(&mut h, "zzz");
    let message = h.state().message.clone();
    assert!(message.as_ref().is_some_and(|(_, m)| m.contains("couldn't find")), "{message:?}");
    h.key_press(Key::Enter);
    h.run_steps(6);
    assert!(h.state().message.is_none(), "Enter closes the message: {:?}", h.state().message);
}

#[test]
fn enter_again_closes_the_message_and_keeps_the_dialog() {
    let mut h = harness();
    let taken = other_sheet(&h);
    h.state_mut().open_dialog("renameSheet", json!({}));
    h.run_steps(6);
    type_and_enter(&mut h, &taken);
    assert!(h.state().message.is_some(), "the first Enter shows why");
    // The second Enter belongs to the message box: it closes it, and doesn't run OK again (which
    // would put the same message straight back).
    h.key_press(Key::Enter);
    h.run_steps(6);
    assert!(h.state().message.is_none(), "{:?}", h.state().message);
    assert!(h.state().dialog.is_some(), "the dialog stays open to fix the name");
}
