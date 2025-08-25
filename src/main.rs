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
    visitor : Arc<Mutex<Vec<String>>>
}

#[derive(Deserialize)]
struct NewVisitor{
    name : String   // the way json is coming in to the server
}

#[derive(Serialize)]
struct VisitorList{
    visitors : Vec<String>   // the way json is coming in to the server
}


#[tokio::main]
async fn main() {
    let state = AppState{
        visitor : Arc::new(Mutex::new(Vec::new()))   
    };
    let app = Router::new()
        .route("/api/visitors",get(get_visitors).post(add_visitors))
        .nest_service("/", ServeDir::new("folder"))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn get_visitors(State(state): State<AppState>) -> Json<VisitorList>{
    let visitors = state.visitor.lock().unwrap().clone();
    Json(VisitorList{visitors})
}
async fn add_visitors(
    State(state) : State<AppState>,
    Json(payload) : Json<NewVisitor>,
) -> &'static str {
    let mut visitors = state.visitor.lock().unwrap();
    visitors.push(payload.name);
    "Visitor added"
}
