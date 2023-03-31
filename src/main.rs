mod retriver;
use retriver::retrive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    retrive().await?;
    Ok(())
}
