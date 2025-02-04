use serde::Deserialize;

#[tokio::main]
pub async fn run() -> Result<(), reqwest::Error> {
    let url = "https://api.coincap.io/v2/assets";
    let url_id :&str = "https://api.coincap.io/v2/assets/xrp/history?interval=d1";


    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;


    for asset in response.data {
        println!("{} ({}) - Price: ${} - Symbol: {}", asset.name, asset.symbol, asset.price_usd, asset.symbol);
    }

    let response_id = reqwest::get(url_id).await?.json::<ApiResponseId>().await?;

    for assetId in response_id.data {
        println!("- Price: ${} - time: {} - date: {}", assetId.price_usd, assetId.time, assetId.date);
    }
    Ok(())
}


#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Asset>,
}


#[derive(Debug, Deserialize)]
struct Asset {
    symbol: String,
    name: String,
    #[serde(rename = "priceUsd")]
    price_usd: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponseId {
    data: Vec<AssetId>,
}

#[derive(Debug, Deserialize)]
struct AssetId {

    #[serde(rename = "priceUsd")]
    price_usd: String,
    time: u128,
    date: String,
}