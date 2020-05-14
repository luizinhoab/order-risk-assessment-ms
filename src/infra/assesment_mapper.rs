use super::super::schema::assessment;
use crate::app::domain::models::{Assessment, Risk};
use chrono::NaiveDateTime;
use diesel::pg::data_types::Cents;
use uuid::Uuid;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "assessment"]
pub struct AssessmentEntity {
    id: Uuid,
    order_number: i64,
    customer_id: Option<Uuid>,
    customer_name: String,
    customer_cpf: String,
    card_number: String,
    card_holder_name: String,
    creation_date_order: NaiveDateTime,
    value: f64,
    status: String,
    motivation: Option<String>,
    create_at: NaiveDateTime,
    update_at: Option<NaiveDateTime>,
}

impl AssessmentEntity {
    pub fn map_to_insert(ass: Assessment) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_number: ass.risk.order_number,
            customer_id: ass.risk.customer_id,
            customer_name: ass.risk.customer_name,
            customer_cpf: ass.risk.customer_cpf,
            card_number: ass.risk.card_number,
            card_holder_name: ass.risk.card_holder_name,
            creation_date_order: ass.risk.creation_date_order,
            value: ass.risk.value,
            status: ass.status,
            motivation: ass.motivation,
            create_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
            update_at: None,
        }
    }

    pub fn map_from_db(ass: AssessmentEntity) -> Assessment {
        Assessment {
            id: Option::from(ass.id),
            risk: Risk {
                order_number: ass.order_number,
                customer_id: ass.customer_id,
                customer_name: ass.customer_name,
                customer_cpf: ass.customer_cpf,
                card_number: ass.card_number,
                card_holder_name: ass.card_holder_name,
                value: ass.value,
                creation_date_order: ass.creation_date_order,
            },
            status: ass.status,
            motivation: ass.motivation,
            create_at: Option::from(ass.create_at),
            update_at: ass.update_at,
        }
    }
}
