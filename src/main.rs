use simple_files::{music::SongEditor, ui::song_edit_cli::ui};

fn main() {
    let mut song_editor = SongEditor::new();
    ui(&mut song_editor);
}