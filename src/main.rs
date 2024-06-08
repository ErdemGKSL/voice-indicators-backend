use routes::get_router;

mod lib;
mod routes;

const PORT: &str = "15264";

#[tokio::main]
async fn main() {
    let app = get_router();


    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + PORT).await.unwrap();
    println!("Server started at http://localhost:{}", PORT);

    axum::serve(listener, app).await.unwrap();
}
