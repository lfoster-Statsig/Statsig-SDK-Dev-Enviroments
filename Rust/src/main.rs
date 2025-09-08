use statsig_rust::{Statsig, StatsigOptions, StatsigUserBuilder};
use std::env;
use std::sync::Arc;

// Simple init

#[tokio::main]
async fn main()
{
    // Initialize the logger
    let statsig = init_statsig().await.unwrap();

    // Pull user information from your system
    let user_id: String = env::var("USER").unwrap_or_else(|_| "default_user".to_string());
    let user: statsig_rust::StatsigUser = StatsigUserBuilder::new_with_user_id(&user_id).build();

    if statsig.check_gate(&user, "new_feature_gate") {
        // Gate is on, enable new feature
        println!("New feature is enabled for user: {}", user_id);
    } else {
        // Gate is off, use old feature
        println!("New feature is NOT enabled for user: {}", user_id);
    }
    statsig.log_event(&user, "app_started", None, None);

    statsig.shutdown().await.unwrap();
}

async fn init_statsig() -> Result<Statsig, Box<dyn std::error::Error>>
{
    let mut options = StatsigOptions::default();
    options.environment = Some("development".to_string());

    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let server_key = env::var("SERVER_KEY")?;
    let statsig = Statsig::new(&server_key, Some(Arc::new(options)));

    statsig.initialize().await.unwrap();

    return Ok(statsig);
}