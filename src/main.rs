use axum::{
    routing::{get,
    post},
    Router,
};
use std::sync::{Arc,Mutex};
use tower_http::services::ServeDir;
use serde::{Deserialize,Serialize};
use axum::extract::{State, Json};

#[derive(Clone)]
struct AppState{
    comments : Arc<Mutex<Vec<Comment>>>
}
#[derive(Deserialize)]
struct NewComment{
    name : String,
    comment : String
}

#[derive(Serialize,Clone)]
struct Comment{
    name : String,
    comment : String
}

#[tokio::main]
async fn main(){
    let state = AppState{
        comments : Arc::new(Mutex::new(Vec::new()))
    };
    let app = Router::new()
        .route("/api/visitors",get(get_comments).post(add_comments))
        .nest_service("/", ServeDir::new("folder"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_comments(State(state): State<AppState>) -> Json<Vec<Comment>>{
    let comment = state.comments.lock().unwrap().clone();
    Json(comment)
}
async fn add_comments(State(state) : State<AppState>, Json(payload): Json<NewComment>){
    let mut comment = state.comments.lock().unwrap();
    comment.push(Comment{
       name: payload.name,
    comment: payload.comment});
}
