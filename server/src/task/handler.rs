use serde_json::json;
use tower_sessions::Session;
use std::sync::Arc;

use axum::{ extract::{ Path, Query, State }, http::StatusCode, response::IntoResponse, Json };

use crate::{auth::model::UserModel, task::{ model::{TaskAgendaModel, TaskModel}, schema::CreateTaskSchema }};
use crate::AppState;

use super::{model::{DisplayTaskSupervisorModel, DisplayTaskUpdateModel, DisplayTaskAgendaVisorModel, TaskGroupModel, TaskSupervisorModel, TaskUpdateModel, TaskAgendaVisorModel}, schema::{CreateTaskAgendaSchema, CreateTaskGroupSchema, CreateTaskSupervisorSchema, CreateTaskUpdateSchema, CreateTaskAgendaVisorSchema, TaskFilters, TaskUpdateFilters, UpdateTaskAgendaSchema, UpdateTaskGroupSchema, UpdateTaskSchema, UpdateTaskSupervisorSchema, UpdateTaskUpdateSchema, UpdateTaskAgendaVisorSchema}};

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
        INNER JOIN task_groups b ON b.group_id = a.group_id
        INNER JOIN task_agendas c ON c.agenda_id = b.agenda_id 
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

    let query = "SELECT a.*,b.email,b.fullname,b.avatar FROM task_agenda_visors a
    INNER JOIN users b on a.user_id = b.id
    WHERE a.agenda_id = $1";
    let mut query  = sqlx::query_as::<_, DisplayTaskAgendaVisorModel>(&query);

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

    let visors = query_result.unwrap();

    let mut supervisors:Vec<DisplayTaskSupervisorModel> = Vec::new();
    let mut updates:Vec<DisplayTaskUpdateModel> = Vec::new();

    for task in tasks.iter(){
           
        let query = "SELECT a.*,b.email,b.fullname,b.avatar FROM task_supervisors a
        INNER JOIN users b on a.user_id = b.id
        WHERE a.task_id = $1";
        let mut query  = sqlx::query_as::<_, DisplayTaskSupervisorModel>(&query);

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

        if !supervisor.is_empty() {
            for item in supervisor{
                supervisors.push(item);
            }
        }

        let query = "SELECT a.*,b.email,b.fullname,b.avatar FROM task_updates a
        INNER JOIN users b on a.user_id = b.id
        WHERE a.task_id = $1";
        let mut query  = sqlx::query_as::<_, DisplayTaskUpdateModel>(&query);

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

        if !update.is_empty() {
            for item in update{
                updates.push(item);
            }
        }
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

pub async fn tasks_list_handler(
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
    
    query = query.bind(user_id);
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

 
pub async fn create_task_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskGroupModel, 
        "SELECT a.* FROM task_groups a
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id 
        WHERE a.group_id = $1 and b.user_id = $2", body.group_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Unauthorized"
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query_result = sqlx
        ::query_as!(
            TaskModel,
            "INSERT INTO tasks (group_id,name,date,expiration_date,status,priority) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            body.group_id,
            body.name.to_string(),
            body.date,
            body.expiration_date,
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

pub async fn edit_task_handler(
    Path(id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskModel, 
        "SELECT a.* FROM tasks a
        INNER JOIN task_groups b ON a.group_id = b.group_id
        INNER JOIN task_agendas c ON b.agenda_id = c.agenda_id 
        WHERE a.task_id = $1 and c.user_id = $2", id, user_id)
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
            TaskModel,
            "UPDATE tasks SET group_id = $1, name = $2, date = $3, expiration_date = $4, status = $5, priority = $6, updated_at = $7 WHERE task_id = $8 RETURNING *",
            body.group_id.to_owned().unwrap_or(item.group_id),
            body.name.to_owned().unwrap_or(item.name),
            body.date.unwrap(),
            body.expiration_date.unwrap(),
            body.status.to_owned().unwrap_or(item.status),
            body.priority.to_owned().unwrap_or(item.priority),            
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

pub async fn delete_task_handler(
    Path(task_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskFilters, 
        "SELECT c.agenda_id FROM tasks a 
        INNER JOIN task_groups b ON a.group_id = b.group_id 
        INNER JOIN task_agendas c ON b.agenda_id = c.agenda_id 
        WHERE a.task_id = $1 and c.user_id = $2", 
        task_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", task_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let agenda = query_result.unwrap();

    let rows_affected = sqlx
        ::query!("DELETE FROM tasks WHERE task_id = $1", task_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", task_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let _result = sqlx
        ::query!("DELETE FROM task_agenda_visors WHERE agenda_id = $1", agenda.agenda_id)
        .execute(&data.db).await;
    let _result = sqlx
        ::query!("DELETE FROM task_supervisors WHERE task_id = $1", task_id)
        .execute(&data.db).await;
    let _result = sqlx
        ::query!("DELETE FROM task_updates WHERE task_id = $1", task_id)
        .execute(&data.db).await;

    let json_response = serde_json::json!({
        "status": "success"
    });
    Ok(Json(json_response))
}

pub async fn task_agenda_list_handler(
    session:Session,
    Path(agenda_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM task_agendas a
        WHERE a.user_id = $1 and a.agenda_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskAgendaModel>(&query);
    
    query = query.bind(user_id);
    query = query.bind(agenda_id);
 
    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "error": query_result.unwrap_err().to_string(),
            "status": "fail",
            "message": "Something bad happened while fetching all items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let item = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "item": item
    });
    Ok(Json(json_response))
}

pub async fn task_agendas_list_handler(
    session:Session,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM task_agendas a
        WHERE a.user_id = $1";
 
    let mut query  = sqlx::query_as::<_, TaskAgendaModel>(&query);
    
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_task_agenda_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskAgendaSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();
    
    let query_result = sqlx
        ::query_as!(
            TaskAgendaModel,
            "INSERT INTO task_agendas (title,description,user_id) VALUES ($1, $2, $3) RETURNING *",
            body.title.to_owned(),
            body.description.to_owned(),
            user_id
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

pub async fn edit_task_agenda_handler(
    Path(agenda_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskAgendaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskAgendaModel,
         "SELECT a.* FROM task_agendas a WHERE a.agenda_id = $1 and a.user_id = $2", agenda_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", agenda_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            TaskAgendaModel,
            "UPDATE task_agendas SET title = $1, description= $2, updated_at = $3 WHERE agenda_id = $4 RETURNING *",
            body.title.to_owned().unwrap_or(item.title),
            body.description.to_owned().unwrap_or(item.description),
            now,
            agenda_id
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

pub async fn delete_task_agenda_handler(
    session: Session,
    Path(agenda_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let rows_affected = sqlx
        ::query!("DELETE FROM task_agendas WHERE agenda_id = $1 and user_id=$2", agenda_id, user_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", agenda_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query = "SELECT a.* FROM tasks a
        LEFT JOIN task_groups b ON b.group_id = a.task_id
        LEFT JOIN task_agendas c ON c.agenda_id = b.agenda_id 
        WHERE c.agenda_id = $1";

    let mut query  = sqlx::query_as::<_, TaskModel>(&query);

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

    let items = query_result.unwrap();

    for item in items{
        let task_id = item.task_id;

        let _result = sqlx
            ::query!("DELETE FROM tasks WHERE task_id = $1", task_id)
            .execute(&data.db).await;
        let _result = sqlx
            ::query!("DELETE FROM task_updates WHERE task_id = $1", task_id)
            .execute(&data.db).await;
        let _result = sqlx
            ::query!("DELETE FROM task_supervisors WHERE task_id = $1", task_id)
            .execute(&data.db).await;
    }

    let _result = sqlx
        ::query!("DELETE FROM task_agenda_visors WHERE agenda_id = $1", agenda_id)
        .execute(&data.db).await;

    let _result = sqlx
        ::query!("DELETE FROM task_groups WHERE agenda_id = $1", agenda_id)
        .execute(&data.db).await;


    let json_response = serde_json::json!({
        "status": "success"
    });

    Ok(Json(json_response))
}

pub async fn task_groups_list_handler(
    opts: Option<Query<TaskFilters>>,
    session:Session,
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

    let query = "SELECT a.* FROM task_groups a
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id
        WHERE b.agenda_id = $1 and b.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskGroupModel>(&query);
    
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_task_group_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskGroupSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();
    let agenda_id = body.agenda_id;

    let query = "SELECT a.* FROM task_groups a
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id 
        WHERE b.agenda_id = $1 and b.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskModel>(&query);
    
    query = query.bind(agenda_id);
    query = query.bind(user_id);
 
    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Unauthorized",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    
    let query_result = sqlx
        ::query_as!(
            TaskGroupModel,
            "INSERT INTO task_groups (agenda_id,title) VALUES ($1, $2) RETURNING *",
            agenda_id,
            body.title
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

pub async fn edit_task_group_handler(
    Path(group_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskGroupSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskGroupModel, 
        "SELECT a.* FROM task_groups a 
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id 
        WHERE a.group_id = $1 and b.user_id = $2", 
        group_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", group_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            TaskGroupModel,
            "UPDATE task_groups SET title = $1, updated_at = $2 WHERE group_id = $3 RETURNING *",
            body.title.to_owned().unwrap_or(item.title),
            now,
            group_id
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

pub async fn delete_task_group_handler(
    session: Session,
    Path(group_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskFilters, 
        "SELECT b.agenda_id FROM task_groups a 
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id 
        WHERE a.group_id = $1 and b.user_id = $2", 
        group_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", group_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let agenda = query_result.unwrap();

    let rows_affected = sqlx
        ::query!("DELETE FROM task_groups WHERE group_id=$1", group_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", group_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query = "SELECT a.* FROM tasks a
        INNER JOIN task_groups b ON b.group_id = a.task_id
        WHERE b.group_id = $1";

    let mut query  = sqlx::query_as::<_, TaskModel>(&query);

    query = query.bind(group_id);

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

    for item in items{
        let task_id = item.task_id;
        let _result = sqlx
            ::query!("DELETE FROM task_updates WHERE task_id = $1", task_id)
            .execute(&data.db).await;

        let _result = sqlx
            ::query!("DELETE FROM task_supervisors WHERE task_id = $1", task_id)
            .execute(&data.db).await;
    }

    let _result = sqlx
        ::query!("DELETE FROM task_agenda_visors WHERE agenda_id = $1", agenda.agenda_id)
        .execute(&data.db).await;

    let _result = sqlx
        ::query!("DELETE FROM tasks WHERE group_id = $1", group_id)
        .execute(&data.db).await;

    let json_response = serde_json::json!({
        "status": "success"
    });
    Ok(Json(json_response))
}

pub async fn task_updates_list_handler(
    opts: Option<Query<TaskUpdateFilters>>,
    session:Session,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let Query(opts) = opts.unwrap_or_default();

    let task_id = opts.task_id.unwrap_or(0);

    if task_id==0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM task_updates a
        INNER JOIN tasks b ON a.task_id = b.task_id
        INNER JOIN taks_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id
        WHERE a.task_id = $1 and d.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskUpdateModel>(&query);
    
    query = query.bind(task_id);
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_task_update_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskUpdateSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();
    let task_id = body.task_id.to_owned();
    
    let query = "SELECT a.* FROM tasks a
        INNER JOIN task_groups b ON b.group_id = a.task_id
        INNER JOIN task_agendas c ON c.agenda_id = b.agenda_id 
        WHERE a.task_id = $1 and c.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskModel>(&query);
    
    query = query.bind(task_id);
    query = query.bind(user_id);
 
    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Unauthorized",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    
    let query_result = sqlx
        ::query_as!(
            TaskUpdateModel,
            "INSERT INTO task_updates (task_id,user_id,text) VALUES ($1, $2, $3) RETURNING *",
            task_id,
            user_id,
            body.text            
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

pub async fn edit_task_update_handler(
    Path(update_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskUpdateSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskUpdateModel, 
        "SELECT a.* FROM task_updates a 
        INNER JOIN tasks b ON a.task_id = b.task_id
        INNER JOIN task_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id
        WHERE a.update_id = $1 and d.user_id = $2", 
        update_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", update_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let item = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            TaskUpdateModel,
            "UPDATE task_updates SET text = $1, updated_at = $2 WHERE update_id = $3 RETURNING *",
            body.text.to_owned().unwrap_or(item.text),
            now,
            update_id
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

pub async fn delete_task_update_handler(
    session: Session,
    Path(update_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskUpdateModel, 
        "SELECT a.* FROM task_updates a 
        INNER JOIN tasks b ON a.task_id = b.task_id 
        INNER JOIN task_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id 
        WHERE a.update_id = $1 and d.user_id = $2", 
        update_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", update_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let rows_affected = sqlx
        ::query!("DELETE FROM task_updates WHERE update_id=$1", update_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", update_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success"
    });
    Ok(Json(json_response))
}

pub async fn task_agenda_visors_list_handler(
    opts: Option<Query<TaskUpdateFilters>>,
    session:Session,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let Query(opts) = opts.unwrap_or_default();

    let task_id = opts.task_id.unwrap_or(0);

    if task_id==0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM task_agenda_visors a
        INNER JOIN tasks b ON a.task_id = b.task_id
        INNER JOIN taks_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id
        WHERE a.task_id = $1 and d.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskAgendaVisorModel>(&query);
    
    query = query.bind(task_id);
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_task_agenda_visor_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskAgendaVisorSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();
    let task_id = body.task_id.to_owned();
    
    let query = "SELECT c.* FROM tasks a
        INNER JOIN task_groups b ON b.group_id = a.task_id
        INNER JOIN task_agendas c ON c.agenda_id = b.agenda_id 
        WHERE a.task_id = $1 and c.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskAgendaModel>(&query);
    
    query = query.bind(task_id);
    query = query.bind(user_id);
 
    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Unauthorized",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let agenda_id = query_result.unwrap().agenda_id;

    let query = "SELECT a.user_id FROM users a 
    WHERE a.email = $1"; 

    let mut query  = sqlx::query_as::<_, UserModel>(&query);

    query = query.bind(body.email);

    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "No such user",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let user = query_result.unwrap();
    
    let query_result = sqlx
        ::query_as!(
            TaskAgendaVisorModel,
            "INSERT INTO task_agenda_visors (agenda_id,user_id) VALUES ($1, $2) RETURNING *",
            agenda_id,
            user.id,         
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

pub async fn edit_task_agenda_visor_handler(
    Path(visor_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskAgendaVisorSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskAgendaVisorModel, 
        "SELECT a.* FROM task_agenda_visors a 
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id
        WHERE a.visor_id = $1 and b.user_id = $2", 
        visor_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() { 
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", visor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();

    let query = "SELECT a.user_id FROM users a 
    WHERE a.email = $1"; 

    let mut query  = sqlx::query_as::<_, UserModel>(&query);

    query = query.bind(body.email);

    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "No such user",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let user = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            TaskAgendaVisorModel,
            "UPDATE task_agenda_visors SET user_id = $1, updated_at = $2 WHERE visor_id = $3 RETURNING *",
            user.id,
            now,
            visor_id
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

pub async fn delete_task_agenda_visor_handler(
    session: Session,
    Path(visor_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskAgendaVisorModel, 
        "SELECT a.* FROM task_agenda_visors a 
        INNER JOIN task_agendas b ON a.agenda_id = b.agenda_id 
        WHERE a.visor_id = $1 and b.user_id = $2", 
        visor_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", visor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let rows_affected = sqlx
        ::query!("DELETE FROM task_agenda_visors WHERE visor_id=$1", visor_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", visor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success"
    });
    Ok(Json(json_response))
}

pub async fn task_supervisors_list_handler(
    opts: Option<Query<TaskUpdateFilters>>,
    session:Session,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let Query(opts) = opts.unwrap_or_default();

    let task_id = opts.task_id.unwrap_or(0);

    if task_id==0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Something went wrong")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query = "SELECT a.* FROM task_supervisors a
        INNER JOIN tasks b ON a.task_id = b.task_id
        INNER JOIN taks_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id
        WHERE a.task_id = $1 and d.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskSupervisorModel>(&query);
    
    query = query.bind(task_id);
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

    let items = query_result.unwrap();
       
    let json_response = serde_json::json!({
        "status": "success",
        "items": items
    });
    Ok(Json(json_response))
}

pub async fn create_task_supervisor_handler(
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTaskSupervisorSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();
    let task_id = body.task_id.to_owned();
    
    let query = "SELECT a.* FROM tasks a
        LEFT JOIN task_groups b ON b.group_id = a.task_id
        LEFT JOIN task_agendas c ON c.agenda_id = b.agenda_id 
        WHERE a.task_id = $1 and c.user_id = $2";
 
    let mut query  = sqlx::query_as::<_, TaskModel>(&query);
    
    query = query.bind(task_id);
    query = query.bind(user_id);
 
    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Unauthorized",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let query = "SELECT a.user_id FROM users a 
    WHERE a.email = $1"; 

    let mut query  = sqlx::query_as::<_, UserModel>(&query);

    query = query.bind(body.email);

    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "No such user",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let user = query_result.unwrap();
    
    let query_result = sqlx
        ::query_as!(
            TaskSupervisorModel,
            "INSERT INTO task_supervisors (task_id,user_id) VALUES ($1, $2) RETURNING *",
            task_id,
            user.id,         
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

pub async fn edit_task_supervisor_handler(
    Path(supervisor_id): Path<i32>,
    session: Session,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskSupervisorSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskSupervisorModel, 
        "SELECT a.* FROM task_supervisors a 
        INNER JOIN tasks b ON a.task_id = b.task_id
        INNER JOIN task_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id
        WHERE a.supervisor_id = $1 and d.user_id = $2", 
        supervisor_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() { 
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", supervisor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();

    let query = "SELECT a.user_id FROM users a 
    WHERE a.email = $1"; 

    let mut query  = sqlx::query_as::<_, UserModel>(&query);

    query = query.bind(body.email);

    let query_result = query.fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "No such user",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let user = query_result.unwrap();

    let query_result = sqlx
        ::query_as!(
            TaskSupervisorModel,
            "UPDATE task_supervisors SET user_id = $1, updated_at = $2 WHERE supervisor_id = $3 RETURNING *",
            user.id,
            now,
            supervisor_id
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

pub async fn delete_task_supervisor_handler(
    session: Session,
    Path(supervisor_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let user_id = session.get::<i32>("id").await.unwrap().unwrap();

    let query_result = sqlx::query_as!(TaskSupervisorModel, 
        "SELECT a.* FROM task_supervisors a 
        INNER JOIN tasks b ON a.task_id = b.task_id 
        INNER JOIN task_groups c ON b.group_id = c.group_id
        INNER JOIN task_agendas d ON c.agenda_id = d.agenda_id 
        WHERE a.supervisor_id = $1 and d.user_id = $2", 
        supervisor_id, user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", supervisor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let rows_affected = sqlx
        ::query!("DELETE FROM task_supervisors WHERE supervisor_id=$1", supervisor_id)
        .execute(&data.db).await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response =
            serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", supervisor_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success"
    });
    Ok(Json(json_response))
}