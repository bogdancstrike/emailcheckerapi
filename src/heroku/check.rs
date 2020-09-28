use crate::{sentry_util, ReacherInput, ReacherOutput, RetryOption};
use check_if_email_exists::check_email as ciee_check_email;

use std::time::Instant;

/// The main `check_email` function, on Heroku.
pub async fn check_email_heroku(body: ReacherInput) -> ReacherOutput {
	// Run `ciee_check_email` with retries if necessary. Also measure the
	// verification time.
	let now = Instant::now();
	let result = ciee_check_email(&body.into())
		.await
		.pop()
		.expect("The input has one element, so does the output. qed.");
	let result = ReacherOutput::Ciee(Box::new(result));

	// This will only log the Heroku verification.
	if let ReacherOutput::Ciee(value) = &result {
		sentry_util::info(
			format!("is_reachable={:?}", value.is_reachable),
			RetryOption::Heroku,
			now.elapsed().as_millis(),
		);
	}

	result
}
