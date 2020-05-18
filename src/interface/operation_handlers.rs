use crate::app::risk_assessment_service::RiskService;
use crate::errors::CustomError;
use crate::infra::risk_postgres::RiskDieselPg;
use crate::interface::documents::RiskRequestBody;
use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{post, web, HttpRequest, HttpResponse, Result};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::path::PathBuf;
use validator::Validate;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn init_service(pool: &Pool) -> RiskService {
    let repository = Box::new(RiskDieselPg::new(pool.clone()));
    RiskService::new(repository)
}

#[post("/risk/assessment")]
pub async fn handle_assessment_risk(
    (document, pool): (web::Json<RiskRequestBody>, Data<Pool>),
) -> Result<HttpResponse, CustomError> {
    document.validate()?;
    let risk_service = init_service(&pool);
    let risk = document.map_to_domain();

    let result = web::block(move || risk_service.assess_risk(risk))
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(&result))
}

pub async fn handle_statics_by_path(path: String, req: HttpRequest) -> Result<NamedFile> {
    let file_name = req.match_info().query("filename");
    let file_path = PathBuf::from(format!("{}{}", path, file_name));
    let file = NamedFile::open(file_path)?;
    Ok(file)
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn given_handle_statics_by_path_when_file_exists_expect_ok() {
        let mock_req = test::TestRequest::default().to_http_request();

        let resp =
            handle_statics_by_path("doc/swaggerui/index.html".parse().unwrap(), mock_req).await;
        assert!(resp.is_ok());
    }

    #[actix_rt::test]
    async fn given_handle_statics_by_path_when_file_exists_expect_err() {
        let mock_req = test::TestRequest::default().to_http_request();
        let resp = handle_statics_by_path("doc/swaggerui/non.js".parse().unwrap(), mock_req).await;
        assert!(resp.is_err());
    }
}
