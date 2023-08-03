#![feature(result_option_inspect)]
use std::env;

#[tokio::main]
async fn main() {
    print!("Enter appid: ");
    let mut appid: String = text_io::try_read!("{}\n").expect("Failed to read appid");
    appid = appid.replace("\r", "").replace("\n", "").replace("\t", "");

    env::set_var("SteamAppId", appid);

    if let Ok((client, single)) =
        steamworks::Client::init().inspect_err(|e| eprintln!("Cannot connect to steam: {}", e))
    {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
