mod bot;
mod helper;

use bot::Bot;

#[tokio::main]
async fn main() {
    let bot = Bot::new(5, "ALP30p3AW/IK4f4FdMb2bzQ/eGZWwhOFT/iNaRfTMdST");

    match bot {
        Err(err) => println!("{}", err.to_string()),
        Ok(bot) => bot.start().await,
    };
}
