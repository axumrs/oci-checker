use std::collections::HashMap;

use crate::{Config, client};

pub async fn send_msg(cfg: &Config, msg: &str) -> anyhow::Result<()> {
    let mut message = HashMap::new();
    message.insert("chat_id", cfg.tg_chat_id.as_str());
    message.insert("text", msg);

    let _resp = client::with_cfg(cfg)?
        .post(cfg.tg_bot_send_message_url().as_str())
        .form(&message)
        .send()
        .await?
        .text()
        .await?;

    tracing::debug!("{_resp}");

    Ok(())
}
