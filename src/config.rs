use std::env;

pub struct Config {
    pub url: String,
    pub proxy: Option<String>,
    pub request_timeout: u64,
    pub tg_chat_id: String,
    pub tg_bot_token: String,
    pub check_duration_min: u64,
    pub check_duration_max: u64,
    pub if_get_stock_failed_send_msg: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let url = env::var("URL")?;
        let proxy = match env::var("PROXY") {
            Err(_) => None,
            Ok(v) => {
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            }
        };
        let request_timeout = env::var("REQUEST_TIMEOUT")?.parse()?;
        let tg_chat_id = env::var("TG_CHAT_ID")?;
        let tg_bot_token = env::var("TG_BOT_TOKEN")?;
        let check_duration_min = env::var("CHECK_DURATION_MIN")?.parse()?;
        let check_duration_max = env::var("CHECK_DURATION_MAX")?.parse()?;
        let if_get_stock_failed_send_msg = env::var("IF_GET_STOCK_FAILED_SEND_MSG")?.parse()?;

        Ok(Self {
            url,
            proxy,
            request_timeout,
            tg_chat_id,
            tg_bot_token,
            check_duration_min,
            check_duration_max,
            if_get_stock_failed_send_msg,
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
}
