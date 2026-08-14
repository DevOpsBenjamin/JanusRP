use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use janus_db::PgPool;
use janus_llm::LlmClient;

#[derive(Clone)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub llm: Arc<dyn LlmClient>,
    pub campaign_locks: Arc<Mutex<HashMap<Uuid, Arc<Mutex<()>>>>>,
}

impl AppState {
    pub fn new(db: Option<PgPool>, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            db,
            llm,
            campaign_locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_campaign_lock(&self, campaign_id: Uuid) -> Arc<Mutex<()>> {
        let mut locks = self.campaign_locks.lock().await;
        locks
            .entry(campaign_id)
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }
}
