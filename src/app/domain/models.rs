use crate::app::domain::models::IndividualTaxPayerSituations::NONEXISTENT;
use chrono::NaiveDateTime;
use derive_more::Display;
use std::borrow::Borrow;
use std::ops::Add;
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
    pub status: AssessmentStatus,
    pub motivation: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

impl Assessment {
    pub fn new(risk: Risk) -> Self {
        Self {
            id: None,
            risk,
            status: AssessmentStatus::APPROVED,
            motivation: None,
            create_at: None,
            update_at: None,
        }
    }
    pub fn verifyInvidualTaxPayerInformation(&self, register: IndividualTaxPayer) -> Self {
        match register.situation {
            IndividualTaxPayerSituations::AUTHORIZED => {
                let mut status = None;
                let mut motivation = if !self.checkIndividualTaxPayerByName(register.name) {
                    Option::from(
                        "Divergence with name registered on individual tax payer service;"
                            .to_string(),
                    )
                } else {
                    None
                };

                Assessment {
                    id: None,
                    risk: self.risk.clone(),
                    status: status.unwrap_or_else(|| self.status.clone()),
                    motivation,
                    create_at: None,
                    update_at: None,
                }
            }
            _ => Assessment {
                id: None,
                risk: self.risk.clone(),
                status: AssessmentStatus::REPROVED,
                motivation: self.motivation.as_ref().map(|f| {
                    f.clone()
                        .add(
                            Option::from(register.situation_description)
                                .unwrap()
                                .as_ref(),
                        )
                        .add(" ")
                }),
                create_at: None,
                update_at: None,
            },
        }
    }

    fn checkIndividualTaxPayerByName(&self, name: String) -> bool {
        let registered_first_name = String::from(name.split_whitespace().next().unwrap());
        let first_name = String::from(self.risk.customer_name.split_whitespace().next().unwrap());
        let registered_last_name = String::from(name.split_whitespace().last().unwrap());
        let last_name = String::from(self.risk.customer_name.split_whitespace().last().unwrap());

        first_name.eq_ignore_ascii_case(registered_first_name.as_ref())
            && last_name.eq_ignore_ascii_case(registered_last_name.as_ref())
    }
}

pub struct IndividualTaxPayer {
    pub number: String,
    pub name: String,
    pub situation: IndividualTaxPayerSituations,
    pub situation_description: String,
}

pub enum IndividualTaxPayerSituations {
    AUTHORIZED,
    UNAUTHORIZED,
    NONEXISTENT,
}

#[derive(Debug, Serialize, Display, Clone)]
pub enum AssessmentStatus {
    #[display(fmt = "APPROVED")]
    APPROVED,
    #[display(fmt = "ANALYZING")]
    ANALYZING,
    #[display(fmt = "REPROVED")]
    REPROVED,
}

impl AssessmentStatus {
    pub fn from(status: String) -> Self {
        match status.to_uppercase().as_str() {
            "APPROVED" => AssessmentStatus::APPROVED,
            "REPROVED" => AssessmentStatus::REPROVED,
            "ANALYZING" => AssessmentStatus::ANALYZING,
            _ => AssessmentStatus::ANALYZING,
        }
    }
}
