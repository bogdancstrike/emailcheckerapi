use lambda_http::{handler, lambda};
use reacher_backend::serverless::lambda::lambda_check_email_handler;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
	env_logger::init();
	lambda::run(handler(lambda_check_email_handler)).await?;
	Ok(())
}
