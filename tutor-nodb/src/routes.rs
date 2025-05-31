use super::handlers::*;

use actix_web::web;

pub fn all_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn restaurant_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/register")
        .route("/", web::post().to(register_restaurant))
        .route("/get", web::get().to(get_resturant))
    );
    cfg.service(
        web::scope("/menu")
        .route("/", web::post().to(add_mennu_items))
        .route("/get", web::get().to(get_menu_items))
        .route("/get/{category}", web::get().to(get_menu_category_items))
    );

    cfg.service(
        web::scope("/order")
        .route("/", web::post().to(add_order))
        .route("/get", web::get().to(all_orders))
        .route("/get/{user_id}", web::get().to(user_order))
    );

    // cfg.route("/order", web::post().to(place_order));
}
