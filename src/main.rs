use reqwest;
use std::error::Error;
use std::process;
use std::time::Duration;
use structopt::StructOpt;
use tokio;

#[derive(StructOpt)]
struct Cli {
    /// Интервал в секундах
    interval: u64,
    /// URL для проверки
    url: String,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let url = match reqwest::Url::parse(&args.url) {
        Ok(u) => u,
        Err(_) => {
            println!("URL parsing error");
            return Ok(());
        }
    };

    let client = reqwest::Client::new();

    loop {
        let response = client.get(url.as_str()).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Checking '{}'. Result: OK(200)", url);
                } else {
                    println!("Checking '{}'. Result: ERR({})", url, resp.status().as_u16());
                }
            }
            Err(_) => {
                println!("URL parsing error");
                break;
            }
        }

        tokio::time::sleep(Duration::from_secs(args.interval)).await;
    }

    Ok(())
}
