use acid::configuration::get_configuration;
use acid::startup::Application;
use acid::telemetry::{get_subscriber, init_subscriber};
use dotenvy::dotenv;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let subscriber = get_subscriber("api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
