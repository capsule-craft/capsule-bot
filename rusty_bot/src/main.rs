mod bot;
pub mod client;
mod config;
mod helper;
mod strategy;

use bot::Bot;

#[tokio::main]
async fn main() {
    match Bot::new(5, "ALP30p3AW/IK4f4FdMb2bzQ/eGZWwhOFT/iNaRfTMdST") {
        Ok(bot) => {
            if let Err(err) = bot.start().await {
                println!("{}", err.to_string())
            }
        }
        Err(err) => println!("{}", err.to_string()),
    }
}
