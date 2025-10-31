use anyhow::anyhow;
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
    let caps = match reg.captures(&body) {
        Some(v) => v,
        None => return Err(anyhow!("无法匹配正则")),
    };
    // tracing::debug!("caps: {caps:?}");

    let stock = match caps.get(1) {
        Some(v) => v.as_str(),
        None => return Err(anyhow!("无法获取正则匹配项")),
    };
    // tracing::debug!("stock: {stock}");

    stock.parse::<StockNum>().map_err(|e| e.into())
}
