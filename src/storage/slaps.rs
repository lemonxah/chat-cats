use mongodb::bson::{doc, Document};
use mongodb::{Database, Collection};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlapStats {
    pub author: i64,
    pub target: i64,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlapCount {
    pub author: i64,
    pub count: i32,
}

pub async fn slap(author: i64, target: i64, db: Database) -> Result<SlapStats, mongodb::error::Error> {
    let filter = doc! { "author": author, "target": target };
    let update = doc! { "$inc": { "count": 1 } };
    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(ReturnDocument::After)
        .build();
    let result = db.collection("slaps")
        .find_one_and_update(filter, update, options)
        .await
        .unwrap();
    Ok(result.unwrap())
}

pub async fn given(author: i64, db: Database) -> Result<i64, mongodb::error::Error> {
    let collection: Collection<Document> = db.collection("slaps");
    let pipeline = vec![
        doc! { "$match": { "author": author } },
        doc! { "$group": { "_id": 0, "total": { "$sum": "$count" } } },
    ];
    let mut cursor = collection.aggregate(pipeline, None).await?;
    if let Some(totals) = cursor.next().await {
        let total = totals.unwrap().get_i32("total");
        return Ok(total.unwrap() as i64);
    }
    Ok(0i64)
}

pub async fn received(author: i64, db: Database) -> Result<i64, mongodb::error::Error> {
    let collection: Collection<Document> = db.collection("slaps");
    let pipeline = vec![
        doc! { "$match": { "target": author } },
        doc! { "$group": { "_id": 0, "total": { "$sum": "$count" } } },
    ];
    let mut cursor = collection.aggregate(pipeline, None).await?;
    if let Some(totals) = cursor.next().await {
        let total = totals.unwrap().get_i32("total");
        return Ok(total.unwrap() as i64);
    }
    Ok(0i64)
}

pub async fn top3_slappers(db: Database) -> Result<Vec<SlapCount>, mongodb::error::Error> {
    let collection: Collection<Document> = db.collection("slaps");
    let pipeline = vec![
        doc! { "$group": { "_id": "$author", "total": { "$sum": "$count" } } },
        doc! { "$sort": { "total": -1 } },
        doc! { "$limit": 3 },
    ];
    let mut cursor = collection.aggregate(pipeline, None).await?;
    let mut results = vec![];
    while let Some(result) = cursor.next().await {
        let result = result.unwrap();
        results.push(SlapCount {
            author: result.get_i64("_id").unwrap(),
            count: result.get_i32("total").unwrap(),
        });
    }
    Ok(results)
}

pub async fn top3_slappees(db: Database) -> Result<Vec<SlapCount>, mongodb::error::Error> {
    let collection: Collection<Document> = db.collection("slaps");
    let pipeline = vec![
        doc! { "$group": { "_id": "$target", "total": { "$sum": "$count" } } },
        doc! { "$sort": { "total": -1 } },
        doc! { "$limit": 3 },
    ];
    let mut cursor = collection.aggregate(pipeline, None).await?;
    let mut results = vec![];
    while let Some(result) = cursor.next().await {
        let result = result.unwrap();
        results.push(SlapCount {
            author: result.get_i64("_id").unwrap(),
            count: result.get_i32("total").unwrap(),
        });
    }
    Ok(results)
}