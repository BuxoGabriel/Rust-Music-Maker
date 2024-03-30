use song_maker::music::SongEditor;

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let mut song_editor = SongEditor::new();
    let ui = AppWindow::new()?;
    ui.on_load_song(move || {
        song_editor.load_song().unwrap();
    });
    ui.on_add_song(move || {

    });

    ui.run()
}
