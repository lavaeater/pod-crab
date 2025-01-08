use crate::entity::{member, member::Entity as Member};
use crate::entity::{post, post::Entity as Post};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_member(
        db: &DbConn,
        form_data: member::Model,
    ) -> Result<member::ActiveModel, DbErr> {
        member::ActiveModel {
            first_name: Set(form_data.first_name.to_owned()),
            last_name: Set(form_data.last_name.to_owned()),
            email: Set(form_data.email.to_owned()),
            mobile_phone: Set(form_data.mobile_phone.to_owned()),
            birth_date: Set(form_data.birth_date.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_member_by_id(
        db: &DbConn,
        id: i32,
        form_data: member::Model,
    ) -> Result<member::Model, DbErr> {
        let member: member::ActiveModel = Member::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find member.".to_owned()))
            .map(Into::into)?;

        member::ActiveModel {
            id: member.id,
            first_name: Set(form_data.first_name.to_owned()),
            last_name: Set(form_data.last_name.to_owned()),
            email: Set(form_data.email.to_owned()),
            mobile_phone: Set(form_data.mobile_phone.to_owned()),
            birth_date: Set(form_data.birth_date.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_member(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let member: member::ActiveModel = Member::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find member.".to_owned()))
            .map(Into::into)?;

        member.delete(db).await
    }

    #[allow(dead_code)]
    pub async fn delete_all_members(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Member::delete_many().exec(db).await
    }

    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    #[allow(dead_code)]
    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }
}
