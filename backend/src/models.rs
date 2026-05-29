use serde::{Deserialize, Serialize};

// Unified dynamic currency variant schema engine blocks
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CurrencyType {
    NGN, // Naira
    USD, // Dollar
    GHS, // Cedis
}

// Concrete data model blueprint tracking every single public market listing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductPost {
    pub post_id: u64,
    pub business_owner: String,
    pub product_name: String,
    pub product_description: String,
    pub price: f64,
    pub currency: CurrencyType,
    // Anti-fraud telemetry: Hiding manual inputs forces hard device permissions tracking
    pub verification_latitude: f64,
    pub verification_longitude: f64,
    pub sponsored_boost_active: bool,
}

// Strict analytical ledger mapping real-time requirements for trust verification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BadgeMilestones {
    pub current_completed_deals: u32,  // Locked target metric: 10 required
    pub permanent_public_reviews: u32,  // Locked target metric: 10 required
    pub total_joined_members: u32,      // Locked target metric: 3,000 required
    pub badge_unlocked_and_paid: bool,
}

// Single profile account engine layout mapping business nodes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAccount {
    pub mobile_phone_key: String,
    pub store_display_name: String,
    pub account_username: String,
    pub location_city_label: String,
    pub facebook_link_url: Option<String>,
    pub instagram_link_url: Option<String>,
    pub milestones: BadgeMilestones,
}

