use crate::app::domain::individual_tax_payer_service::IndividualTaxPayerService;
use crate::app::domain::models::IndividualTaxPayer;
use crate::errors::CustomError;
use crate::infra::http::document::CPFResponseBody;
use async_trait::async_trait;
use reqwest::Client;
use std::env::var;

pub struct RFBClient;

#[async_trait]
impl IndividualTaxPayerService for RFBClient {
    async fn find_payer_by_number(number: String) -> Result<IndividualTaxPayer, CustomError> {
        let url = var("RFP_BASE_URL").expect("RFP_BASE_URL environment variable not found.");
        let token = var("RFP_API_TOKEN").expect("RFP_API_TOKEN environment variable not found.");
        let service_url = format!("{}{}", url, number);

        let client = Client::new();

        let result = client
            .get(service_url.as_str())
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        match result.status().as_u16() {
            200 => {
                let body = result.json::<CPFResponseBody>().await.unwrap();
                Ok(body.map_to_domain())
            }
            404 => Ok(IndividualTaxPayer {
                number,
                name: "".to_string(),
                situation_code: "Nonexistent".to_string(),
                situation_description: "".to_string(),
            }),
            _ => {
                error!(
                    "The service {} answered with status {}",
                    result.url(),
                    result.status().as_str()
                );
                Err(CustomError::IntegrationError(format!(
                    "The service {} answered with status {}",
                    result.url(),
                    result.status().as_str()
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate httpmock;
    use super::*;
    use httpmock::Method::GET;
    use httpmock::{mock, with_mock_server};
    use std::env::set_var;

    lazy_static! {
        #[derive(Copy, Clone, Debug)]
        pub static ref HOST_TEST: String =
            "http://localhost:5000/consulta-cpf-df-trial/v1/cpf/".to_string();
        #[derive(Copy, Clone, Debug)]
        pub static ref TOKEN_TEST: String = "4e1a1858bdd584fdc077fb7d80f39283".to_string();
    }

    #[actix_rt::test]
    #[with_mock_server]
    async fn given_rfb_client_when_find_payer_by_number_valid_cpf_expect_ok() {
        let mock = mock(GET, "/consulta-cpf-df-trial/v1/cpf/63017285995")
            .return_status(200)
            .return_body("{\"ni\":\"63017285995\",\"nome\":\"Nome do CPF 630.172.859-95\",\"situacao\":{\"codigo\":\"0\",\"descricao\":\"Regular\"}}")
            .create();

        set_var("RFP_BASE_URL", &**HOST_TEST);
        set_var("RFP_API_TOKEN", &**TOKEN_TEST);

        let result = RFBClient::find_payer_by_number("63017285995".parse().unwrap()).await;

        assert_eq!(mock.times_called(), 1);
        assert!(result.is_ok())
    }

    #[actix_rt::test]
    #[with_mock_server]
    async fn given_rfb_client_when_find_payer_by_number_invalid_cpf_expect_err() {
        let mock = mock(GET, "/consulta-cpf-df-trial/v1/cpf/63017285991")
            .return_status(500)
            .return_body("{\"mensagem\":\"CPF <63017285991> inválido!\"}")
            .create();

        set_var("RFP_BASE_URL", &**HOST_TEST);
        set_var("RFP_API_TOKEN", &**TOKEN_TEST);

        let result = RFBClient::find_payer_by_number("63017285991".parse().unwrap()).await;

        assert_eq!(mock.times_called(), 1);
        assert!(result.is_err())
    }

    #[actix_rt::test]
    #[with_mock_server]
    async fn given_rfb_client_when_find_payer_by_number_nonexistent_cpf_expect_err() {
        let mock = mock(GET, "/consulta-cpf-df-trial/v1/cpf/02401374000")
            .return_status(404)
            .return_body("Nenhum registro encontrado.")
            .create();

        set_var("RFP_BASE_URL", &**HOST_TEST);
        set_var("RFP_API_TOKEN", &**TOKEN_TEST);

        let result = RFBClient::find_payer_by_number("02401374000".parse().unwrap()).await;

        assert_eq!(mock.times_called(), 1);
        assert!(result.is_ok())
    }
}
