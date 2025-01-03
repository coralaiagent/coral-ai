use async_trait::async_trait;
use teloxide::{prelude::*, types::ParseMode};
use super::{Notifier, NotificationMessage};

pub struct TelegramNotifier {
    bot: Bot,
    chat_id: ChatId,
}

impl TelegramNotifier {
    pub fn new(token: String, chat_id: i64) -> Self {
        Self {
            bot: Bot::new(token),
            chat_id: ChatId(chat_id),
        }
    }

    fn format_message(&self, message: &NotificationMessage) -> String {
        format!(
            "*{}*\n\n\
            {}\n\n\
            Priority: {:?}\n\
            Category: {:?}\n\
            Time: {}",
            message.title,
            message.content,
            message.priority,
            message.category,
            message.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

#[async_trait]
impl Notifier for TelegramNotifier {
    async fn send_notification(&self, message: &NotificationMessage) -> Result<(), Box<dyn Error>> {
        let formatted_message = self.format_message(message);
        
        self.bot
            .send_message(self.chat_id, formatted_message)
            .parse_mode(ParseMode::MarkdownV2)
            .disable_web_page_preview(true)
            .await?;

        Ok(())
    }
}
