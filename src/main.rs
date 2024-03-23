use simple_files::{music::SongEditor, ui::song_editor_cli::ui};

fn main() {
    let mut song_editor = SongEditor::new();
    ui(&mut song_editor);
}