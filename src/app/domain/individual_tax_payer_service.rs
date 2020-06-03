use crate::app::domain::models::IndividualTaxPayer;
use crate::errors::CustomError;
use async_trait::async_trait;

#[async_trait]
pub trait IndividualTaxPayerService {
    async fn find_payer_by_number(&self, number: String)
        -> Result<IndividualTaxPayer, CustomError>;
}
