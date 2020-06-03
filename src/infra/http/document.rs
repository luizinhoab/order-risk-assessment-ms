use crate::app::domain::models::{IndividualTaxPayer, IndividualTaxPayerSituations};

#[derive(Debug, Clone, Deserialize)]
pub struct CPFResponseBody {
    pub ni: String,
    pub nome: String,
    pub situacao: SituationCPF,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SituationCPF {
    pub codigo: String,
    pub descricao: String,
}

impl CPFResponseBody {
    pub fn map_to_domain(&self) -> IndividualTaxPayer {
        let situation = match self.situacao.codigo.as_str() {
            "0" | "4" => IndividualTaxPayerSituations::AUTHORIZED,
            _ => IndividualTaxPayerSituations::UNAUTHORIZED,
        };
        IndividualTaxPayer {
            number: self.ni.clone(),
            name: self.nome.clone(),
            situation,
            situation_description: self.situacao.descricao.clone(),
        }
    }
}
