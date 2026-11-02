// The mod's row in Options > Mods.
fn modcontrols_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "modcontrols".into(),
        title: "Mod Key Binds 1.0.0".into(),
        description: "Lists every mod's keys in Options > Controls > Key Binds, as Fabric mods' keys are.".into(),
        keys: Vec::new(),
    });
}
