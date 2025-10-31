use lazy_static::lazy_static;
use regex::Regex;

use crate::{Config, client};

lazy_static! {
    static ref STOCK_PAT: &'static str = r#"<p class="card-text">库存： (\d+?)</p>"#;
    static ref STOCK_REG: Result<Regex, regex::Error> = Regex::new(*STOCK_PAT);
}

pub type StockNum = i32;

pub async fn get_in_stock(cfg: &Config) -> anyhow::Result<StockNum> {
    let cli = client::with_cfg(&cfg)?;

    let body = cli.get(&cfg.url).send().await?.text().await?;
    // tracing::debug!("body: {body}");

    let reg = match &*STOCK_REG {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };
    let caps = reg.captures(&body).unwrap();
    // tracing::debug!("caps: {caps:?}");

    let stock = caps.get(1).unwrap().as_str();
    // tracing::debug!("stock: {stock}");

    stock.parse::<StockNum>().map_err(|e| e.into())
}
