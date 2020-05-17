use crate::app::domain::models::{Assessment, Risk};
use crate::app::domain::repository::Repository;
use crate::errors::CustomError;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DBError};

pub struct RiskService {
    repository: Box<dyn Repository<Assessment, CustomError> + Send + Sync>,
}

impl RiskService {
    pub fn new(repository: Box<dyn Repository<Assessment, CustomError> + Send + Sync>) -> Self {
        Self { repository }
    }
    pub fn assess_risk(self, risk: Risk) -> Result<Assessment, CustomError> {
        let assessment = Assessment {
            id: None,
            risk,
            status: "APPROVED".to_string(),
            motivation: None,
            create_at: None,
            update_at: None,
        };
        let result = self.repository.save(assessment)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::domain::repository::MockRepository;

    #[test]
    fn given_risk_service_when_save_success_expect_ok() {
        let mut mock = MockRepository::<Assessment, CustomError>::new();

        let risk = Risk {
            order_number: 0,
            customer_id: None,
            customer_name: "".to_string(),
            customer_cpf: "".to_string(),
            card_number: "".to_string(),
            card_holder_name: "".to_string(),
            value: 0.0,
            creation_date_order: chrono::Local::now().naive_local(),
        };

        mock.expect_save().returning(|x| Ok(x));

        let service = RiskService {
            repository: Box::new(mock),
        };

        assert!(service.assess_risk(risk).is_ok());
    }

    #[test]
    fn given_risk_service_when_assess_save_error_expect_err() {
        let mut mock = MockRepository::<Assessment, CustomError>::new();

        let risk = Risk {
            order_number: 0,
            customer_id: None,
            customer_name: "".to_string(),
            customer_cpf: "".to_string(),
            card_number: "".to_string(),
            card_holder_name: "".to_string(),
            value: 0.0,
            creation_date_order: chrono::Local::now().naive_local(),
        };

        mock.expect_save()
            .returning(|x| Err(CustomError::InternalServerError));

        let service = RiskService {
            repository: Box::new(mock),
        };

        assert!(service.assess_risk(risk).is_err());
    }
}
