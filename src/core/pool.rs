use std::time::Duration;

use bb8_redis::{bb8, RedisConnectionManager};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

pub type PGPool = PgPool;
pub type RedisPool = bb8::Pool<RedisConnectionManager>;


// mysql链接池
pub async fn create_pg_pool() -> PGPool {
    let conn_str: String = dotenvy::var("PG_URL").expect("env not found pg URL");

    let opt: PgConnectOptions = conn_str
        .parse()
        .expect("could not parse db connection string");

    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        //.connect_timeout(Duration::from_secs(conn_timeout))
        .idle_timeout(Duration::from_secs(30))
        .test_before_acquire(true)
        .connect_with(opt)
        .await
        .expect("could not create db_pool due to")
}

// redis 连接池
pub async fn create_redis_pool() -> RedisPool {
    let conn_str = dotenvy::var("REDIS_URL").expect("ENV not found redis url");

    let mgr = RedisConnectionManager::new(conn_str)
        .expect("could not create RedisConnectionManager due to");
    bb8::Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(10))
        .idle_timeout(Some(Duration::from_secs(60)))
        .build(mgr)
        .await
        .expect("could not create RedisPool due to")
}