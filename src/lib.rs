pub mod wav;

pub mod music {
    mod song;
    mod part;
    mod note;
    mod serializable;

    pub use song::Song as Song;
    pub use part::Part as Part;
    pub use note::Note as Note;
    pub use serializable::Serializable as Serializable;
}

pub mod ui {
    mod song_editor;

    pub mod choice_ui;
    pub use song_editor::SongEditor as SongEditor;
}