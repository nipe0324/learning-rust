use crate::app::article::model::Article;
use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use crate::error::AppError;
use diesel::pg::PgConnection;

// Get comments for an article
pub struct FetchArticleComments {
    pub slug: String,
}

pub fn fetch_article_comments(
    conn: &mut PgConnection,
    params: &FetchArticleComments,
) -> Result<Vec<(Comment, Profile)>, AppError> {
    let (article, _author) = Article::find_by_slug_with_author(conn, &params.slug)?;
    let list = Comment::find_comments_with_author_by_article_id(conn, &article.id)?;

    let comments_with_authors = list
        .iter()
        .map(|(comment, user)| {
            // TODO: fix N+1 query
            let profile = user.get_profile(conn, &user.id);

            // TODO: avoid copy
            (comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    Ok(comments_with_authors)
}
