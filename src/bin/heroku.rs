use reacher_backend::{heroku::warp::create_api, setup_sentry};
use std::{env, net::IpAddr};

/// Run a HTTP server using warp.
///
/// # Panics
///
/// If at least one of the environment variables:
/// - RCH_HTTP_HOST
/// - RCH_PROXY_HOST
/// - RCH_PROXY_PORT
/// is malformed, then the program will panic.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	env_logger::init();
	let _guard = setup_sentry();

	let api = create_api();

	let host = env::var("RCH_HTTP_HOST")
		.unwrap_or_else(|_| "127.0.0.1".into())
		.parse::<IpAddr>()
		.expect("Env var RCH_HTTP_HOST is malformed.");
	let port = env::var("PORT")
		.map(|port| port.parse::<u16>().expect("Env var PORT is malformed."))
		.unwrap_or(8080);
	log::info!(target: "reacher", "Server is listening on {}:{}.", host, port);

	warp::serve(api).run((host, port)).await;
	Ok(())
}
