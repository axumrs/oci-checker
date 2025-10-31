use std::time;

use reqwest::{Client, Proxy};

use crate::Config;

pub fn new(user_agent: &str, timeout: u64, prx: &Option<String>) -> anyhow::Result<Client> {
    let cb = Client::builder()
        .connect_timeout(time::Duration::from_secs(timeout))
        .user_agent(user_agent);
    let cb = if let Some(prx) = prx {
        cb.proxy(proxy(&prx)?)
    } else {
        cb
    };

    cb.build().map_err(|e| e.into())
}

pub fn with_cfg(cfg: &Config) -> anyhow::Result<Client> {
    new(&cfg.user_agent, cfg.request_timeout, &cfg.proxy)
}

fn proxy(proxy: &str) -> anyhow::Result<Proxy> {
    Proxy::all(proxy).map_err(|e| e.into())
}
