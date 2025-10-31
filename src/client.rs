use std::time;

use reqwest::{Client, Proxy};

use crate::{Config, user_agent};

pub fn new(timeout: u64, prx: Option<&str>) -> anyhow::Result<Client> {
    let ua = user_agent::get();
    tracing::debug!("用户代理：{ua}");

    let cb = Client::builder()
        .connect_timeout(time::Duration::from_secs(timeout))
        .user_agent(ua);
    let cb = if let Some(prx) = prx {
        tracing::debug!("代理：{prx}");
        cb.proxy(proxy(&prx)?)
    } else {
        cb
    };

    cb.build().map_err(|e| e.into())
}

pub fn with_cfg(cfg: &Config) -> anyhow::Result<Client> {
    new(cfg.request_timeout, cfg.proxy())
}

fn proxy(proxy: &str) -> anyhow::Result<Proxy> {
    Proxy::all(proxy).map_err(|e| e.into())
}
