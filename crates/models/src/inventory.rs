use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx_ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub inventory_id: Ulid,
    pub ingredient_type: String,
    pub quantity: String,
    pub created_at: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl Inventory {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        let query = sqlx::query!(
            r#"
            SELECT
                inventory_id::uuid as "inventory_id: Ulid",
                ingredient_type,
                quantity,
                created_at,
                expiration_date
            FROM 
                inventory
            WHERE 
                inventory_id::uuid = $1
            "#,
            id.to_sqlx_uuid()
        );

        let inventory = query.fetch_one(pool).await?;
        match inventory.inventory_id {
            Some(inventory_id) => {
                let inventory = Inventory {
                    inventory_id,
                    ingredient_type: inventory.ingredient_type,
                    quantity: inventory.quantity,
                    created_at: inventory.created_at,
                    expiration_date: inventory.expiration_date,
                };

                Ok(Some(inventory))
            }
            _ => Ok(None),
        }
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO 
                inventory(inventory_id, ingredient_type, quantity, created_at, expiration_date)
            SELECT 
                inventory_id::uuid, ingredient_type, quantity, created_at, expiration_date::timestamptz
            FROM(
                VALUES(
                    $1, $2, $3, 
                    $4, $5
                )
            ) AS data(inventory_id, ingredient_type, quantity, created_at, expiration_date)
            "#,
        )
        .bind(self.inventory_id.queryable())
        .bind(&self.ingredient_type)
        .bind(&self.quantity)
        .bind(self.created_at)
        .bind(self.expiration_date)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        _pagination: crate::Pagination,
    ) -> Result<(Vec<Self>, crate::Pagination)> {
        // TODO: Implement pagination for querying
        let inventory = sqlx::query!(
            r#"
            SELECT
                inventory_id::uuid as "inventory_id: Ulid",
                ingredient_type,
                quantity,
                created_at,
                expiration_date
            FROM 
                inventory
            "#,
        )
        .fetch_all(pool)
        .await?;

        let inventory = inventory
            .into_iter()
            .filter_map(|inventory| match inventory.inventory_id {
                Some(inventory_id) => Some(Inventory {
                    inventory_id,
                    ingredient_type: inventory.ingredient_type,
                    quantity: inventory.quantity,
                    created_at: inventory.created_at,
                    expiration_date: inventory.expiration_date,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: inventory.last().map(|inventory| inventory.inventory_id),
            count: inventory.len(),
        };

        Ok((inventory, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    inventory_id: Option<Ulid>,
    ingredient_type: Option<String>,
    quantity: Option<String>,
    created_at: Option<DateTime<Utc>>,
    expiration_date: Option<DateTime<Utc>>,
}

impl Builder {
    pub fn inventory_id(mut self, inventory_id: Ulid) -> Self {
        self.inventory_id = Some(inventory_id);
        self
    }

    pub fn ingredient_type(mut self, ingredient_type: String) -> Self {
        self.ingredient_type = Some(ingredient_type);
        self
    }

    pub fn quantity(mut self, quantity: String) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn expiration_date(mut self, expiration_date: Option<DateTime<Utc>>) -> Self {
        self.expiration_date = expiration_date;
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = Inventory;

    fn build(self) -> Result<Self::Item> {
        let inventory_id = self
            .inventory_id
            .or_else(|| Some(Ulid::generate()))
            .unwrap();
        let ingredient_type = self
            .ingredient_type
            .ok_or_else(|| Error::ModelFieldsMissing("ingredient_type"))?;
        let quantity = self
            .quantity
            .ok_or_else(|| Error::ModelFieldsMissing("quantity"))?;
        let created_at = self
            .created_at
            .ok_or_else(|| Error::ModelFieldsMissing("created_at"))?;
        let expiration_date = self.expiration_date;

        Ok(Inventory {
            inventory_id,
            ingredient_type,
            quantity,
            created_at,
            expiration_date,
        })
    }
}
