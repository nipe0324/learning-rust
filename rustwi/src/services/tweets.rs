use crate::repos::Tweets;
use crate::views::Home;

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list().await;
    Home {
        tweets: tweets.into_iter().map(|x| x.into()).collect(),
    }
}
