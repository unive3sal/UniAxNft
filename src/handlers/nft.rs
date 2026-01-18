use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::error::{UniAxNftErr, UniAxNftResult};
use crate::middleware::auth::UserInfo;
use crate::state::UniAxNftState;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

fn default_sort() -> String {
    "created_at".to_string()
}

fn default_order() -> String {
    "desc".to_string()
}

#[derive(Debug, Serialize)]
pub struct NftResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub metadata_url: Option<String>,
    pub mint_address: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct PaginatedNftResponse {
    pub data: Vec<NftResponse>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

pub async fn user_nft_info(
    State(state): State<UniAxNftState>,
    Extension(user_info): Extension<UserInfo>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> UniAxNftResult<impl IntoResponse> {
    // Validate that the authenticated user can only access their own NFTs
    if user_info.user_id != user_id {
        return Err(UniAxNftErr::AuthErr(
            "You can only access your own NFTs".to_string(),
        ));
    }

    // Validate pagination parameters
    let page = params.page.max(1);
    let limit = params.limit.clamp(1, 100);
    let offset = (page - 1) * limit;

    // Validate sort column to prevent SQL injection
    let sort_column = match params.sort.as_str() {
        "created_at" | "updated_at" | "name" | "status" => params.sort.as_str(),
        _ => "created_at",
    };

    // Validate order direction
    let order_direction = match params.order.to_lowercase().as_str() {
        "asc" => "ASC",
        _ => "DESC",
    };

    // Get total count
    let count_row = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM nfts
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| UniAxNftErr::DatabaseErr(format!("Failed to count NFTs: {}", e)))?;

    let total: i64 = count_row.get("count");
    let total_pages = (total + limit - 1) / limit;

    // Build and execute the query with dynamic ORDER BY
    // Note: We validate sort_column above to prevent SQL injection
    let query = format!(
        r#"
        SELECT id, user_id, name, description, image_url, metadata_url, mint_address, status, created_at, updated_at
        FROM nfts
        WHERE user_id = $1
        ORDER BY {} {}
        LIMIT $2 OFFSET $3
        "#,
        sort_column, order_direction
    );

    let rows = sqlx::query(&query)
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| UniAxNftErr::DatabaseErr(format!("Failed to fetch NFTs: {}", e)))?;

    let nfts: Vec<NftResponse> = rows
        .iter()
        .map(|row| {
            let created_at: PrimitiveDateTime = row.get("created_at");
            let updated_at: PrimitiveDateTime = row.get("updated_at");
            NftResponse {
                id: row.get("id"),
                user_id: row.get("user_id"),
                name: row.get("name"),
                description: row.get("description"),
                image_url: row.get("image_url"),
                metadata_url: row.get("metadata_url"),
                mint_address: row.get("mint_address"),
                status: row.get("status"),
                created_at: created_at.to_string(),
                updated_at: updated_at.to_string(),
            }
        })
        .collect();

    Ok(Json(PaginatedNftResponse {
        data: nfts,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}
