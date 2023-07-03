//Incomplete
use reqwest::ClientBuilder;
use std::error::Error;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let timeout_duration = Duration::from_secs(1);
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .timeout(timeout_duration)
        .build()?;
    let mut i=0;
    for i in 1..254{
        let url=format!("http://123.63.2.{i}/");
        //println!("{}",&url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        println!("Status code: {}", status);
    }
    Ok(())
}
