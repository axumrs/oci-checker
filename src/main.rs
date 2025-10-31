use std::sync::{Arc, Mutex};

use chrono::{FixedOffset, Local, Utc};
use dotenv::dotenv;
use oci_checker::{Config, oci, tg_bot};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

enum BotMessage {
    Stock(oci::StockNum),
    GetStockFailed(String),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let last_stock_num = Arc::new(Mutex::<oci::StockNum>::new(-1));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Arc::new(Config::from_env()?);

    loop {
        tracing::debug!("🔧 开始工作");
        let stock = match oci::get_in_stock(&cfg).await {
            Ok(v) => v,
            Err(e) => {
                if cfg.if_get_stock_failed_send_msg {
                    tokio::spawn(send_bot_msg(
                        cfg.clone(),
                        build_msg(BotMessage::GetStockFailed(e.to_string())),
                    ));
                }
                tracing::error!("❌ 获取库存失败：{e}");
                sleep(cfg.check_duration()).await;
                continue;
            }
        };

        // 读取
        {
            let lsn = match last_stock_num.lock() {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("❌ 读取共享状态失败：{e}");
                    sleep(cfg.check_duration()).await;
                    continue;
                }
            };

            if *lsn == stock {
                tracing::info!("🌙 库存未变");
                sleep(cfg.check_duration()).await;
                continue;
            }
        }

        // 写入
        {
            match last_stock_num.lock() {
                Ok(mut v) => *v = stock,
                Err(e) => {
                    tracing::error!("❌ 写入共享状态失败：{e}");
                    sleep(cfg.check_duration()).await;
                    continue;
                }
            };
        }

        if stock < cfg.skip_notify_stock_num {
            tracing::info!("☀️ 库存不足, 稍后重试");
            sleep(cfg.check_duration()).await;
            continue;
        }
        tokio::spawn(send_bot_msg(
            cfg.clone(),
            build_msg(BotMessage::Stock(stock)),
        ));

        tracing::debug!("✅ 本次工作完成");
        sleep(cfg.check_duration()).await;
    }
}

fn build_msg(bot_msg: BotMessage) -> String {
    let now = match FixedOffset::east_opt(8 * 3600) {
        Some(v) => Utc::now().with_timezone(&v),
        None => Local::now().fixed_offset(),
    }
    .format("%Y-%m-%d %H:%M:%S")
    .to_string();

    let msg = match bot_msg {
        BotMessage::GetStockFailed(e) => format!("❌ 获取库存信息失败：{e}"),
        BotMessage::Stock(stock) => {
            if stock <= 0 {
                "无库存".to_string()
            } else {
                format!("库存：{stock}")
            }
        }
    };

    format!("🎉 OCI 库存通知 🎉\n{msg}\n{now}")
}

fn sleep(duration: u64) -> tokio::time::Sleep {
    tokio::time::sleep(tokio::time::Duration::from_secs(duration))
}

async fn send_bot_msg(cfg: Arc<Config>, msg: String) {
    if let Err(e) = tg_bot::send_msg(&*cfg, &msg).await {
        tracing::error!("❌ 推送TG消息失败：{e}");
        sleep(cfg.check_duration()).await;
    }
}
