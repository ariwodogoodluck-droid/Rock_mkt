mod models;

use axum::{
    routing::post,
    Json, Router,
};
use models::{ProductPost, UserAccount};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

async fn verify_gate_entry(Json(payload): Json<UserAccount>) -> Json<String> {
    println!("[SECURITY GATE] Checking clearance for profile: {}", payload.account_username);
    if payload.milestones.total_joined_members >= 3000
        && payload.milestones.current_completed_deals >= 10
        && payload.milestones.permanent_public_reviews >= 10
    {
        println!("[MILESTONES] Verification Badge Purchase is UNLOCKED!");
    }
    Json("Gate Authentication Approved.".to_string())
}

async fn publish_market_post(Json(post): Json<ProductPost>) -> Json<String> {
    if post.product_name.trim().is_empty() {
        return Json("Refused: Item name cannot be empty.".to_string());
    }
    if post.price <= 0.0 {
        return Json("Refused: Price must be greater than zero.".to_string());
    }
    println!("[MARKET REGISTER] New Public Posting Published Successfully!");
    Json("Publication Approved.".to_string())
}

#[tokio::main]
async fn main() {
    println!("--------------------------------------------------");
    println!("    ROCK MKT - COMPILED BUSINESS RUNTIME ENGINE   ");
    println!("--------------------------------------------------");

    let app = Router::new()
        .route("/api/gate/verify", post(verify_gate_entry))
        .route("/api/market/post", post(publish_market_post))
        .fallback_service(ServeDir::new("../frontend"));

    let address = SocketAddr::from(([0, 0, 0, 0], 10000));
    println!("[SERVER COMPLIANCE] Listening safely on: http://{}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
