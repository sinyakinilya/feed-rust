use mongodb::{
    bson::{doc, Bson},
    sync::{Client, Collection},
};

use uuid::Uuid;

#[derive(Debug)]
pub struct FeedCollection {
    collection: Collection,
}

// pub struct FindFeedReq {
//     pub row_id: Uuid,
// }

#[derive(Debug)]
pub struct Feed {
    //    pub idempotent_key: Uuid,
    pub row_id: Uuid,
    pub account_id: Uuid,
}

impl FeedCollection {
    pub fn new(url: &str) -> Self {
        println!("try to connect {}", url);
        let client = Client::with_uri_str(url).unwrap();
        let database = client.database("feed");
        let collection = database.collection("feed");

        Self { collection }
    }

    pub fn find_feed(&self, row_id: Uuid) -> Vec<Feed> {
        let filter = doc! { "id": row_id.to_hyphenated().to_string() };
        let cursor = self.collection.find(filter, None).unwrap();

        let mut feed: Vec<Feed> = vec![];
        for result in cursor {
            match result {
                Ok(document) => {
                    if let Some(account_id) = document.get("account_id").and_then(Bson::as_str) {
                        let acc = Uuid::parse_str(account_id).unwrap();
                        feed.push(Feed {
                            row_id: row_id,
                            account_id: acc,
                        })
                    } else {
                        println!("no account_id found");
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        feed
    }
}
