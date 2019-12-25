use async_trait::async_trait;
use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{Update, UpdateKind},
    Api, Config, ExecuteError, UpdateHandler,
};

struct Handler {
    api: Api,
}

#[async_trait]
impl UpdateHandler for Handler {
    type Error = ExecuteError;

    async fn handle(&mut self, update: Update) -> Result<(), Self::Error> {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.get_text() {
                let api = self.api.clone();
                let chat_id = message.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                api.execute(method).await?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let proxy = env::var("TGRS_PROXY").ok();
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}
