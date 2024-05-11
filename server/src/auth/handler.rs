use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{auth::model::UserModel, task::model::TaskAgendaModel};
use crate::auth::schema::Login;
use crate::AppState;

use bcrypt::{hash, verify,DEFAULT_COST};
use tower_sessions::Session;

use super::schema::{CollectedUser, User};

pub async fn test_login_handler(
    session: Session,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    session.insert("logged_in",1).await.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
    });
    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn login_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Login>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email = body.email.unwrap();
    let password = body.password.unwrap();

    let query_result = sqlx::query_as!(
        User,
        "SELECT id,username,email,password,fullname,role,avatar,active FROM users WHERE email=$1",
        email
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User not exists",
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let user = query_result.unwrap();

    let password_hash = user.password.clone();
    let valid = verify(&password, &password_hash).unwrap();

    if !valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Passwords did not match",
        });
        return Err((StatusCode::NOT_ACCEPTABLE, Json(error_response)));
    }

    let query_result = sqlx::query_as!(
        TaskAgendaModel,
        "SELECT * FROM task_agendas WHERE user_id=$1",
        user.id
    )
    .fetch_all(&data.db)
    .await;

    let agendas = query_result.unwrap();

    let collected_user = CollectedUser {
        id: user.id,
        email: user.email.clone(),
        username: user.username.clone(),
        password: user.password,
        fullname: user.fullname.clone(),
        role: user.role,
        avatar: user.avatar.clone(),
        active: user.active,
        agendas, 
    };


    let json_response = serde_json::json!({
        "status": "success",
        "data": collected_user
    });

    session.insert("logged_in",1).await.unwrap();
    session.insert("id",user.id).await.unwrap();
    session.insert("email",user.email).await.unwrap();
    session.insert("username",user.username).await.unwrap();
    session.insert("fullname",user.fullname).await.unwrap();
    session.insert("role",user.role).await.unwrap();
    session.insert("avatar",user.avatar).await.unwrap();
    session.insert("active",user.active).await.unwrap();

    Ok((StatusCode::OK, Json(json_response)))
}


pub async fn register_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<Login>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email = body.email.unwrap();
    let password = body.password.unwrap();

    let hash = hash(&password,DEFAULT_COST).unwrap();

    let query_result = sqlx
    ::query_as!(
        UserModel,
        "INSERT INTO users (username,password,email,fullname,role,avatar,notes,active) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        "",
        hash,
        email,
        "Name Surname",
        1,
        "avatar.webp",
        "",
        1
    )
    .fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User already exists",
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": item
    });

    session.insert("logged_in",1).await.unwrap();
    session.insert("id",item.id).await.unwrap();
    session.insert("email",item.email).await.unwrap();
    session.insert("username",item.username).await.unwrap();
    session.insert("fullname",item.fullname).await.unwrap();
    session.insert("role",item.role).await.unwrap();
    session.insert("avatar",item.avatar).await.unwrap();
    session.insert("active",item.active).await.unwrap();

    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn logout_handler(
    session: Session,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    session.flush().await.unwrap();

    let json_response = serde_json::json!({
        "status": "success"
    });

    Ok((StatusCode::OK, Json(json_response)))
}