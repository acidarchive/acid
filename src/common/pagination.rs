use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PaginatedResponse<T>
where
    T: Serialize,
{
    pub records: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total: Option<i64>,
    pub total_pages: Option<i64>,
}

impl<T> PaginatedResponse<T>
where
    T: Serialize,
{
    pub fn new(
        records: Vec<T>,
        page: i64,
        page_size: i64,
        total: Option<i64>,
        total_pages: Option<i64>,
    ) -> Self {
        Self {
            records,
            page,
            page_size,
            total,
            total_pages,
        }
    }
}

impl<T> From<sqlx_paginated::PaginatedResponse<T>> for PaginatedResponse<T>
where
    T: Serialize,
{
    fn from(sqlx_response: sqlx_paginated::PaginatedResponse<T>) -> Self {
        Self {
            records: sqlx_response.records,
            page: sqlx_response.pagination.clone().unwrap().page,
            page_size: sqlx_response.pagination.unwrap().page_size,
            total: sqlx_response.total,
            total_pages: sqlx_response.total_pages,
        }
    }
}
