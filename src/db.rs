use std::time::Duration;

use deadpool_postgres::{
    Config, CreatePoolError, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime,
};

pub fn new_pool() -> Result<Pool, CreatePoolError> {
    let mut pool_cfg = PoolConfig::new(4);
    pool_cfg.timeouts.wait = Some(Duration::from_millis(200));
    pool_cfg.timeouts.create = Some(Duration::from_millis(2000));
    pool_cfg.timeouts.recycle = Some(Duration::from_millis(200));
    let mut cfg = Config::new();
    cfg.host = Some(String::from("postgres"));
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.dbname = Some(String::from("postgres"));
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.pool = Some(pool_cfg);
    cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
}
