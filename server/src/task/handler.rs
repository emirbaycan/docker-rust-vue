use serde_json::json;
use tower_sessions::Session;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::task::{ model::TaskModel, schema::CreateTaskSchema };
use crate::AppState;

use super::{model::{TaskGroupModel, TaskSupervisorModel, TaskUpdateModel, TaskVisorModel}, schema::TaskFilters};

use chrono::DateTime;

pub async fn all_tasks_list_handler(
    session:Session,
    opts: Option<Query<TaskFilters>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let agenda_id = opts.agenda_id.unwrap_or(0);

    if agenda_id==0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM tasks a
        LEFT JOIN task_groups b ON b.group_id = a.task_id
        LEFT JOIN task_agendas c ON c.agenda_id = b.agenda_id 
        WHERE c.agenda_id = $1 and c.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskModel>(&query);
    
    query = query.bind(agenda_id);
    query = query.bind(user_id);
 
    let query_result = query.fetch_all(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "error": query_result.unwrap_err().to_string(),
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let tasks = query_result.unwrap();
    
    let query = "SELECT * FROM task_groups WHERE agenda_id = $1";
    let mut query  = sqlx::query_as::<_, TaskGroupModel>(&query);

    query = query.bind(agenda_id);
    
    let query_result = query.fetch_all(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "error": query_result.unwrap_err().to_string(),
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let groups = query_result.unwrap();

    let mut visors:Vec<Vec<TaskVisorModel>> = Vec::new();    
    let mut supervisors:Vec<Vec<TaskSupervisorModel>> = Vec::new();
    let mut updates:Vec<Vec<TaskUpdateModel>> = Vec::new();

    for task in tasks.iter(){
        
        let query = "SELECT * FROM task_visors WHERE task_id = $1";
        let mut query  = sqlx::query_as::<_, TaskVisorModel>(&query);

        query = query.bind(task.task_id);
        
        let query_result = query.fetch_all(&data.db).await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "error": query_result.unwrap_err().to_string(),
                "status": "fail",
                "message": "Something bad happened while fetching all items",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }

        let visor = query_result.unwrap();

        visors.push(visor);

        let query = "SELECT * FROM task_supervisors WHERE task_id = $1";
        let mut query  = sqlx::query_as::<_, TaskSupervisorModel>(&query);

        query = query.bind(task.task_id);
        
        let query_result = query.fetch_all(&data.db).await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "error": query_result.unwrap_err().to_string(),
                "status": "fail",
                "message": "Something bad happened while fetching all items",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }

        let supervisor = query_result.unwrap();

        supervisors.push(supervisor);

        let query = "SELECT * FROM task_updates WHERE task_id = $1";
        let mut query  = sqlx::query_as::<_, TaskUpdateModel>(&query);

        query = query.bind(task.task_id);
        
        let query_result = query.fetch_all(&data.db).await;

        if query_result.is_err() {
            let error_response = serde_json::json!({
                "error": query_result.unwrap_err().to_string(),
                "status": "fail",
                "message": "Something bad happened while fetching all items",
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }

        let update = query_result.unwrap();

        updates.push(update);
    }


    let json_response = serde_json::json!({
        "status": "success",
        "data": {
            "tasks": tasks,
            "groups": groups,
            "visors": visors,
            "supervisors": supervisors,
            "updates": updates
        }
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
