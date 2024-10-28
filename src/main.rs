use axum::routing::Router;
use std::net::SocketAddr;

mod fonki;
mod holychords;
mod psalms;
mod songs;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(psalms::routes())
        .merge(fonki::routes())
        .merge(holychords::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("The server is running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
