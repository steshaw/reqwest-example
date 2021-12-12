async fn get_printer_fact() -> Result<String, reqwest::Error> {
    let resp = reqwest::get("https://printerfacts.cetacean.club/fact").await?;

    resp.text().await
}

#[tokio::main]
async fn main() {
    get_printer_fact().await.into_iter().for_each(|f| {
        println!("your printer fact is: {}", f);
    });
}
