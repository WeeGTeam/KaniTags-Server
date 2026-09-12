use crate::Postgres;
use crate::dao::Dao;
use crate::models::collection::CollectionInsertRow;
use crate::models::collection_image::CollectionImageInsertRow;
use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::collection_database::CollectionDatabase;
use kani_domain_api_outgoing::database::error::{DbError, ReadDbError, ReadWriteDbError, WriteDbError};
use tracing::debug;

#[async_trait::async_trait]
impl CollectionDatabase for Postgres {
    async fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Collection, ReadDbError> {
        debug!("Loading collection by user and id: {}, {}", user.id, *collection_id);
        let (user_id, cid) = (user.id, *collection_id);
        let row = self
            .transaction::<_, _ , ReadDbError>(move |conn| conn.collection_dao().get_collection_by_user_and_id(user_id, cid))
            .await?;
        Ok(row.try_into()?)
    }

    async fn load_collection_by_user_and_name(&self, user: &User, collection_name: &CollectionName) -> Result<Collection, ReadDbError> {
        debug!("Loading collection by user and name: {}, {}", user.id, &**collection_name);
        let (user_id, name) = (user.id, collection_name.to_string());
        let row = self
            .transaction::<_, _ , ReadDbError>(move |conn| conn.collection_dao().get_collection_by_user_and_name(user_id, &name))
            .await?;
        Ok(row.try_into()?)
    }

    async fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, DbError> {
        debug!("Loading collections by user: {}", user.id);
        let user_id = user.id;
        let rows = self
            .transaction::<_, _ , DbError>(move |conn| conn.collection_dao().get_collections_by_user(user_id))
            .await?;
        Ok(rows.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?)
    }

    async fn create_collection(&self, user: &User, name: &CollectionName) -> Result<Collection, WriteDbError> {
        debug!("Creating collection for user: {}, name: {}", user.id, &**name);
        let insert = CollectionInsertRow {
            user_id: user.id,
            name: name.to_string(),
        };
        let row = self
            .transaction::<_, _, WriteDbError>(move |conn| conn.collection_dao().insert_collection(&insert))
            .await?;
        debug!("Created collection: {:?}", row.id);
        Ok(row.try_into()?)
    }

    async fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), ReadDbError> {
        debug!("Deleting collection for user: {}, id: {}", user.id, *collection_id);
        let (user_id, cid) = (user.id, *collection_id);
        self.transaction::<_, _, ReadDbError>(move |conn| conn.collection_dao().delete_collection(user_id, cid))
            .await?;
        Ok(())
    }

    async fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, ReadWriteDbError> {
        debug!("Adding {} images to collection for user: {}, collection_id: {}", image_ids.len(), user.id, *collection_id);
        let cid = *collection_id;
        let rows: Vec<CollectionImageInsertRow> = image_ids
            .iter()
            .map(|image_id| CollectionImageInsertRow {
                collection_id: cid,
                image_id: **image_id,
            })
            .collect();
        let inserted = self
            .transaction::<_, _, ReadWriteDbError>(move |conn| conn.collection_dao().insert_collection_images(&rows))
            .await?;
        debug!("Added {} images to collection: {}", inserted.len(), cid);
        Ok(inserted.len())
    }

    async fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, DbError> {
        debug!("Removing {} images from collection for user: {}, collection_id: {}", image_ids.len(), user.id, *collection_id);
        let cid = *collection_id;
        let ids: Vec<i64> = image_ids.iter().map(|image_id| **image_id).collect();
        let deleted = self
            .transaction::<_, _, DbError>(move |conn| conn.collection_dao().delete_collection_images(cid, &ids))
            .await?;
        debug!("Deleted {} images from collection: {}", deleted.len(), cid);
        Ok(deleted.len())
    }

    async fn get_collection_images(&self, user: &User, collection_id: CollectionId) -> Result<Vec<ImageId>, DbError> {
        debug!("Getting images from collection for user: {}, collection_id: {}", user.id, *collection_id);
        let images = self
            .transaction::<_, _, DbError>(move |conn| conn.collection_dao().get_all_collection_images(*collection_id))
            .await?;
        debug!("Finished gettimg collection images with {} results", images.len());
        Ok(images.into_iter().map(Into::into).collect())
    }
}
