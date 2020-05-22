use crate::app::domain::models::{Assessment, Risk};
use crate::schema::assessment;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Insertable, Queryable)]
#[table_name = "assessment"]
pub struct AssessmentEntity {
    pub id: Uuid,
    pub order_number: i32,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_cpf: String,
    pub card_number: String,
    pub card_holder_name: String,
    pub creation_date_order: NaiveDateTime,
    pub value: f32,
    pub status: String,
    pub motivation: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: Option<NaiveDateTime>,
}

impl AssessmentEntity {
    pub fn map_to_insert(ass: Assessment) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_number: i32::from(ass.risk.order_number),
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
