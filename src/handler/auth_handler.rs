use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{
    middleware::JwtMiddleware,
    models::UserModel,
    response::FilteredUser,
    schema::{LoginUserSchema, RegisterUserSchema, TokenClaims},
    AppState,
};

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exists = data
        .user_service
        .find_by_email_exists(&body.email.to_owned())
        .await;

    if let Err(_err) = exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let query_result = data
        .user_service
        .create_user(
            &body.firstname.to_owned(),
            &body.lastname.to_owned(),
            &body.email.to_owned(),
            &hashed_password,
        )
        .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_user_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = data
        .user_service
        .find_user_by_email(&body.email.to_owned())
        .await;

    let user = match query_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "Invalid email or password"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Error retrieving user: {}", err)
            }));
        }
    };

    let parsed_hash = PasswordHash::new(&user.password).unwrap();
    let is_valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|err| {
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Error verifying password: {}", err)
            }))
        });

    if let Err(err) = is_valid {
        return HttpResponse::Conflict().json(serde_json::json!({
            "status": "fail",
            "message": format!("Invalid email or password: {:?}", err)
        }));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
async fn logout_handler(_: JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user_result = data.user_service.find_by_id(*user_id).await;

    match user_result {
        Ok(Some(user)) => {
            let filtered_user = filter_user_record(&user);

            let json_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "user": filtered_user
                })
            });

            HttpResponse::Ok().json(json_response)
        }
        Ok(None) => {
            let json_response = serde_json::json!({
                "status": "fail",
                "message": "User not found"
            });

            HttpResponse::NotFound().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": format!("Error retrieving user: {}", err)
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

fn filter_user_record(user: &UserModel) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        firstname: user.firstname.to_owned(),
        lastname: user.lastname.to_owned(),

        role: user.role.to_owned(),
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}
