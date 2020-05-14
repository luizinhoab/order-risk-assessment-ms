use chrono::NaiveDateTime;
use diesel::pg::data_types::Cents;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Risk {
    pub order_number: i64,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_cpf: String,
    pub card_number: String,
    pub card_holder_name: String,
    pub value: f64,
    pub creation_date_order: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub id: Option<Uuid>,
    pub risk: Risk,
    pub status: String,
    pub motivation: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}
