use futures::StreamExt;
use mongodb::bson::{doc, document::Document, Bson};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use rust_tools::bson::{to_doc, to_doc_vec};

const DB_NAME: &str = "articles";
const COLL: &str = "published";
const LIMIT: i64 = 24;

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Hits {
    pub results : Vec<String>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl DB {
    pub async fn init(url: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = Some("mongodb-frontpage".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn search(&self, query: &str) -> Result<Hits> {

        let pretty_query = format!("{{{}}}", query);

        // Log info
        log::info!(
            "query={}",
            &pretty_query
        );

        let data = match to_doc(&pretty_query) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let find_options = FindOptions::builder()
            .sort(doc! { "time": -1 })
            .limit(Some(LIMIT))
            .build();

        let mut cursor = self.get_collection().find(data, find_options).await?;

        let mut results: Vec<String> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match serde_json::to_string(&doc?) {
                Ok(converted) => results.push(converted),
                Err(e) => {
                    log::error!("Caught error, skipping: {}", e);
                    continue;
                }
            }
        }
        Ok(Hits { results })
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }
}
