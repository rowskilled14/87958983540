use std::{collections::HashMap, io, sync::Arc};
use axum::{extract::{Path, State}, routing::{get, post}, Router};


struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

#[tokio::main]
#[axum::debug_handler]
async fn main() {
    let mut movie_hash: std::collections::HashMap<String, Movie> = std::collections::HashMap::new();

    let movie_arc = std::sync::Arc::new(movie_hash);



    println!("Hello, world!");
    let app = Router::new()
        .route("/movie/{id}", get(path)).with_state(movie_arc);
        // .route("/movie", post(post_handler()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn post_handler() -> Movie {
    todo!()
}

async fn path(Path(id): Path<String>, State(state): State<Arc<HashMap<String, Movie>>>)  {
    let mov = state.get(&id);
}



/* 

use std::io;

 

fn main() {

    // Create Axum server with the following endpoints:

    // 1. GET /movie/{id} - This should return back a movie given the id

    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent

    // via a JSON payload. 

    

    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.

    

    struct Movie {

        id: String,

        name: String,

        year: u16,

        was_good: bool

    }

}
    */
