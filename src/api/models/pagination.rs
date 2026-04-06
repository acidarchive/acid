use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, Debug, IntoParams)]
pub struct PaginationParams {
    #[param(minimum = 1, maximum = 100, default = 20, example = 20)]
    pub limit: Option<i64>,
    #[param(minimum = 0, default = 0, example = 0)]
    pub offset: Option<i64>,
}
