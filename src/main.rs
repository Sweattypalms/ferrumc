use chrono::Local;
use colored::Colorize;
use ferrumc_utils::config;
use log::{error, info};
use std::io::Write;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".cyan(),
            };
            writeln!(
                buf,
                "[{} {}]: {}",
                Local::now().format("%H:%M:%S"),
                level,
                record.args()
            )
        })
        .filter_level(log::LevelFilter::max())
        .init();

    start().await;
}

async fn start() {
    info!(
        "{}",
        "                                                            ".purple()
    );
    info!(
        "{}",
        "   ______                                             _____ ".purple()
    );
    info!(
        "{}",
        "  |  ____|                                           / ____|".purple()
    );
    info!(
        "{}",
        "  | |__      ___   _ __   _ __   _   _   _ __ ___   | |     ".purple()
    );
    info!(
        "{}",
        "  |  __|    / _ \\ | '__| | '__| | | | | | '_ ` _ \\  | |     ".purple()
    );
    info!(
        "{}",
        "  | |      |  __/ | |    | |    | |_| | | | | | | | | |____ ".purple()
    );
    info!(
        "{}",
        "  |_|       \\___| |_|    |_|     \\__,_| |_| |_| |_|  \\_____|".purple()
    );
    info!(
        "{}",
        "                                                            ".purple()
    );

    info!("Starting FerrumC... ");
    info!("Minecraft version: {}", "1.17.1".green());
    let config = config::get_config();

    let server = ferrumc_net::start_server(&config.host, config.port).await;

    if let Err(err) = server {
        error!("{:?}", err);
    }
}