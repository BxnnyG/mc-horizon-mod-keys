// Remembers the scroll before the page clamps it.
fn modcontrols_scroll(ui: &OptionsUi) {
    modcontrols::REQUESTED.with(|r| r.set(ui.scroll));
}
