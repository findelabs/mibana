use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, options::FindOptions, Client};
use serde::{Deserialize, Serialize};
use rust_tools::bson::{to_doc};
//use bson::Bson;

const LIMIT: i64 = 24;

#[derive(Clone, Debug)]
pub struct DB {
    pub db: String,
    pub client: Client
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Hits {
    pub results : Vec<String>,
    pub collections: Vec<String>
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl DB {
    pub async fn init(url: &str, db: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("mongodb-mibana".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
            db: db.to_owned()
        })
    }

    pub async fn collections(&self) -> Result<Vec<String>> {
        // Log that we are trying to list collections
        log::debug!("Getting collections in {}", self.db);

        match self
            .client
            .database(&self.db)
            .list_collection_names(None)
            .await
        {
            Ok(collections) => {
                log::info!("Success listing collections in {}", self.db);
                Ok(collections)
            }
            Err(e) => {
                log::error!("Got error {}", e);
                Ok(Vec::new())
            }
        }
    }

    pub async fn search(&self, query: &str, collection: &str, projection: &str) -> Result<Hits> {
        let pretty_query = format!("{{{}}}", query);
        let pretty_projection = format!("{{{}}}", projection);

        // Log info
        log::info!(
            "query={} projection={}",
            &pretty_query,
            &pretty_projection
        );

        // Convert json to bson
        let data = match to_doc(&pretty_query) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        // Convert json to bson
        let projection = match to_doc(&pretty_projection) {
            Ok(mut d) => { 
                d.insert("_id", 0);
                d 
            },
            Err(e) => return Err(e),
        };

        let find_options = FindOptions::builder()
            .sort(doc! { "_time": -1 })
            .limit(Some(LIMIT))
            .projection(Some(projection))
            .build();

        let collection = self.client.database(&self.db).collection(collection);

        let mut cursor = collection.find(data, find_options).await?;

        let mut results: Vec<String> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match serde_json::to_string_pretty(&doc?) {
                Ok(converted) => {
                    results.push(converted)
                },
                Err(e) => {
                    log::error!("Caught error, skipping: {}", e);
                    continue;
                }
            }
        }

        let collections = self.collections().await?;
        Ok(Hits { results, collections })
    }
}
