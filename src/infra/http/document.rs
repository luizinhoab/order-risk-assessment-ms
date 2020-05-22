use crate::app::domain::models::IndividualTaxPayer;

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
        IndividualTaxPayer {
            number: self.ni.clone(),
            name: self.nome.clone(),
            situation_code: self.situacao.codigo.clone(),
            situation_description: self.situacao.descricao.clone(),
        }
    }
}
