use mongodb::bson::doc;
use mongodb::Database;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Whois {
    pub target: i64,
    pub message: String,
}

pub async fn is(target: i64, message: String, db: Database) -> Result<Whois, mongodb::error::Error> {
    let filter = doc! { "target": target };
    let update = doc! { "message": message };
    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(ReturnDocument::After)
        .build();
    let result = db.collection("whois")
        .find_one_and_update(filter, update, options)
        .await
        .unwrap();
    Ok(result.unwrap())
}

pub async fn whois(target: i64, db: Database) -> Result<Whois, mongodb::error::Error> {
    let filter = doc! { "target": target };
    let mut cursor = db.collection::<Whois>("whois")
        .find(filter, None)
        .await?;
    if let Ok(Some(x)) = cursor.try_next().await {
        Ok(x)
    } else {
        Ok(Whois { target, message: String::from("No whois message set") })
    }
}