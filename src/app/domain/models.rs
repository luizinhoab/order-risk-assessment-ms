use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Risk {
    pub order_number: i32,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_cpf: String,
    pub card_number: String,
    pub card_holder_name: String,
    pub value: f32,
    pub creation_date_order: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct Assessment {
    pub id: Option<Uuid>,
    pub risk: Risk,
    pub status: String,
    pub motivation: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct IndividualTaxPayer {
    pub number: String,
    pub name: String,
    pub situation_code: String,
    pub situation_description: String,
}
