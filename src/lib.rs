pub mod wav;

pub mod music {
    mod song;
    mod part;
    mod note;
    mod song_editor;
    mod serializable;

    pub use song::Song as Song;
    pub use part::Part as Part;
    pub use note::Note as Note;
    pub use song_editor::SongEditor as SongEditor;
    pub use serializable::Serializable as Serializable;
}