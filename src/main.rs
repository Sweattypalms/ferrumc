extern crate ferrumc;

use anyhow::Result;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use tokio::net::TcpListener;
use tokio::spawn;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("FerrumC").unwrap();
    let mut red = 255;
    figure.to_string().split("\n").for_each(|line| {
        red -= 25;
        println!(
            "{}",
            line.color(owo_colors::Rgb {
                0: red,
                1: 105,
                2: 180
            })
        );
    });

    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    info!("TCP listener created and bound to port 25565");
    loop {
        // server loop
        let (socket, _) = listener.accept().await?;
        spawn(ferrumc_net::handle_connection(socket));
    }
}