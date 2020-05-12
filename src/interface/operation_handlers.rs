use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

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
        let resp = handle_statics_by_path("doc/swaggerui/index.html".parse().unwrap(), mock_req).await;
        assert!(resp.is_ok());
    }

    #[actix_rt::test]
    async fn given_handle_statics_by_path_when_file_exists_expect_err() {
        let mock_req = test::TestRequest::default().to_http_request();
        let resp = handle_statics_by_path("doc/swaggerui/non.js".parse().unwrap(), mock_req).await;
        assert!(resp.is_err());
    }
}
