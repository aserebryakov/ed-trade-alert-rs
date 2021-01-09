#![feature(generators, generator_trait)]

extern crate reqwest;
extern crate json;

async fn get_page(number: u32) -> Result<String, reqwest::Error> {
    let request = format!("https://eddbapi.kodeblox.com/api/v4/stations?economyname=Refinery&statenames=Infrastructure Failure&page={}", number);
    reqwest::get(&request).await?.text().await
}

async fn get_market(id: String) -> Result<String, reqwest::Error> {
    let request = format!("https://www.edsm.net/api-system-v1/stations/market?marketId={}", id);
    reqwest::get(&request).await?.text().await
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = get_page(1).await?;

    let parsed = json::parse(&body).unwrap();
    
    for station in parsed["docs"].members() {
        println!("id = {}", station["ed_market_id"].to_string());
        let market = get_market(station["ed_market_id"].to_string()).await?;
        let market_json = json::parse(&market).unwrap();
        println!("{}", market_json.pretty(4));
    }

    Ok(())
}
