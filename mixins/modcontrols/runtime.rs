// Mod Controls: the scroll the Key Binds page was asked for, which it clamps
// to its own rows before the mods' rows are added below them.
use std::cell::Cell;

thread_local! {
    pub(super) static REQUESTED: Cell<f32> = const { Cell::new(0.0) };
}
