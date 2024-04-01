use std::{cell::RefCell, rc::Rc};

use song_maker::music::{Song, SongEditor};

slint::include_modules!();

fn song_model_from_editor(editor: &SongEditor) -> slint::ModelRc<slint::SharedString> {
    let songs: Vec<slint::SharedString> = editor.loaded_songs.iter().map(|song| slint::SharedString::from(song.name.to_string())).collect();
    slint::ModelRc::from(songs.as_slice())
}
fn main() -> Result<(), slint::PlatformError> {
    let song_editor = Rc::new(RefCell::new(SongEditor::new()));
    let ui = AppWindow::new()?;
    ui.set_songs(song_model_from_editor(&song_editor.borrow_mut()));
    let ui_weak = ui.as_weak();
    let song_editor_copy = song_editor.clone();
    ui.on_add_song(move || {
        song_editor_copy.borrow_mut().loaded_songs.push(Song::new("New Song".to_string(), 120));
        ui_weak.upgrade().unwrap().set_songs(song_model_from_editor(&song_editor_copy.borrow_mut()));
    });
    let ui_weak = ui.as_weak();
    let song_editor_copy = song_editor.clone();
    ui.on_load_song(move || {
        if let Ok(()) = song_editor_copy.borrow_mut().load_song() {
            ui_weak.upgrade().unwrap().set_songs(song_model_from_editor(&song_editor_copy.borrow_mut()));
        }
    });
    let ui_weak = ui.as_weak();
    let song_editor_copy = song_editor.clone();
    ui.on_delete_song(move |index| {
        song_editor_copy.borrow_mut().loaded_songs.remove(index as usize);
        ui_weak.upgrade().unwrap().set_songs(song_model_from_editor(&song_editor_copy.borrow_mut()));
    });
    ui.run()
}
