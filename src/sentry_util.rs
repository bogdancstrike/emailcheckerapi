use super::RetryOption;
use sentry::protocol::{Event, Level, Value};
use std::{collections::BTreeMap, env};

pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Helper to add provider information (Serverless, Heroku) to Sentry events.
fn add_provider_info(extra: &mut BTreeMap<String, Value>) {
	if let Ok(reacher_provider) = env::var("RCH_PROVIDER") {
		extra.insert("RCH_PROVIDER".into(), reacher_provider.into());
	}
}

/// Helper function to send an Info event to Sentry.
pub fn info(message: String, option: RetryOption, duration: u128) {
	log::info!("Sending to Sentry: {}", message);

	let mut extra = BTreeMap::new();

	add_provider_info(&mut extra);
	extra.insert("duration".into(), duration.to_string().into());
	extra.insert("proxy_option".into(), option.to_string().into());

	sentry::capture_event(Event {
		extra,
		level: Level::Info,
		message: Some(message),
		// FIXME It seams that this doesn't work on Sentry, so I added it in
		// the `extra` field above too.
		release: Some(CARGO_PKG_VERSION.into()),
		..Default::default()
	});
}

/// Helper function to send an Error event to Sentry.
pub fn error(message: String, result: Option<&str>, option: RetryOption) {
	log::debug!("Sending to Sentry: {}", message);
	let mut extra = BTreeMap::new();

	add_provider_info(&mut extra);
	if let Some(result) = result {
		extra.insert("CheckEmailOutput".into(), result.into());
	}
	extra.insert("proxy_option".into(), option.to_string().into());

	sentry::capture_event(Event {
		extra,
		level: Level::Error,
		message: Some(message),
		// FIXME It seams that this doesn't work on Sentry, so I added it in
		// the `extra` field above too.
		release: Some(CARGO_PKG_VERSION.into()),
		..Default::default()
	});
}
