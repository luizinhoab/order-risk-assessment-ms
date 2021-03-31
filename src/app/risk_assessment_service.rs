use crate::app::domain::individual_tax_payer_service::IndividualTaxPayerService;
use crate::app::domain::models::{Assessment, IndividualTaxPayer, Risk};
use crate::app::domain::repository::Repository;
use crate::errors::CustomError;
use actix_web::web;
use std::sync::Arc;

pub struct RiskService {
    repository: Arc<dyn Repository<Assessment, CustomError> + Send + Sync>,
    individual_payer_service: Arc<dyn IndividualTaxPayerService + Send + Sync>,
}

impl RiskService {
    pub fn new(
        repository: Arc<dyn Repository<Assessment, CustomError> + Send + Sync>,
        individual_payer_service: Arc<dyn IndividualTaxPayerService + Send + Sync>,
    ) -> RiskService {
        Self {
            repository,
            individual_payer_service,
        }
    }
    pub async fn assess_risk(self, risk: Risk) -> Result<Assessment, CustomError> {
        let individual_tax_payer = self
            .individual_payer_service
            .find_payer_by_number(risk.clone().customer_cpf)
            .await?;

        let assessment =
            Assessment::new(risk).verifyInvidualTaxPayerInformation(individual_tax_payer);

        let result = web::block(move || self.repository.save(assessment))
            .await
            .map_err(|_| CustomError::InternalServerError)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::domain::models::IndividualTaxPayerSituations;
    use crate::app::domain::repository::MockRepository;
    use async_trait::async_trait;

    struct MockIndividualTaxPayerService;

    impl MockIndividualTaxPayerService {
        fn new() -> MockIndividualTaxPayerService {
            MockIndividualTaxPayerService
        }
    }

    #[async_trait]
    impl IndividualTaxPayerService for MockIndividualTaxPayerService {
        async fn find_payer_by_number(
            &self,
            number: String,
        ) -> Result<IndividualTaxPayer, CustomError> {
            match number.as_str() {
                "12345678910" => Ok(IndividualTaxPayer {
                    number,
                    name: "Robert Cecil Martin".to_string(),
                    situation: IndividualTaxPayerSituations::AUTHORIZED,
                    situation_description: "Regular".to_string(),
                }),
                "00000000000" => Ok(IndividualTaxPayer {
                    number,
                    name: "Robert Cecil Martin".to_string(),
                    situation: IndividualTaxPayerSituations::NONEXISTENT,
                    situation_description: "".to_string(),
                }),
                "11111111111" => Ok(IndividualTaxPayer {
                    number,
                    name: "Robert Cecil Martin".to_string(),
                    situation: IndividualTaxPayerSituations::UNAUTHORIZED,
                    situation_description: "".to_string(),
                }),
                _ => Err(CustomError::IntegrationError(
                    "Mocked Error".parse().unwrap(),
                )),
            }
        }
    }

    #[actix_rt::test]
    async fn given_risk_service_when_assess_risk_with_authorized_expect_ok() {
        let mut mock_repository = MockRepository::new();
        let mock_service = MockIndividualTaxPayerService::new();

        let risk = Risk {
            order_number: 666,
            customer_id: None,
            customer_name: "Robert Cecil Martin".to_string(),
            customer_cpf: "12345678910".to_string(),
            card_number: "4444333322221111".to_string(),
            card_holder_name: "Robert Cecil Martin".to_string(),
            value: 150.10,
            creation_date_order: chrono::Local::now().naive_local(),
        };

        mock_repository.expect_save().returning(|x| Ok(x));

        let service = RiskService::new(Arc::new(mock_repository), Arc::new(mock_service));

        assert!(service.assess_risk(risk).await.is_ok());
    }

    #[actix_rt::test]
    async fn given_risk_service_when_assess_risk_with_nonexistent_expect_ok() {
        let mut mock_repository = MockRepository::<Assessment, CustomError>::new();
        let mock_service = MockIndividualTaxPayerService::new();

        let risk = Risk {
            order_number: 666,
            customer_id: None,
            customer_name: "Robert Cecil Martin".to_string(),
            customer_cpf: "00000000000".to_string(),
            card_number: "4444333322221111".to_string(),
            card_holder_name: "Robert Cecil Martin".to_string(),
            value: 150.10,
            creation_date_order: chrono::Local::now().naive_local(),
        };

        mock_repository.expect_save().returning(|x| Ok(x));

        let service = RiskService::new(Arc::new(mock_repository), Arc::new(mock_service));

        let result = service.assess_risk(risk).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn given_risk_service_when_assess_risk_with_error_on_tax_payer_service_expect_err() {
        let mut mock_repository = MockRepository::new();
        let mock_service = MockIndividualTaxPayerService::new();

        let risk = Risk {
            order_number: 666,
            customer_id: None,
            customer_name: "Robert Cecil Martin".to_string(),
            customer_cpf: "".to_string(),
            card_number: "4444333322221111".to_string(),
            card_holder_name: "Robert Cecil Martin".to_string(),
            value: 150.10,
            creation_date_order: chrono::Local::now().naive_local(),
        };
        let service = RiskService::new(Arc::new(mock_repository), Arc::new(mock_service));

        let result = service.assess_risk(risk).await;
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn given_risk_service_when_assess_risk_with_authorized_with_error_db_expect_err() {
        let mut mock_repository = MockRepository::new();
        let mock_service = MockIndividualTaxPayerService::new();

        let risk = Risk {
            order_number: 666,
            customer_id: None,
            customer_name: "Robert Cecil Martin".to_string(),
            customer_cpf: "11111111111".to_string(),
            card_number: "4444333322221111".to_string(),
            card_holder_name: "Robert Cecil Martin".to_string(),
            value: 150.10,
            creation_date_order: chrono::Local::now().naive_local(),
        };

        let service = RiskService::new(Arc::new(mock_repository), Arc::new(mock_service));

        let result = service.assess_risk(risk).await;
        assert!(result.is_err());
    }
}
