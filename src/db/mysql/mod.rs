use crate::prelude::{Result, *, Error};
mod sql;
use sql::*;

use sqlx::mysql::MySqlPool;

use sqlx::*;
use uuid::Uuid;

#[rocket::async_trait]
impl DBConnection for MySqlPool {
	async fn init(&self) -> Result<()> {
		query(CREATE_TABLE).execute(self).await?;
		Ok(())
	}
	async fn create_user(&self, id: Uuid, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
		query(INSERT_USER)
			.bind(id)
			.bind(email)
			.bind(hash)
			.bind(is_admin)
			.execute(self)
			.await?;
		Ok(())
	}
	async fn update_user(&self, user: &User) -> Result<(), Error> {
		query(UPDATE_USER)
			.bind(&user.email)
			.bind(&user.password)
			.bind(user.is_admin)
			.bind(user.id)
			.execute(self)
			.await?;

		Ok(())
	}
	async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), Error> {
		query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
		Ok(())
	}
	async fn delete_user_by_email(&self, email: &str) -> Result<(), Error> {
		query(REMOVE_BY_EMAIL).bind(email).execute(self).await?;
		Ok(())
	}
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Error> {
		let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;

		Ok(user)
	}
	async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
		let user = query_as(SELECT_BY_EMAIL).bind(email).fetch_one(self).await?;
		Ok(user)
	}
}
