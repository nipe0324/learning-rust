use crate::entities::Tweet;
use crate::repos::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
    }
}

pub async fn create_tweet(repo: &impl Tweets, message: &str) {
    let new_tweet = Tweet::create(message);
    repo.create(&new_tweet).await;
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    let tweet = repo.find(id).await;
    match tweet {
        Some(mut tweet) => {
            tweet.delete();
            repo.delete(&tweet).await;
        }
        None => {
            println!("Tweet not found: {}", id);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    use crate::entities::Tweet;
    use crate::repos::MockTweets;

    fn tweet(id: i32) -> Tweet {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(date, time);

        Tweet::new(
            id,
            format!("message{}", id),
            DateTime::from_utc(naive_datetime, Utc),
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

    #[tokio::test]
    async fn test_delete_tweet() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| Some(tweet(1)));
        tweets
            .expect_delete()
            .withf(|e| e.id() == Some(1) && e.is_deleted())
            .once()
            .return_const(());

        super::delete_tweet(&tweets, 1).await;
    }

    #[tokio::test]
    async fn test_delete_tweet_not_found() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| None);
        tweets.expect_delete().never();

        super::delete_tweet(&tweets, 1).await;
    }
}
