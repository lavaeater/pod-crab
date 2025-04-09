use entities::{episode, import_row, member, member::Entity as Member};
use entities::{post, post::Entity as Post};

use sea_orm::prelude::Uuid;
use sea_orm::*;
use sha2::{Digest, Sha256};

pub struct Mutation;

pub trait RecordHash {
    fn hash(&self) -> String;
}

impl RecordHash for member::Model {
    fn hash(&self) -> String {
        // Normalize data by trimming and converting to lowercase
        let normalized = format!(
            "{}:{}:{}:{}:{}",
            self.first_name.trim().to_lowercase(),
            self.last_name.trim().to_lowercase(),
            self.birth_date.to_string().trim(),
            self.mobile_phone.trim().replace(['-', ' ', '(', ')', '+'], ""),
            self.email.trim().to_lowercase()
        );

        // Calculate SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(normalized);
        let result = hasher.finalize();

        // Convert to hex string
        format!("{:x}", result)
    }
}

impl RecordHash for import_row::Model {
    fn hash(&self) -> String {
        let normalized = format!(
            "{}:{}:{}:{}:{}",
            self.first_name.trim().to_lowercase(),
            self.last_name.trim().to_lowercase(),
            self.birth_date.to_string().trim(),
            self.mobile_phone.trim().replace(['-', ' ', '(', ')', '+'], ""),
            self.email.trim().to_lowercase()
        );

        // Calculate SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(normalized);
        let result = hasher.finalize();

        // Convert to hex string
        format!("{:x}", result)
        
    }
}

fn calculate_member_hash(
    first_name: &str,
    last_name: &str,
    birthdate: &str,
    phone_number: &str,
    email: &str
) -> String {
    
}

impl Mutation {
    pub async fn create_episode(
        db: &DatabaseConnection,
        form_data: episode::Model,
    ) -> Result<episode::ActiveModel, DbErr> {
        form_data.into_active_model().save(db).await
    }
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
            hash: Set(form_data.hash.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_member_by_id(
        db: &DbConn,
        id: Uuid,
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
            hash: Default::default(),
        }
        .update(db)
        .await
    }

    pub async fn delete_member(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
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
        id: Uuid,
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

    pub async fn delete_post(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
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
