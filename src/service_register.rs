use std::sync::Arc;

use crate::{
    abstract_trait::{DynNoteRepository, DynNoteService, DynUserService},
    config::{Config, ConnectionPool},
    repository::{NoteRepository, UserRepository},
    service::{NoteService, UserService},
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub env: Config,
    pub note_service: DynNoteService,
    pub user_service: DynUserService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Config) -> Self {
        let note_repository = Arc::new(NoteRepository::new(pool.clone())) as DynNoteRepository;
        let note_service = Arc::new(NoteService::new(note_repository)) as DynNoteService;

        let user_repository = Arc::new(UserRepository::new(pool.clone()));
        let user_service = Arc::new(UserService::new(user_repository.clone()));

        ServiceRegister {
            env: config.clone(),
            note_service,
            user_service,
        }
    }
}
