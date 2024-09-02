use crate::forms::is_secure;
use crate::session::AuthKey;
use crate::user::rand_string;

mod test_forms {
    use super::*;

	/// Test the is_secure function.\
	/// It checks if the password is at least 8 characters long, has at least one uppercase character, one lowercase character, and one number.
	#[test]
	fn test_is_secure() {
		// test correct
		assert_eq!(is_secure("eXample1").is_ok(), true);
		// test is_long error
		assert_eq!(is_secure("1e7Xr"), Err(validator::ValidationError::new("The password must be at least 8 characters long.\n")));
		// test has_uppercase error
		assert_eq!(is_secure("example1"), Err(validator::ValidationError::new("The password must include least one uppercase character.\n")));
		// test has_lowercase error
		assert_eq!(is_secure("EXAMPLE1"), Err(validator::ValidationError::new("The password must include least one uppercase character.\n")));
		// test has_number error
		assert_eq!(is_secure("Examplee"), Err(validator::ValidationError::new("The password has to contain at least one digit.\n")));
	}
}

mod test_session {
    use std::any::Any;

    use super::*;

	#[test]
	fn test_auth_key() {
		let key = AuthKey::try_from(rand_string(64));
		assert!(key.is_ok());
		assert_eq!(key.unwrap().type_id(), AuthKey::from("").type_id());
	}
}

mod test_user {
    use super::*;

	#[test]
	fn test_rand_string() {
		assert_eq!(rand_string(10).len(), 10);
		assert_eq!(rand_string(20).len(), 20);
		assert_eq!(rand_string(30).len(), 30);
		assert_ne!(rand_string(10), rand_string(10));
	}
}