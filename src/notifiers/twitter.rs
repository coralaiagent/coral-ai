use async_trait::async_trait;
use egg_mode::{Token, tweet::DraftTweet};
use super::{Notifier, NotificationMessage};

pub struct TwitterNotifier {
    token: Token,
}

impl TwitterNotifier {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        let token = Token::Access {
            consumer: egg_mode::KeyPair::new(consumer_key, consumer_secret),
            access: egg_mode::KeyPair::new(access_token, access_token_secret),
        };

        Self { token }
    }

    fn format_message(&self, message: &NotificationMessage) -> String {
        // Twitter has 280 character limit
        let mut tweet = format!(
            "🚀 {}\n\n{}\n\n#{:?} #{:?}",
            message.title,
            message.content,
            message.category,
            message.priority
        );

        if tweet.len() > 280 {
            tweet.truncate(277);
            tweet.push_str("...");
        }

        tweet
    }
}

#[async_trait]
impl Notifier for TwitterNotifier {
    async fn send_notification(&self, message: &NotificationMessage) -> Result<(), Box<dyn Error>> {
        let tweet_text = self.format_message(message);
        
        DraftTweet::new(tweet_text)
            .send(&self.token)
            .await?;

        Ok(())
    }
}
