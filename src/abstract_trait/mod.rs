mod note;
mod user;

pub use note::{DynNoteRepository, DynNoteService, NoteRepositoryTrait, NoteServiceTrait};
pub use user::{DynUserRepository, DynUserService, UserRepositoryTrait, UserServiceTrait};
