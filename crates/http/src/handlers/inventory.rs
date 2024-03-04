use crate::{error::Result, ServerState};
use annapurna_models::{
    entity::Builder,
    inventory::{Builder as InventoryBuilder, Inventory},
};
use axum::{extract::State, Json};
use sqlx::types::chrono::{self, DateTime, Utc};
use sqlx_ulid::Ulid;

pub(crate) async fn list_inventory(
    State(ServerState { pg_pool, .. }): State<ServerState>,
) -> Result<Json<Vec<Inventory>>> {
    let pagination = annapurna_models::Pagination {
        last_key: None,
        count: 10,
    };

    let query = Inventory::query(&pg_pool, pagination).await?;
    let (items, _pagination) = query;

    Ok(Json(items))
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CreateInventory {
    pub inventory_id: Option<Ulid>,
    pub ingredient_type: String,
    pub quantity: String,
    pub expiration_date: Option<DateTime<Utc>>,
}

pub(crate) async fn create_inventory(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    payload: axum::extract::Json<CreateInventory>,
) -> Result<Json<Inventory>> {
    let created_at = chrono::Utc::now();
    let inventory_id = payload.inventory_id.unwrap_or_else(Ulid::generate);

    let item = InventoryBuilder::default()
        .inventory_id(inventory_id)
        .ingredient_type(payload.ingredient_type.clone())
        .quantity(payload.quantity.clone())
        .created_at(created_at)
        .expiration_date(payload.expiration_date)
        .build()?;

    item.create(&pg_pool).await?;

    Ok(Json(item))
}
