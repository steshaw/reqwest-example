async fn get_printer_fact() -> Result<String, reqwest::Error> {
    let resp = reqwest::get("https://printerfacts.cetacean.club/fact").await?;

    resp.text().await
}

#[tokio::main]
async fn main() {
    let f = get_printer_fact().await;
    match f {
        Ok(f) => println!("Printer fact: {}", f),
        Err(err) => println!("Argh: {}", err),
    }
}
