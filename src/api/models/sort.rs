use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, Debug, IntoParams)]
pub struct SortParams {
    #[param(example = "desc")]
    pub order: Option<String>,
}
