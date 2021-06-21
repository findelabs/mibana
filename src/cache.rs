use crate::db;
use chrono::offset::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

use db::{Posts, DB};

type Result = std::result::Result<Posts, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Default)]
pub struct Inner {
    pub time: i64,
    pub posts: Posts,
}

#[derive(Debug, Clone, Default)]
pub struct Cache {
    pub inner: Arc<RwLock<Inner>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner::default())),
        }
    }

    pub async fn posts(&mut self, db: DB) -> Result {
        // Read inner field
        let me = self.inner.read().await;

        // Get now
        let now = Utc::now().timestamp();

        // If timestamp of Cache is longer than ten seconds ago, try to reload cache
        if now - me.time > 30 {
            log::info!("Cache is more than 30 seconds old");

            // Drop time, and try to get write access
            drop(me);

            // Try and get write access to time lock
            let mut me = self.inner.write().await;
            log::info!("Acquired lock on inner");

            // Update self.posts with newly refreshed posts, and update time
            let refreshed = db.get_posts().await?;
            log::info!("Received refreshed posts");

            me.posts = refreshed;
            me.time = now;

            log::info!("Updated cache with refreshed posts");
            Ok(me.posts.clone())
        } else {
            Ok(me.posts.clone())
        }
    }
}
