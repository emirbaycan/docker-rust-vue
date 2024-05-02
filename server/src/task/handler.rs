use serde_json::json;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::task::{ model::TaskModel, schema::CreateTaskSchema };
use crate::AppState;

use super::schema::TaskFilters;

use chrono::DateTime;

pub async fn task_list_handler(
    opts: Option<Query<TaskFilters>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let agenda_id = opts.agenda_id.unwrap_or("".to_string());

    let mut sets = "WHERE ".to_string();
    let mut params: Vec<String> = Vec::new(); 
    let mut parameter_count = 0;

    if !agenda_id.is_empty(){
        sets.push_str("a.agenda_id = $");
        parameter_count += 1;
        sets.push_str(&parameter_count.to_string());
        params.push(agenda_id);
    }else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
 
    let query = if parameter_count > 0 {
        format!("SELECT * FROM tasks a {} 
        INNER JOIN task_groups b ON a.group_id = b.group_id
        INNER JOIN task_updates c ON a.task_id = c.task_id        
        INNER JOIN task_visors d ON a.task_id = d.task_id
        INNER JOIN task_supervisors e ON a.task_id = e.task_id
        INNER JOIN task_agendas f ON a.agenda_id = b.agenda_id
        ORDER by a.created_at", sets)
    } else {
        "SELECT * FROM tasks a
        INNER JOIN task_groups b ON a.group_id = b.group_id
        INNER JOIN task_updates c ON a.task_id = c.task_id        
        INNER JOIN task_visors d ON a.task_id = d.task_id
        INNER JOIN task_supervisors e ON a.task_id = e.task_id
        INNER JOIN task_agendas f ON a.agenda_id = b.agenda_id
        ORDER by a.created_at".to_string()
    };

    let mut query  = sqlx::query_as::<_, TaskModel>(&query);
    for param in params {
        query  = query.bind(param);
    }

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
        "items": items
    });
    Ok(Json(json_response))
}
 
pub async fn create_task_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = sqlx
        ::query_as!(
            TaskModel,
            "INSERT INTO tasks (group_id,name,date,expiration_date,status,priority) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            body.group_id,
            body.name.to_string(),
            DateTime::from_timestamp(body.date, 0),
            DateTime::from_timestamp(body.expiration_date, 0),
            body.status,
            body.priority,
        )
        .fetch_one(&data.db).await;

    match query_result {
        Ok(item) => {
            let item_response =
                json!({"status": "success","data": json!({
                "item": item
            })});

            return Ok((StatusCode::CREATED, Json(item_response)));
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                let error_response =
                    serde_json::json!({
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

pub async fn delete_task_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx
        ::query!("DELETE FROM tasks WHERE task_id = $1", id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
