use crate::app::domain::models::Assessment;
use crate::app::domain::repository::Repository;
use crate::infra::assesment_mapper::AssessmentEntity;
use crate::infra::database::DbClient;
use crate::schema::assessment;
use diesel::r2d2::ConnectionManager;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use std::env::var;
use std::io::Error;

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

impl Repository<Assessment, Error> for RiskDieselPg {
    fn save(&self, object: Assessment) -> Result<Assessment, Error> {
        let conn = &self
            .pool
            .as_ref()
            .unwrap()
            .try_get()
            .expect("Pool unreached for operation");

        let result = insert_into(assessment::table)
            .values(AssessmentEntity::map_to_insert(object))
            .get_result::<AssessmentEntity>(conn)
            .expect("Cannot insert Assesment");

        Ok(AssessmentEntity::map_from_db(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::domain::models::Risk;
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
    fn given_risk_diesel_pg_when_pool_none_call_new_expect_some_pool() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let risk_postgres = RiskDieselPg { pool: None }.new();

        assert!(
            risk_postgres.pool.is_some(),
            "given RiskDieselPg when pool is none and environment was configured expect some poll"
        );
    }

    #[test]
    fn given_risk_diesel_pg_when_pool_referenced_expect_some_pool() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let risk_postgres = RiskDieselPg {
            pool: Option::from(r2d2::Pool::builder().build(manager).unwrap()),
        };

        assert!(
            risk_postgres.pool.is_some(),
            "given RiskDieselPg when pool referenced expect some poll"
        );
    }

    #[test]
    #[should_panic(expected = "Failed to create pool")]
    fn given_risk_diesel_pg_postgres_when_environment_var_empty_pool_referenced_expect_panic() {
        set_var("DATABASE_URL", String::from(""));
        let risk_postgres = RiskDieselPg { pool: None }.new();
    }

    #[test]
    fn given_risk_diesel_pg_when_save_new_assessment_expect_ok() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let risk_postgres = RiskDieselPg { pool: None }.new();
        let assessment = Assessment {
            id: None,
            risk: Risk {
                order_number: 1,
                customer_id: None,
                customer_name: "Linus Torvalds".to_string(),
                customer_cpf: "00000000000".to_string(),
                card_number: "4444333322221111".to_string(),
                card_holder_name: "Linus Torvalds".to_string(),
                value: 45.20,
                creation_date_order: chrono::Local::now().naive_local()
                    + chrono::Duration::hours(24),
            },
            status: "APPROVED".to_string(),
            motivation: None,
            create_at: None,
            update_at: None,
        };

        assert!(risk_postgres.save(assessment).is_ok());
    }
}
