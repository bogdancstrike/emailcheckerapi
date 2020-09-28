use std::{env, fmt};

#[derive(Debug)]
pub struct IncorrectSaasifySecret {}

impl Default for IncorrectSaasifySecret {
	fn default() -> Self {
		IncorrectSaasifySecret {}
	}
}

impl IncorrectSaasifySecret {
	pub fn new() -> Self {
		Default::default()
	}
}

impl fmt::Display for IncorrectSaasifySecret {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl warp::reject::Reject for IncorrectSaasifySecret {}

pub const SAASIFY_SECRET_HEADER: &str = "x-saasify-proxy-secret";

/// Get the server's Saasify secret, either from ENV, or use the fallback dev
/// secret.
pub fn get_saasify_secret() -> String {
	env::var("RCH_SAASIFY_SECRET").unwrap_or_else(|_| "reacher_dev_secret".into())
}
