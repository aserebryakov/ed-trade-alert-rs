extern crate reqwest;
extern crate json;

struct CommodityData {
    system: String,
    station: String,
    name: String,
    supply: u32,
    price: u32,
}

async fn get_page(number: u32) -> Result<String, reqwest::Error> {
    let request = format!("https://eddbapi.kodeblox.com/api/v4/stations?economyname=Refinery&statenames=Infrastructure Failure&page={}", number);
    reqwest::get(&request).await?.text().await
}

async fn get_market(id: String) -> Result<String, reqwest::Error> {
    let request = format!("https://www.edsm.net/api-system-v1/stations/market?marketId={}", id);
    reqwest::get(&request).await?.text().await
}

fn get_commodity_data(market: &json::JsonValue, commodity_name: String) -> Option<CommodityData> {
    match market["commodities"].members().filter(|x| x["name"].to_string() == commodity_name).next() {
        Some(data) => Some(CommodityData {
                system: market["name"].to_string(),
                station: market["sName"].to_string(),
                name: commodity_name,
                supply: data["stock"].as_u32().unwrap(),
                price: data["sellPrice"].as_u32().unwrap(),
            }),
        None => None
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = get_page(1).await?;
    let parsed = json::parse(&body).unwrap();
    let number_of_pages = parsed["pages"].as_u32().unwrap();
    println!("pages {}", number_of_pages);

    let mut entries = Vec::<CommodityData>::new();

    for page in 1..(number_of_pages + 1) {
        let body = get_page(page).await?;
        let stations = json::parse(&body).unwrap();
        for station in stations["docs"].members() {
            let market = get_market(station["ed_market_id"].to_string()).await?;
            let market_json = json::parse(&market).unwrap();
            if let Some(commodity_data) = get_commodity_data(&market_json, String::from("Silver")) {
                println!("Entry processed");
                entries.push(commodity_data);
            }
        }
    }

    entries.sort_by(|a, b| b.supply.cmp(&a.supply));

    for commodity in entries.iter().filter(|e| e.supply > 500) {
        println!("{:30} at {:30} {} price {:10} supply {:10}",
                commodity.system,
                commodity.station,
                commodity.name,
                commodity.price,
                commodity.supply);
    }

    Ok(())
}
