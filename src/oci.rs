use crate::{Config, client};

const STOCK_PAT: &str = r#"<p class="card-text">库存： (\d+?)</p>"#;

pub async fn get_in_stock(cfg: &Config) -> anyhow::Result<u32> {
    let cli = client::with_cfg(&cfg)?;

    let body = cli.get(&cfg.url).send().await?.text().await?;
    // tracing::debug!("body: {body}");

    let reg = regex::Regex::new(STOCK_PAT)?;
    let caps = reg.captures(&body).unwrap();
    // tracing::debug!("caps: {caps:?}");

    let stock = caps.get(1).unwrap().as_str();
    // tracing::debug!("stock: {stock}");

    stock.parse::<u32>().map_err(|e| e.into())
}
