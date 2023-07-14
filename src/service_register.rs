use std::sync::Arc;

use crate::{
    config::{Config, ConnectionPool},
    repository::{NoteRepository, UserRepository},
    service::{NoteService, UserService},
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub env: Config,
    pub note_service: Arc<NoteService>,
    pub user_service: Arc<UserService>,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Config) -> Self {
        let note_repository = NoteRepository {
            db_pool: pool.clone(),
        };

        let note_repository_shared: Arc<NoteRepository> = Arc::new(note_repository);
        let note_service = NoteService::new(note_repository_shared.clone());

        let user_repository = UserRepository {
            db_pool: pool.clone(),
        };
        let user_repository_shared = Arc::new(user_repository);
        let user_service = UserService::new(user_repository_shared.clone());

        ServiceRegister {
            env: config.clone(),
            note_service: Arc::new(note_service.clone()),
            user_service: Arc::new(user_service.clone()),
        }
    }
}
