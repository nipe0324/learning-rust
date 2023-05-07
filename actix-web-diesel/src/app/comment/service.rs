use super::model;
use crate::app::article::model::Article;
use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

// Get comments for an article
pub struct FetchArticleCommentsService {
    pub slug: String,
}

pub fn fetch_article_comments(
    conn: &mut PgConnection,
    params: &FetchArticleCommentsService,
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

// Create an article comment

pub struct CreateArticleCommentService {
    pub slug: String,
    pub body: String,
    pub author: User,
}

pub fn create_article_comment(
    conn: &mut PgConnection,
    params: &CreateArticleCommentService,
) -> Result<(Comment, Profile), AppError> {
    let (article, _) = Article::find_by_slug_with_author(conn, &params.slug)?;

    let comment = Comment::create(
        conn,
        &model::CreateComment {
            article_id: article.id,
            author_id: params.author.id,
            body: params.body.to_string(),
        },
    )?;
    let profile = params.author.get_profile(conn, &params.author.id);
    Ok((comment, profile))
}

// Delete an article comment

pub struct DeleteArticleCommentService {
    pub slug: String,
    pub comment_id: Uuid,
    pub author_id: Uuid,
}

pub fn delete_article_comment(
    conn: &mut PgConnection,
    params: &DeleteArticleCommentService,
) -> Result<(), AppError> {
    let (article, _) = Article::find_by_slug_with_author(conn, &params.slug)?;
    let comment =
        Comment::find_by_comment_id_and_author_id(conn, &params.comment_id, &params.author_id)?;

    Comment::delete(
        conn,
        &model::DeleteComment {
            article_id: article.id,
            author_id: params.author_id,
            comment_id: comment.id,
        },
    )?;

    Ok(())
}
