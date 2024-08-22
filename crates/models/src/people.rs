use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx_ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub person_id: Ulid,
    pub name: String,
}

impl Person {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        let query = sqlx::query!(
            r#"
            SELECT
                person_id::uuid as "person_id: Ulid",
                name
            FROM 
                people
            WHERE 
                person_id::uuid = $1
            "#,
            id.to_sqlx_uuid()
        );

        let person = query.fetch_one(pool).await?;
        match person.person_id {
            Some(person_id) => {
                let person = Person {
                    person_id,
                    name: person.name,
                };

                Ok(Some(person))
            }
            _ => Ok(None),
        }
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO 
                people(person_id, name)
            SELECT 
                person_id::uuid, name,
            FROM(
                VALUES(
                    $1, $2
                )
            ) AS data(person_id, name)
            "#,
        )
        .bind(self.person_id.queryable())
        .bind(&self.name)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        _pagination: crate::Pagination,
    ) -> Result<(Vec<Self>, crate::Pagination)> {
        // TODO: Implement pagination for querying
        let person = sqlx::query!(
            r#"
            SELECT
                person_id::uuid as "person_id: Ulid",
                name
            FROM 
                people
            "#,
        )
        .fetch_all(pool)
        .await?;

        let person = person
            .into_iter()
            .filter_map(|person| match person.person_id {
                Some(person_id) => Some(Person {
                    person_id,
                    name: person.name,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: person.last().map(|person| person.person_id),
            count: person.len(),
        };

        Ok((person, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    person_id: Option<Ulid>,
    name: Option<String>,
}

impl Builder {
    pub fn person_id(mut self, person_id: Ulid) -> Self {
        self.person_id = Some(person_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = Person;

    fn build(self) -> Result<Self::Item> {
        let person_id = self.person_id.or_else(|| Some(Ulid::generate())).unwrap();
        let name = self.name.ok_or_else(|| Error::ModelFieldsMissing("name"))?;

        Ok(Person { person_id, name })
    }
}
