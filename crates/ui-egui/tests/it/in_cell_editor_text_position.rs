//! The in-cell editor draws the text where the cell shows it, as in Excel: starting to edit a
//! bottom-aligned (the default) or centred value doesn't move it up to the top of the cell (#2334).

use egui::{Context, RawInput, Rect, Shape, pos2, vec2};
use gridcraft_engine::Session;
use gridcraft_engine::core::CellRef;
use gridcraft_ui_egui::{SheetApp, grid::Geo, theme};
use serde_json::json;

fn frame(ctx: &Context, app: &mut SheetApp) -> egui::FullOutput {
    let mut output =
        ctx.run_ui(RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1200.0, 800.0))), ..Default::default() }, |ui| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        });
    output.textures_delta.clear();
    output
}

/// The top of each text shape reading `text` that starts in `cell` (the formula bar's copy is
/// elsewhere), in paint order.
fn text_tops(out: &egui::FullOutput, text: &str, cell: Rect) -> Vec<f32> {
    out.shapes
        .iter()
        .filter_map(|s| match &s.shape {
            Shape::Text(t) if t.galley.job.text == text => Some(pos2(t.pos.x, t.pos.y + t.galley.rows.first().map_or(0.0, |r| r.min_y()))),
            _ => None,
        })
        .filter(|p| cell.expand(4.0).contains(*p))
        .map(|p| p.y)
        .collect()
}

/// Where B3's text `input` is drawn before and while it is edited, after `setup` has run.
fn shown_and_edited(input: &str, setup: &[(&str, serde_json::Value)]) -> (f32, f32) {
    let ctx = Context::default();
    theme::apply(&ctx, false);
    let mut session = Session::new();
    session.new_workbook();
    let mut app = SheetApp::new(session, Default::default());
    app.run("cell.set", json!({"cell": "B3", "input": input})).unwrap();
    for (id, params) in setup {
        app.run(id, params.clone()).unwrap();
    }
    app.run("selection.set", json!({"cell": "B3"})).unwrap();
    for _ in 0..4 {
        frame(&ctx, &mut app);
    }
    let sheet = app.session.active().unwrap().wb.active().unwrap();
    let cell = Geo::new(sheet, app.grid.rect.unwrap(), app.view().scroll).cell_rect(sheet, CellRef::parse("B3").unwrap());
    let shown = *text_tops(&frame(&ctx, &mut app), input, cell).first().expect("the cell's text");
    app.begin_edit(None, false);
    for _ in 0..4 {
        frame(&ctx, &mut app);
    }
    // The editor paints over the cell, so its text comes last.
    let edited = *text_tops(&frame(&ctx, &mut app), input, cell).last().expect("the editor's text");
    (shown, edited)
}

fn assert_same_place(setup: &[(&str, serde_json::Value)], what: &str) {
    assert_text_in_place("Wxyz", setup, what);
}

fn assert_text_in_place(input: &str, setup: &[(&str, serde_json::Value)], what: &str) {
    let (shown, edited) = shown_and_edited(input, setup);
    // The field snaps its text to a 1/16 pt grid; the jump this guards against was 0.6 pt and up.
    assert!((shown - edited).abs() < 0.1, "{what}: shown at y {shown}, edited at y {edited}");
}

#[test]
fn a_default_row_keeps_the_text_in_place() {
    assert_same_place(&[], "default row");
}

#[test]
fn a_tall_row_keeps_bottom_aligned_text_at_the_bottom() {
    assert_same_place(&[("home.rowHeight", json!({"rows": "3:3", "points": 90}))], "tall row, bottom");
}

#[test]
fn a_tall_row_keeps_centred_and_top_aligned_text_in_place() {
    let tall = ("home.rowHeight", json!({"rows": "3:3", "points": 90}));
    assert_same_place(&[tall.clone(), ("home.alignMiddle", json!({"range": "B3"}))], "tall row, middle");
    assert_same_place(&[tall, ("home.alignTop", json!({"range": "B3"}))], "tall row, top");
}

#[test]
fn a_tall_row_keeps_wrapped_text_in_place() {
    assert_same_place(&[("home.rowHeight", json!({"rows": "3:3", "points": 90})), ("home.wrapText", json!({"range": "B3"}))], "tall row, wrap text");
}

#[test]
fn zoomed_in_keeps_the_text_in_place() {
    assert_same_place(&[("view.zoom", json!({"percent": 200}))], "200% zoom");
}

#[test]
fn a_line_break_keeps_both_lines_in_place() {
    // A line break turns Wrap Text on and fits the row to the two lines, a little less than the
    // editor's own estimate for them.
    assert_text_in_place("Wxyz\nWxyz", &[], "line break, fitted row");
}

#[test]
fn justified_text_keeps_its_place() {
    assert_same_place(&[("home.rowHeight", json!({"rows": "3:3", "points": 90})), ("home.justify", json!({"range": "B3"}))], "tall row, justified");
}
