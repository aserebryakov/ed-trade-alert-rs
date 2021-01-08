extern crate reqwest;
extern crate json;

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://eddbapi.kodeblox.com/api/v4/stations?economyname=Refinery&statenames=Infrastructure Failure").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    let parsed = json::parse(&body).unwrap();

    Ok(())
}

//fn main() {}
