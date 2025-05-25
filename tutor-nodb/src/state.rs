use std::sync::Mutex;
use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub struct AppState{
    pub health_status: String,
    pub visitor_count: Mutex<u32>,
    pub users: Mutex<Vec<User>>,
    pub restaurant: Mutex<Restaurant>,
    pub menu_items: Mutex<Vec<MenuItem>>,
    pub orders: Mutex<Vec<Order>>,
}

// Example request/response structures
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub telegram_id: String,
    pub username: String,
    pub phone_number: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
}
impl From<web::Json<User>> for User {
    fn from(user: web::Json<User>) -> Self {
        User {
            telegram_id: user.telegram_id.clone(),
            username: user.username.clone(),
            phone_number: user.phone_number.clone(),
            address: user.address.clone(),
            created_at: user.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Restaurant {
    pub id: u32,
    pub name: String,
    pub cuisine: String,
    pub address: String,
    pub rating: f32,
    pub is_open: bool,
}
impl From<web::Json<Restaurant>> for Restaurant {
    fn from(restaurant: web::Json<Restaurant>) -> Self {
        Restaurant {
            id: restaurant.id,
            name: restaurant.name.clone(),
            cuisine: restaurant.cuisine.clone(),
            address: restaurant.address.clone(),
            rating: restaurant.rating,
            is_open: restaurant.is_open,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MenuItem {
    pub id: u32,
    pub restaurant_id: u32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub category: String,
    is_available: bool,
}
impl From<web::Json<MenuItem>> for MenuItem {
    fn from(menu_item: web::Json<MenuItem>) -> Self{
        MenuItem { id: menu_item.id, restaurant_id: menu_item.restaurant_id, name: menu_item.name.clone(), description: menu_item.description.clone(), price: menu_item.price, category: menu_item.category.clone(), is_available: menu_item.is_available }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub user_id: String,
    pub restaurant_id: u32,
    pub items: Vec<MenuItem>,
    pub total_amount: f32,
    pub status: OrderStatus,
    created_at: DateTime<Utc>,
}
impl From<web::Json<Order>> for Order {
    fn from(order: web::Json<Order>) -> Self {
        Order { id: order.id, user_id: order.user_id.clone(), restaurant_id: order.restaurant_id, items: order.items.clone(), total_amount: order.total_amount, status: order.status.clone(), created_at: order.created_at }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
}