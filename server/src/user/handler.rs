use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use tower_sessions::Session;

use crate::general::schema::{FilterOptions, Table};
use crate::user::{
    model::UserModel,
    schema::{CreateUserSchema, UpdateUserSchema},
};

use crate::AppState;

pub async fn user_list_handler(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let search = opts.search.unwrap_or("".to_string());
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let mut sets = "WHERE ".to_string();
    let mut params: Vec<String> = Vec::new(); 
    let mut parameter_count = 0;

    if !search.is_empty(){
        sets.push_str("name LIKE CONCAT('%',$");
        parameter_count += 1;
        sets.push_str(&parameter_count.to_string());
        sets.push_str(",'%') ");
        params.push(search);
    }

    sets.push_str("role > 1");

    let query = format!("SELECT count(id) as count FROM users {}", sets);

    let mut query  = sqlx::query_as::<_, Table>(&query);

    for param in params.clone() {
        query  = query .bind(param);
    }

    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "error": query_result.unwrap_err().to_string(),
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let item = query_result.unwrap();
    let count = item.count;

    let query = format!("SELECT * FROM users {} ORDER by created_at LIMIT ${} OFFSET ${}", sets, parameter_count+1, parameter_count+2);

    let mut query  = sqlx::query_as::<_, UserModel>(&query);
    for param in params {
        query  = query.bind(param);
    }

    query = query.bind(limit as i32).bind(offset as i32);

    let query_result = query.fetch_all(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "error": query_result.unwrap_err().to_string(),
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "count": count,
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx
        ::query_as!(
            UserModel,
            "INSERT INTO users (username,password,email,fullname,role,avatar,notes,active) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            body.username.to_string(),
            body.password.to_string(),
            body.email.to_string(),
            body.fullname.to_string(),
            body.role,
            body.avatar.to_string(),
            body.notes.to_string(),
            body.active
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response = json!({"status": "success","data": json!({
                "item": item
            })});

            return Ok((StatusCode::CREATED, Json(item_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_user_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Item with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn new_user_list_handler(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(Table, 
        "SELECT count(id) as count FROM users WHERE role = 1")
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let item = query_result.unwrap();

    let count = item.count;

    let query_result = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE role = 1 ORDER by created_at LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "count": count,
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn edit_user_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            UserModel,
            "UPDATE users SET username = $1, password = $2, email = $3, fullname = $4, role = $5, avatar = $6, notes = $7, active = $8, updated_at = $9 WHERE id = $10 RETURNING *",
            body.username.to_owned().unwrap_or(item.username),
            body.password.to_owned().unwrap_or(item.password),
            body.email.to_owned().unwrap_or(item.email),
            body.fullname.to_owned().unwrap_or(item.fullname),
            body.role.to_owned().unwrap_or(item.role),
            body.avatar.to_owned().unwrap_or(item.avatar),
            body.notes.to_owned().unwrap_or(item.notes),
            body.active.to_owned().unwrap_or(item.active),
            now,
            id
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(item_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_user_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_user_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();
    let mut password = item.password;

    if body.password.is_some() && body.old_password.is_some(){
        if verify(&body.old_password.unwrap(),&password).unwrap(){
            password = hash(body.password.unwrap(),DEFAULT_COST).unwrap();
        }else{
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Passwords didn't match"
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }

    let query_result = sqlx
        ::query_as!(
            UserModel,
            "UPDATE users SET username = $1, password = $2, email = $3, fullname = $4, role = $5, avatar = $6, notes = $7, active = $8, updated_at = $9 WHERE id = $10 RETURNING *",
            body.username.to_owned().unwrap_or(item.username),  
            &password,
            body.email.to_owned().unwrap_or(item.email),
            body.fullname.to_owned().unwrap_or(item.fullname),
            body.role.to_owned().unwrap_or(item.role),
            body.avatar.to_owned().unwrap_or(item.avatar),
            body.notes.to_owned().unwrap_or(item.notes),
            body.active.to_owned().unwrap_or(item.active),
            now,
            id 
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(_item) => {
            let item_response = serde_json::json!({"status": "success"});

            return Ok(Json(item_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}