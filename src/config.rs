use std::env;

use crate::oci;

pub struct Config {
    pub url: String,
    pub proxies: Option<String>,
    pub request_timeout: u64,
    pub tg_chat_id: String,
    pub tg_bot_token: String,
    pub check_duration_min: u64,
    pub check_duration_max: u64,
    pub if_get_stock_failed_send_msg: bool,
    pub skip_notify_stock_num: oci::StockNum,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let url = env::var("URL").unwrap_or("https://oci.ee/cart?fid=5".into());
        let proxies = match env::var("PROXIES") {
            Err(_) => None,
            Ok(v) => {
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            }
        };
        let request_timeout = env::var("REQUEST_TIMEOUT").unwrap_or("10".into()).parse()?;
        let tg_chat_id = env::var("TG_CHAT_ID").expect("请指定 `TG_CHAT_ID` 环境变量");
        let tg_bot_token = env::var("TG_BOT_TOKEN").expect("请指定 `TG_BOT_TOKEN` 环境变量");
        let check_duration_min = env::var("CHECK_DURATION_MIN")
            .unwrap_or("6".into())
            .parse()
            .unwrap_or(6);
        let check_duration_max = env::var("CHECK_DURATION_MAX")
            .unwrap_or("12".into())
            .parse()
            .unwrap_or(12);
        let if_get_stock_failed_send_msg = env::var("IF_GET_STOCK_FAILED_SEND_MSG")
            .unwrap_or("true".into())
            .parse()
            .unwrap_or(true);
        let skip_notify_stock_num = env::var("SKIP_NOTIFY_STOCK_NUM")
            .unwrap_or("1".into())
            .parse()
            .unwrap_or(1);

        Ok(Self {
            url,
            proxies,
            request_timeout,
            tg_chat_id,
            tg_bot_token,
            check_duration_min,
            check_duration_max,
            if_get_stock_failed_send_msg,
            skip_notify_stock_num,
        })
    }

    pub fn tg_bot_send_message_url(&self) -> String {
        format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.tg_bot_token
        )
    }

    pub fn check_duration(&self) -> u64 {
        rand::random_range(self.check_duration_min..=self.check_duration_max)
    }

    pub fn proxies(&self) -> Option<Vec<&str>> {
        if let Some(proxies_str) = &self.proxies {
            return Some(proxies_str.split(",").collect());
        }
        None
    }

    pub fn proxy(&self) -> Option<&str> {
        if let Some(proxies) = self.proxies() {
            return proxies.get(rand::random_range(0..proxies.len())).copied();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use super::*;

    #[test]
    fn test_get_proxy() {
        dotenv().ok();

        let cfg = Config::from_env().unwrap();
        let proxy = cfg.proxy();
        println!("{proxy:?}");
    }
}
