use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx_ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chore {
    pub chore_id: Ulid,
    pub name: String,
    pub description: Option<String>,
    pub rrule: String,
}

impl Chore {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        let query = sqlx::query!(
            r#"
            SELECT
                chore_id::uuid as "chore_id: Ulid",
                name,
                description,
                rrule
            FROM 
                chores
            WHERE 
                chore_id::uuid = $1
            "#,
            id.to_sqlx_uuid()
        );

        let chore = query.fetch_one(pool).await?;
        match chore.chore_id {
            Some(chore_id) => {
                let chore = Chore {
                    chore_id,
                    name: chore.name,
                    description: chore.description,
                    rrule: chore.rrule,
                };

                Ok(Some(chore))
            }
            _ => Ok(None),
        }
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO 
                chores(chore_id, name)
            SELECT 
                chore_id::uuid, name, description, rrule
            FROM(
                VALUES(
                    $1, $2
                )
            ) AS data(chore_id, name)
            "#,
        )
        .bind(self.chore_id.queryable())
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.rrule)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        _pagination: crate::Pagination,
    ) -> Result<(Vec<Self>, crate::Pagination)> {
        // TODO: Implement pagination for querying
        let chore = sqlx::query!(
            r#"
            SELECT
                chore_id::uuid as "chore_id: Ulid",
                name,
                description,
                rrule
            FROM 
                chores
            "#,
        )
        .fetch_all(pool)
        .await?;

        let chore = chore
            .into_iter()
            .filter_map(|chore| match chore.chore_id {
                Some(chore_id) => Some(Chore {
                    chore_id,
                    name: chore.name,
                    description: chore.description,
                    rrule: chore.rrule,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: chore.last().map(|chore| chore.chore_id),
            count: chore.len(),
        };

        Ok((chore, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    chore_id: Option<Ulid>,
    name: Option<String>,
    description: Option<String>,
    rrule: Option<String>,
}

impl Builder {
    pub fn chore_id(mut self, chore_id: Ulid) -> Self {
        self.chore_id = Some(chore_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn rrule(mut self, rrule: String) -> Self {
        self.rrule = Some(rrule);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = Chore;

    fn build(self) -> Result<Self::Item> {
        let chore_id = self.chore_id.or_else(|| Some(Ulid::generate())).unwrap();
        let name = self.name.ok_or_else(|| Error::ModelFieldsMissing("name"))?;
        let description = self.description;
        let rrule = self
            .rrule
            .ok_or_else(|| Error::ModelFieldsMissing("rrule"))?;

        Ok(Chore {
            chore_id,
            name,
            description,
            rrule,
        })
    }
}
