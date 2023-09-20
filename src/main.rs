extern crate ferrumc;

use anyhow::Result;
use ferrumc::server::Server;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;

#[tokio::main]
async fn main() -> Result<()> {
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
    // println!("{}", figure.to_string());
    let server = Server::new(25565).await?;
    server.run().await?;

    Ok(())
}