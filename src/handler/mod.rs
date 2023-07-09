use actix_web::web;

use self::auth_handler::{
    get_me_handler, login_user_handler, logout_handler, register_user_handler,
};
use self::note_handler::{
    create_note_handler, delete_note_handler, edit_note_handler, get_note_handler, get_notes,
    health_checker_handler,
};

mod auth_handler;
mod note_handler;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(get_notes)
        .service(create_note_handler)
        .service(get_note_handler)
        .service(edit_note_handler)
        .service(delete_note_handler)
        .service(login_user_handler)
        .service(register_user_handler)
        .service(get_me_handler)
        .service(logout_handler);

    conf.service(scope);
}
