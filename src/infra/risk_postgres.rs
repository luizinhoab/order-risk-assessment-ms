use crate::infra::database::DbClient;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::env::var;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct RiskDieselPg {
    pool: Option<Pool>,
}

impl RiskDieselPg {
    pub fn new(&self) -> Self {
        Self {
            pool: Option::from(self.init_pool()),
        }
    }
}

impl DbClient<Pool> for RiskDieselPg {
    fn init_pool(&self) -> Pool {
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

        let manager = ConnectionManager::new(database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::set_var;
    use tempdb_cockroach::TempCockroach;
    embed_migrations!();

    lazy_static! {
        static ref DB: TempCockroach = setup_db();
    }

    fn setup_db() -> TempCockroach {
        let temp = TempCockroach::new().expect("Failed to create DB");
        let manager = ConnectionManager::<PgConnection>::new(temp.url());
        let conn = r2d2::Pool::builder()
            .build(manager)
            .unwrap()
            .try_get()
            .expect("Failed to connect db");

        embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
            .expect("unable to migrate");

        temp
    }

    #[test]
    fn given_tweetx_postgres_when_pool_none_call_new_expect_some_pool() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let tweetx_postgres = RiskDieselPg { pool: None }.new();

        assert!(tweetx_postgres.pool.is_some(), "given TweetxPostgres when pool is none and environment was configured expect some poll");
    }

    #[test]
    fn given_tweetx_postgres_when_pool_referenced_expect_some_pool() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let tweetx_postgres = RiskDieselPg {
            pool: Option::from(r2d2::Pool::builder().build(manager).unwrap()),
        };

        assert!(
            tweetx_postgres.pool.is_some(),
            "given TweetxPostgres when pool referenced expect some poll"
        );
    }

    #[test]
    #[should_panic(expected = "Failed to create pool")]
    fn given_tweetx_postgres_when_pool_referenced_expect_panic() {
        set_var("DATABASE_URL", String::from(""));
        let tweetx_postgres = RiskDieselPg { pool: None }.new();
    }
}
