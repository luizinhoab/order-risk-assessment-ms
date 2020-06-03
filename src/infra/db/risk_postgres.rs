use crate::app::domain::models::Assessment;
use crate::app::domain::repository::Repository;
use crate::errors::CustomError;
use crate::infra::db::entities::AssessmentEntity;
use crate::schema::assessment;
use diesel::r2d2::ConnectionManager;
use diesel::{insert_into, PgConnection, RunQueryDsl};

pub(crate) type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct RiskDieselPg {
    pub pool: Pool,
}

impl RiskDieselPg {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl Repository<Assessment, CustomError> for RiskDieselPg {
    fn save(&self, object: Assessment) -> Result<Assessment, CustomError> {
        let conn = &self.pool.try_get().expect("Pool unreached for operation");
        let entity = AssessmentEntity::map_to_insert(object);
        let result = insert_into(assessment::table)
            .values(&entity)
            .get_result::<AssessmentEntity>(conn)
            .expect("");

        Ok(AssessmentEntity::map_from_db(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::domain::models::{AssessmentStatus, Risk};
    use std::env::{set_var, var};
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
    fn given_risk_diesel_pg_when_new_expect_get_ok() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        let risk_postgres = RiskDieselPg::new(pool);

        assert!(
            risk_postgres.pool.get().is_ok(),
            "given RiskDieselPg when pool referenced expect some poll"
        );
    }

    #[test]
    fn given_risk_diesel_pg_when_save_new_assessment_expect_ok() {
        set_var("DATABASE_URL", String::from(DB.url()));
        let database_url =
            var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        let risk_postgres = RiskDieselPg::new(pool);

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
            status: AssessmentStatus::APPROVED,
            motivation: None,
            create_at: None,
            update_at: None,
        };

        assert!(risk_postgres.save(assessment).is_ok());
    }
}
