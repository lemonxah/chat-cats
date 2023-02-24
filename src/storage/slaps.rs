use mongodb::bson::doc;
use mongodb::{Database, Collection};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlapStats {
    pub author: i64,
    pub target: i64,
    pub count: i64,
}

pub async fn slap(author: i64, target: i64, db: Database) -> Result<SlapStats, mongodb::error::Error> {
    let collection: Collection<SlapStats> = db.collection("slaps");
    let filter = doc! { "author": author, "target": target };
    let update = doc! { "$inc": { "count": 1 } };
    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(ReturnDocument::After)
        .build();
    let result = collection
        .find_one_and_update(filter, update, options)
        .await
        .unwrap();
    Ok(result.unwrap())
}
