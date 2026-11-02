// The mods' keys under the game's, each mod's under its name, with the
// same change button as Key Binds; a key used twice shows in yellow.
fn modcontrols_rows(
    ui: &mut OptionsUi,
    s: &Settings,
    w: f32,
    value: (Vec<Widget>, f32, f32, f32),
) -> (Vec<Widget>, f32, f32, f32) {
    let (mut out, top, bottom, content) = value;
    if ui.page() != Page::KeyBinds {
        return (out, top, bottom, content);
    }
    let chosen = |id: &str, name: &str, default: &str| {
        s.mods
            .keys
            .get(id)
            .and_then(|c| c.get(name))
            .map_or(default.to_owned(), Clone::clone)
    };
    let mut used = Vec::new();
    for m in s.mods.installed.iter() {
        for [name, _, default] in m.native.iter().flat_map(|p| p.keys.iter()) {
            used.push(chosen(&m.id, name, default));
        }
    }
    let query = ui.settings_search.value.to_lowercase();
    let row_left = (w / 2.0).floor() - 170.0;
    let change_x = row_left + 348.0 - 60.0 - 80.0;
    let first = out.len();
    let mut y = top + content - 2.0 - ui.scroll;
    for (i, m) in s.mods.installed.iter().enumerate() {
        let Some(page) = &m.native else {
            continue;
        };
        let wanted: Vec<usize> = (0..page.keys.len())
            .filter(|&k| query.is_empty() || page.keys[k][1].to_lowercase().contains(&query))
            .collect();
        if wanted.is_empty() {
            continue;
        }
        let name = m.title.rsplit_once(' ').map_or(m.title.as_str(), |t| t.0);
        out.push(Widget {
            x: 0.0,
            y: y + 7.0,
            w,
            h: 9.0,
            kind: WKind::Text {
                text: Text::literal(name).with_style(&Style {
                    bold: Some(true),
                    underlined: Some(true),
                    ..Style::default()
                }),
                color: u32::MAX,
                centered: true,
            },
            list: false,
            row: 0,
        });
        y += 20.0;
        for k in wanted {
            let [key_name, label, default] = &page.keys[k];
            let key = chosen(&m.id, key_name, default);
            // An unchanged letter works by what the keyboard writes on it.
            let unchanged = s.mods.keys.get(&m.id).is_none_or(|c| !c.contains_key(key_name.as_str()));
            let letter = default.strip_prefix("key.keyboard.").filter(|l| l.len() == 1);
            let mut message = match letter.filter(|_| unchanged) {
                Some(l) => Text::literal(l.to_uppercase()),
                None => keybinds::display_name(&key),
            };
            let twice = key != "key.keyboard.unknown"
                && (used.iter().filter(|u| **u == key).count() > 1
                    || MAPPINGS.iter().any(|b| s.keys.get(b.0) == key));
            if twice {
                message = Text::literal("[ ")
                    .append(message.with_color(text::WHITE))
                    .append(" ]")
                    .with_color(text::YELLOW);
            }
            if ui.selected_native_key == Some((i, k)) {
                message = Text::literal("> ")
                    .append(message.with_color(text::WHITE))
                    .append(" <")
                    .with_color(text::YELLOW);
            }
            out.push(Widget {
                x: row_left + 2.0,
                y: y + 6.0,
                w: 200.0,
                h: 9.0,
                kind: WKind::Text {
                    text: Text::literal(label.clone()),
                    color: u32::MAX,
                    centered: false,
                },
                list: false,
                row: 0,
            });
            out.push(button(change_x, y, 130.0, message, Act::NativeKey(i, k)));
            y += 20.0;
        }
    }
    let content = y + ui.scroll - top + 2.0;
    out[first..].iter_mut().for_each(|w| w.list = true);
    // The page clamped the scroll to its own rows: allow the new ones.
    let max = (content - (bottom - top)).max(0.0);
    let wanted = modcontrols::REQUESTED.with(|r| r.get()).clamp(0.0, max);
    let moved = wanted - ui.scroll;
    if moved != 0.0 {
        ui.scroll = wanted;
        out.iter_mut().filter(|w| w.list).for_each(|w| w.y -= moved);
    }
    (out, top, bottom, content)
}
