use crate::repos::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, DateTime, NaiveDate, NaiveTime, NaiveDateTime};

    use crate::entities::Tweet;
    use crate::repos::MockTweets;

    fn tweet(id: i32) -> Tweet {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(date, time);

        Tweet::new(
            id,
            format!("message{}", id),
            DateTime::from_utc(naive_datetime, Utc)
        )
    }

    #[tokio::test]
    async fn test_list_tweets() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![tweet(2), tweet(1)]);

        let result = super::list_tweets(&tweets).await;
        assert_eq!(result.tweets.len(), 2);
        let result0 = result.tweets.get(0).unwrap();
        assert_eq!(result0.message, "message2");
        assert_eq!(result0.posted_at, "2020/01/01 00:00");
    }

    #[tokio::test]
    async fn test_list_tweets_empty() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![]);

        let result = super::list_tweets(&tweets).await;
        assert_eq!(result.tweets.len(), 0);
    }
}
