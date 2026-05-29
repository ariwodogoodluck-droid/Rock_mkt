use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Main routing path mapping root context
    let app = Router::new().route("/", get(serve_market_app));

    // FIX: Added the proper local IP array variables inside the brackets
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 =================================================");
    println!("   ROCK~ MKT SERVER ENGINE RUNNING PERFECTLY!");
    println!("   Open your link window: http://127.0.0.1:3000");
    println!("   =================================================\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_market_app(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    // Check which navigation link view state is active
    let active_tab = params.get("tab").cloned().unwrap_or_else(|| "feed".to_string());
    
    // Fetch basic structural interface files
    let base_html = include_str!("index.html");
    let mut modified_html = base_html.to_string();
    
    // Injection processing block setting layout active states cleanly
    if active_tab == "feed" {
        modified_html = modified_html.replace("{feed_active}", "active").replace("{feed_display}", "block");
        modified_html = modified_html.replace("{friends_active}", "").replace("{friends_display}", "none");
        modified_html = modified_html.replace("{msg_active}", "").replace("{msg_display}", "none");
        modified_html = modified_html.replace("{notif_active}", "").replace("{notif_display}", "none");
    } else if active_tab == "friends" {
        modified_html = modified_html.replace("{feed_active}", "").replace("{feed_display}", "none");
        modified_html = modified_html.replace("{friends_active}", "active").replace("{friends_display}", "block");
        modified_html = modified_html.replace("{msg_active}", "").replace("{msg_display}", "none");
        modified_html = modified_html.replace("{notif_active}", "").replace("{notif_display}", "none");
    } else if active_tab == "messages" {
        modified_html = modified_html.replace("{feed_active}", "").replace("{feed_display}", "none");
        modified_html = modified_html.replace("{friends_active}", "").replace("{friends_display}", "none");
        modified_html = modified_html.replace("{msg_active}", "active").replace("{msg_display}", "block");
        modified_html = modified_html.replace("{notif_active}", "").replace("{notif_display}", "none");
    } else if active_tab == "notifications" {
        modified_html = modified_html.replace("{feed_active}", "").replace("{feed_display}", "none");
        modified_html = modified_html.replace("{friends_active}", "").replace("{friends_display}", "none");
        modified_html = modified_html.replace("{msg_active}", "").replace("{msg_display}", "none");
        modified_html = modified_html.replace("{notif_active}", "active").replace("{notif_display}", "block");
    }

    Html(modified_html)
}

