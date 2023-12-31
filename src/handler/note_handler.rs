use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use serde_json::json;

use crate::{
    schema::{CreateNoteSchema, UpdateNoteSchema},
    service_register::ServiceRegister,
};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/notes")]
async fn get_notes(state: web::Data<ServiceRegister>) -> impl Responder {
    let query_result = state.note_service.get_notes().await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all note items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/notes")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let query_result = state
        .note_service
        .create_note(&body.title.to_string(), &body.content.to_string())
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/notes/{id}")]
async fn get_note_handler(
    path: web::Path<uuid::Uuid>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let note_id = path.into_inner();

    let query_result = state.note_service.get_note_id(note_id).await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found", note_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/notes/{id}")]
async fn edit_note_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateNoteSchema>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let note_id = path.into_inner();

    let query_result = state.note_service.get_note_id(note_id).await;

    if query_result.is_err() {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let _note = query_result.unwrap();

    let query_result = state
        .note_service
        .update_note(
            note_id,
            &body.title.to_owned().to_string(),
            &body.content.to_owned().to_string(),
        )
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/notes/{id}")]
async fn delete_note_handler(
    path: web::Path<uuid::Uuid>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let note_id = path.into_inner();

    if let Err(err) = state.note_service.delete_note(note_id).await {
        log::error!("Failed to delete note: {:?}", err);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::NoContent().finish()
}
