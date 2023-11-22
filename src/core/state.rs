use super::pool;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: pool::PGPool,
    pub redis_pool: pool::RedisPool
}

pub async fn init_state() -> AppState {
    let pg_pool = pool::create_pg_pool().await;
    let redis_pool = pool::create_redis_pool().await;
    AppState {
        pg_pool,
        redis_pool
    }
}


pub struct ReqContext {
    pub user_id: String
}