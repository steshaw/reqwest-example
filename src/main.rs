use eyre::{Result, WrapErr};

async fn get(url: &str) -> Result<String> {
    let resp = reqwest::get(url).await?;

    resp.text().await.wrap_err(format!("Failed to get url={}", url))
}

async fn get_printer_fact() -> Result<String> {
    get("https://printerfacts.cetacean.club/fact").await
}

#[tokio::main]
async fn main() {
    let f = get_printer_fact().await;
    match f {
        Ok(f) => println!("Printer fact: {}", f),
        Err(err) => println!("Argh: {}", err),
    }

    let broken = get("broken").await;
    println!("broken = {:?}", broken);

    let broken2 = get("http://127.0.0.1/argh").await;
    println!("broken2 = {:?}", broken2);
}
