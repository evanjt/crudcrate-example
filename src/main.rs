mod common;
mod routes;

use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let db = Database::connect("sqlite::memory:").await.unwrap();

    // Run migrations
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let local_address = "0.0.0.0:3000";

    let addr: std::net::SocketAddr = local_address.parse().unwrap();
    println!("Visit http://{addr}/docs to see the Scalar OpenAPI docs and test the API.");
    println!("Or access the CRUD endpoints at http://{addr}/todo");

    let router = routes::build_router(&db);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        router.into_make_service(),
    )
    .await
    .unwrap();
}
