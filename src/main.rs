mod editor;
mod tui;

use crate::editor::{Editor, document, cursor, viewport, mode, command};

fn main() -> color_eyre::Result<()> {
    let mut editor = Editor::new();
    let _ = tui::run(&mut editor);

    Ok(())
}