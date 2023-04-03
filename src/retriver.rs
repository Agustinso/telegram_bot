use crate::providers;

pub async fn retrive() -> Result<(), Box<dyn std::error::Error>> {
    let smn_now = providers::smn::now().await?;
    providers::smn::forecast().await?;
    dbg!(smn_now);
    Ok(())
}
