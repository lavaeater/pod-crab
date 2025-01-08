use sea_orm::*;
use sea_orm::prelude::Uuid;
use entities::{member, member::Entity as Member};
use entities::{post, post::Entity as Post};



pub struct Query;

impl Query {
    pub async fn find_member_by_id(db: &DbConn, id: Uuid) -> Result<Option<member::Model>, DbErr> {
        Member::find_by_id(id).one(db).await
    }

    /// If ok, returns (member models, num pages).
    pub async fn find_members_in_page(
        db: &DbConn,
        page: u64,
        members_per_page: u64,
    ) -> Result<(Vec<member::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Member::find()
            .order_by_asc(member::Column::Id)
            .paginate(db, members_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated members
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
    pub async fn find_post_by_id(db: &DbConn, id: Uuid) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_posts_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
