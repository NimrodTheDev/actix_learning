use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

use state::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_status: "Delivery Server is running successfully".to_string(),
        visitor_count: Mutex::new(0),
        users: Mutex::new(vec![]),
        restaurant: Mutex::new(Restaurant {
            id: 1,
            name: "Martin Resturant".to_string(),
            cuisine: "Igbo food".to_string(),
            address: "No 3. Don Wale Street, Onitsha, Anambra State".to_string(),
            rating: 4.5,
            is_open: true,
        }),
        menu_items: Mutex::new(vec![]),
        orders: Mutex::new(vec![]),
    });
    let app = move|| App::new().app_data(shared_data.clone()).configure(&routes::all_routes).configure(&routes::restaurant_routes);
    HttpServer::new(app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

