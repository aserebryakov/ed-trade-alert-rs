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

fn print_silver_data(market: json::JsonValue) {
    for commodity in market["commodities"].members() {
        if (commodity["name"].to_string() == "Silver") {
            println!("system {:20} station {:20} {} price {:20} stock {:20}", market["name"], market["sName"], commodity["name"], commodity["sellPrice"], commodity["stock"]);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = get_page(1).await?;
    let parsed = json::parse(&body).unwrap();
    let number_of_pages = parsed["pages"].as_u32().unwrap();
    println!("pages {}", number_of_pages);

    for page in 2..number_of_pages {
        let body = get_page(page).await?;
        let parsed = json::parse(&body).unwrap();
        for station in parsed["docs"].members() {
            let market = get_market(station["ed_market_id"].to_string()).await?;
            let market_json = json::parse(&market).unwrap();
            print_silver_data(market_json);
        }
    }

    Ok(())
}
