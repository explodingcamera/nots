use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    println!("listening on 0.0.0.0:3333");

    axum::Server::bind(&"0.0.0.0:3333".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hi"
}
