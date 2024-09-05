use super::SessionManager;
use crate::prelude::*;

use redis::{Client, Commands};
use uuid::Uuid;

const YEAR_IN_SECS: usize = 365 * 60 * 60 * 24;

impl SessionManager for Client {
	#[throws(Error)]
	fn insert(&self, id: Uuid, key: String) {
		let mut cnn = self.get_connection()?;
		cnn.set_ex(id.as_bytes(), key, YEAR_IN_SECS)?;
	}
	#[throws(Error)]
	fn insert_for(&self, id: Uuid, key: String, time: Duration) {
		let mut cnn = self.get_connection()?;
		cnn.set_ex(id.as_bytes(), key, time.as_secs() as usize)?;
	}
	#[throws(Error)]
	fn remove(&self, id: Uuid) {
		let mut cnn = self.get_connection()?;
		cnn.del(id.as_bytes())?;
	}
	#[throws(as Option)]
	fn get(&self, id: Uuid) -> String {
		let mut cnn = self.get_connection().ok()?;
		let key = cnn.get(id.as_bytes()).ok()?;
		key
	}
	#[throws(Error)]
	fn clear_all(&self) {
		let mut cnn = self.get_connection()?;
		redis::Cmd::new().arg("FLUSHDB").execute(&mut cnn);
	}
	#[throws(Error)]
	fn clear_expired(&self) {}
}
