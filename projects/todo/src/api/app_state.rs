use sqlx::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
}
