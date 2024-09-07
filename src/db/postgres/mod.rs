use crate::prelude::{Result, *, Error};
mod sql;
use sql::*;

use sqlx::postgres::PgPool;

use sqlx::*;
use uuid::Uuid;

#[rocket::async_trait]
impl DBConnection for PgPool {
	async fn init(&self) -> Result<()> {
		query(CREATE_TABLE).execute(self).await?;
		Ok(())
	}
	async fn create_user(&self, id: Uuid, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
		match query(INSERT_USER)
			.bind(id)
			.bind(email)
			.bind(hash)
			.bind(is_admin)
			.execute(self)
			.await {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::from(e)),
			}
	}
	async fn update_user(&self, user: &User) -> Result<(), Error> {
		match query(UPDATE_USER)
			.bind(user.id)
			.bind(&user.email)
			.bind(&user.password)
			.bind(user.is_admin)
			.execute(self)
			.await {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::from(e)),
			}
	}
	async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), Error> {
		match query(REMOVE_BY_ID).bind(user_id).execute(self).await {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::from(e)),
		}
	}
	async fn delete_user_by_email(&self, email: &str) -> Result<(), Error> {
		match query(REMOVE_BY_EMAIL).bind(email).execute(self).await {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::from(e)),
		}
	}
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Error> {
		match query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await {
			Ok(user) => Ok(user),
			Err(e) => Err(Error::from(e)),
		}
	}
	async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
		match query_as(SELECT_BY_EMAIL).bind(email).fetch_one(self).await {
			Ok(user) => Ok(user),
			Err(e) => Err(Error::from(e)),
		}
	}
}
	