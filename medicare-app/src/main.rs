use axum::routing::get;
use axum::{Extension, Router};
use common::Database;
use std::sync::Arc;
use std::sync::RwLock;

type DatabaseT = Arc<RwLock<Database>>;

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        println!("Exited: error {:?}", e);
    }
}

async fn try_main() -> anyhow::Result<()> {
    //init Database Lock
    let database = Arc::new(RwLock::new(Database::new("127.0.0.1", 5432)?));
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/home", get(home_handler))
        .layer(Extension(database));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn home_handler(database_lock: Extension<DatabaseT>) -> String {
    match database_lock.read() {
        Ok(database) => {
            let mut output = String::from("Home Handler");
            output.push_str(format!("\nDatabase IP : {} ", database.ip).as_str());
            output.push_str(format!("\nDatabase IP : {} ", database.port).as_str());
            output
        }

        Err(err) => {
            eprintln!("Failed to acquire read lock on database: {}", err);
            String::from("Error acquiring database lock")
        }
    }
}
