use startup::Application;

mod api;
mod context;
mod error;
mod server;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let application = Application::build().await?;
    application.run_untill_stopped().await
}
