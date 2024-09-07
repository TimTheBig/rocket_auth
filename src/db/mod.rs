#[cfg(feature = "sqlx-postgres")]
mod postgres;

#[cfg(feature = "sqlx-mysql")]
mod mysql;
#[cfg(any(feature = "sqlx-sqlite", feature = "rusqlite"))]
mod sqlite;

#[cfg(feature = "tokio-postgres")]
mod tokio_postgres;

use uuid::Uuid;

use crate::prelude::*;

#[rocket::async_trait]
pub trait DBConnection: Send + Sync {
	async fn init(&self) -> Result<()>;
	async fn create_user(&self, id: Uuid, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
	async fn update_user(&self, user: &User) -> Result<(), Error>;
	async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), Error>;
	async fn delete_user_by_email(&self, email: &str) -> Result<(), Error>;
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Error>;
	async fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for std::sync::Arc<T> {
	async fn init(&self) -> Result<()> {
		T::init(self).await
	}
	async fn create_user(&self, id: Uuid, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
		T::create_user(self, id, email, hash, is_admin).await
	}
	async fn update_user(&self, user: &User) -> Result<(), Error> {
		T::update_user(self, user).await
	}
	async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), Error> {
		T::delete_user_by_id(self, user_id).await
	}
	async fn delete_user_by_email(&self, email: &str) -> Result<(), Error> {
		T::delete_user_by_email(self, email).await
	}
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Error> {
		T::get_user_by_id(self, user_id).await
	}
	async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
		T::get_user_by_email(self, email).await
	}
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for tokio::sync::Mutex<T> {
	async fn init(&self) -> Result<()> {
		self.init().await
	}
	async fn create_user(&self, id: Uuid, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
		self.lock().await.create_user(id, email, hash, is_admin).await
	}
	async fn update_user(&self, user: &User) -> Result<(), Error> {
		self.lock().await.update_user(user).await
	}
	async fn delete_user_by_id(&self, user_id: Uuid) -> Result<(), Error> {
		self.lock().await.delete_user_by_id(user_id).await
	}
	async fn delete_user_by_email(&self, email: &str) -> Result<(), Error> {
		self.lock().await.delete_user_by_email(email).await
	}
	async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Error> {
		self.lock().await.get_user_by_id(user_id).await
	}
	async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
		self.lock().await.get_user_by_email(email).await
	}
}
