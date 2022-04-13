use convert_case::{Casing, Case};
use poem::{handler, web::{Query, Json}};
use serde::Deserialize;
use serde_json::json;



#[derive(Deserialize)]
pub struct QueryParams {
    pub name: Option<String>,
}

#[handler]
pub async fn helloworld_handler(Query(QueryParams { name }): Query<QueryParams>) -> String {
    let mut name_var = "Stranger".to_string();
    if name.is_some() && !name.to_owned().unwrap().is_empty() {
        name_var = name.unwrap().to_case(Case::Title);
    }
    return "Hello ".to_owned() + &name_var;
}

#[handler]
pub async fn versionz_handler() -> Json<serde_json::Value> {
    return Json(json!({"name":env!("CARGO_PKG_NAME"),"rev":env!("GIT_SHA","Git Commit Sha is undefined.")}));
}
