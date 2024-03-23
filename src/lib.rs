pub mod wav;

pub mod music {
    mod song;
    mod part;
    mod note;
    mod serializable;
    mod song_editor;

    pub use song::Song as Song;
    pub use part::Part as Part;
    pub use note::Note as Note;
    pub use serializable::Serializable as Serializable;
    pub use song_editor::SongEditor as SongEditor;
}

pub mod ui {
    pub mod choice_ui;
    pub mod song_edit_cli;
}