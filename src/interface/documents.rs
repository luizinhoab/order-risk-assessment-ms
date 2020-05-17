use crate::app::domain::models::Risk;
use regex::Regex;
use uuid::Uuid;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ISO8601: Regex = Regex::new(r"^([1][9][0-9]{2}|[2][0-9]{3})-([1-9]|([0][1-9]|[1][0-2]))-(([0][1-9]|[1][0-9]|[2][0-9]|[3][0-1]))T(\d{2}):(\d{2}):(\d{2})$").unwrap();
    static ref INTEGER: Regex = Regex::new(r"^\d+$").unwrap();
}

impl RiskRequestBody {
    pub fn mapToDomain(&self) -> Risk {
        Risk {
            order_number: self.order_number.parse().unwrap(),
            customer_id: self.customer_id,
            customer_name: self.customer_name.clone(),
            customer_cpf: self.customer_cpf.clone(),
            card_number: self.card_number.clone(),
            card_holder_name: self.card_holder_name.clone(),
            value: self.value,
            creation_date_order: chrono::NaiveDateTime::parse_from_str(
                self.creation_date_order.as_ref(),
                "%Y-%m-%dT%H:%M:%S",
            )
            .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct RiskRequestBody {
    #[validate(regex = "INTEGER")]
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    #[validate(length(min = 1, message = "Invalid name"))]
    pub customer_name: String,
    #[validate(length(min = 11, max = 11, message = "Invalid cpf"))]
    pub customer_cpf: String,
    #[validate(length(max = 16))]
    pub card_number: String,
    pub card_holder_name: String,
    #[validate(length(min = 4, max = 4, message = "Invalid card expiration date"))]
    pub card_expiration: String,
    pub value: f64,
    #[validate(regex = "ISO8601")]
    pub creation_date_order: String,
}
