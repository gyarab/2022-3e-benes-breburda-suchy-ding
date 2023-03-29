use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct WebState {
    pub pool: Box<PgPool>,
}

pub trait StateWithDb {
    fn db(&self) -> &PgPool;
}

impl StateWithDb for WebState {
    fn db(&self) -> &PgPool {
        &self.pool
    }
}
