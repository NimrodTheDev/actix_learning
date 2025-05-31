use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use super::state::*;

#[derive(Serialize, Deserialize, Debug)]
struct Response <T>{
    status: String,
    message: String,
    payload: Option<T>
}

pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let health_status = &data.health_status;
    let mut visitor_count = data.visitor_count.lock().unwrap();
    *visitor_count += 1;
    let response = format!("Health: {}, Visitors: {}", health_status, visitor_count);
    let payload = Response::<String>{
        status: "success".to_string(),
        message: response,
        payload: None
    };
    HttpResponse::Ok().json(payload)
}
pub async fn get_resturant(data: web::Data<AppState>) -> impl Responder {
    let webdata = data.restaurant.lock().unwrap();
    let payload = Restaurant{
        id: webdata.id,
        name: webdata.name.clone(),
        cuisine: webdata.cuisine.clone(),
        address: webdata.address.clone(),
        rating: webdata.rating,
        is_open: webdata.is_open
    };

    HttpResponse::Ok().json(Response{
        status: "success".to_string(),
        message: "Restuarant retrieved successfully".to_string(),
        payload: Some(payload)
    })
}

pub async fn register_restaurant(restaurant: web::Json<Restaurant>,data: web::Data<AppState>) -> impl Responder {
    println!("Called");
    let mut resturant_state = data.restaurant.lock().unwrap();
    *resturant_state = restaurant.clone().into();
    let payload: Restaurant = restaurant.into();
    HttpResponse::Ok().json(Response{
        status: "success".to_string(),
        message: "Restaurant registered successfully".to_string(),
        payload: Some(payload)
    })
}

pub async fn add_mennu_items(item: web::Json<Vec<MenuItem>>, data: web::Data<AppState>)-> impl Responder {
    let mut menu = data.menu_items.lock().unwrap();
    menu.extend(item.into_inner());
    HttpResponse::Ok().json(Response {
        status: "success".to_string(),
        message: "Menu items added successfully".to_string(),
        payload: Some(menu.clone())
    })
}


pub async fn get_menu_items(data: web::Data<AppState>)-> impl Responder {
    let resource = data.menu_items.lock().unwrap();
    HttpResponse::Ok().json(Response{
        status: "success".to_string(),
        message: "Menu retrieved successfully".to_string(),
        payload: Some(resource.clone())
    })
}

pub async fn get_menu_category_items(data: web::Data<AppState>, params: web::Path<String>)-> impl Responder {
    let resource = data.menu_items.lock().unwrap();
    let category = params;
    let resource = resource.clone().into_iter().filter(|ele| ele.category == *category).collect::<Vec<MenuItem>>();
    if resource.len() == 0 {
        HttpResponse::Ok().json(Response{
            status: "success".to_string(),
            message: "No menu items".to_string(),
            payload: Some(resource.clone())
        })
    }else{
        HttpResponse::Ok().json(Response{
            status: "success".to_string(),
            message: "Menu retrieved successfully".to_string(),
            payload: Some(resource.clone())
        })
    }
}

pub async fn add_order(payload: web::Json<Order>, state: web::Data<AppState>)-> impl Responder{
    let mut order = state.orders.lock().unwrap();
    order.push(payload.into());
    HttpResponse::Ok().json(Response::<String>{
        status: "success".to_string(),
        message: "Order added successfully".to_string(),
        payload: None
    })
} 

pub async fn all_orders(state: web::Data<AppState>)-> impl Responder{
    let order = state.orders.lock().unwrap();
    HttpResponse::Ok().json(Response::<Vec<Order>>{
        status: "success".to_string(),
        message: "Order added successfully".to_string(),
        payload: Some(order.clone())
    })
}

pub async fn user_order(state: web::Data<AppState>, param: web::Path<String>)-> impl Responder{
    let order = state.orders.lock().unwrap();
    let user_id = param.clone();
    let order: Vec<Order> = order.clone().into_iter().filter(|ele| ele.user_id == user_id).collect();
    HttpResponse::Ok().json(Response::<Vec<Order>>{
        status: "success".to_string(),
        message: "Order added successfully".to_string(),
        payload: Some(order.clone())
    })
}