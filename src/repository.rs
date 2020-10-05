use mongodb::{
    bson::{doc, Bson, Document},
    error::{ErrorKind, WriteFailure},
    sync::{Client, Collection},
};

use std::{collections::HashMap, time::SystemTime};
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
        println!("try to connect feed {}", url);
        let client = Client::with_uri_str(url).unwrap();
        let database = client.database("feed");
        let collection = database.collection("feed");

        Self { collection }
    }

    pub fn find_feed(&self, ids: Vec<String>) -> Result<Vec<FeedRow>, ()> {
        let filter = doc! {
            "id": {"$in": ids}
        };
        let cursor = self.collection.find(filter, None).unwrap();

        let mut feed: Vec<FeedRow> = vec![];
        for result in cursor {
            match result {
                Ok(document) => {
                    let row = FeedRow::from(document);
                    feed.push(row);
                    // if let Some(account_id) = document.get("account_id").and_then(Bson::as_str) {
                    //     let acc = Uuid::parse_str(account_id).unwrap();
                    //     feed.push(Feed {
                    //         row_id: row_id,
                    //         account_id: acc,
                    //     })
                    // } else {
                    //     println!("no account_id found");
                    // }
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(());
                }
            }
        }
        Ok(feed)
    }
}

pub struct FeedRow {
    pub idempotent_key: IdempotentKey,
    pub id: FeedRowID,
    pub account_id: AccountID,
    pub partner_account_id: AccountID,
    pub group_id: String,
    pub operation_type: String, // "p2p Sell | p2p Buy | Deposit(currency|asset) | Withdraw(currency|asset)"
    pub contractor_id: ContractorID,
    pub operation_objects: Vec<OperationObject>,
    pub balance: Balance,
    pub states: States,
    pub current_state: CurrentState,
    pub details: Details,
    pub created_at: SystemTime,
}
pub type FeedRowID = String;
pub type AccountID = Uuid;
pub type ContractorID = String;
pub type Details = HashMap<String, Vec<u8>>;

pub type ObjectMeta = HashMap<String, Vec<u8>>; //golang map[string]interface{}

pub struct OperationObject {
    pub id: String,
    pub title: String,
    pub url: String,
    pub meta: ObjectMeta,
}

pub struct Balance {
    pub balance_type: String,
    pub amount: i64,
    pub asset_title: String,
    pub asset_type: String,
}
pub struct CurrentState {
    pub id: StateID,
    pub code: StateCode,
    pub params: Params, // "timer":"123123123","active":"true","link_url":"http://escort-service.endpoint?param1=1"
    pub updated_at: SystemTime,
}
pub type Params = HashMap<String, Vec<u8>>;
pub type States = HashMap<StateID, State>;
pub type State = HashMap<StateCode, StateInfo>;
pub type StateCode = String;
pub type StateID = String;
pub struct StateInfo {
    params: Params,
    created_at: SystemTime,
}

pub type IdempotentKey = String;

impl From<Document> for FeedRow {
    fn from(d: Document) -> Self {
        let idempotent_key = d.get("_id").and_then(Bson::as_str).unwrap().to_string();
        let feed_row_id = d.get_str("id").unwrap().to_string();
        let account_id = d
            .get("account_id")
            .and_then(Bson::as_str)
            .and_then(|s| match Uuid::parse_str(s) {
                Ok(id) => Some(id),
                Err(_) => None,
            })
            .unwrap();
        let oo = vec![OperationObject {
            id: String::from("oo_id"),
            title: String::from("oo_title"),
            url: String::from("oo_url"),
            meta: HashMap::new(),
        }];
        FeedRow {
            idempotent_key: idempotent_key,
            id: feed_row_id,
            account_id: account_id,
            partner_account_id: Uuid::new_v4(),
            group_id: String::from("group_id"),
            operation_type: String::from("p2pSell"),
            contractor_id: String::from("contractor_id"),
            operation_objects: oo,
            balance: Balance {
                balance_type: String::from("Credit"),
                amount: 500,
                asset_title: String::from("currency|asset"),
                asset_type: String::from("USD|asset"),
            },
            states: HashMap::new(),
            current_state: CurrentState {
                id: String::from("StateID"),
                code: String::from("StateCode"),
                params: HashMap::new(),
                updated_at: SystemTime::now(),
            },
            details: HashMap::new(),
            created_at: SystemTime::now(),
        }
    }
}

pub struct ContractorCollection {
    collection: Collection,
}

impl ContractorCollection {
    pub fn new(url: &str) -> Self {
        println!("try to connect contractor collection{}", url);
        let client = Client::with_uri_str(url).unwrap();
        let database = client.database("feed");
        let collection = database.collection("contractor");

        Self { collection }
    }

    pub fn store(&self, app: &Contractor) -> Result<(), ()> {
        let mut params = Document::new();
        for k in app.params.keys().into_iter() {
            params.insert(k.to_string(), app.params[k].to_string());
        }
        let doc = doc! {
            "_id": app.id.clone(),
            "params": params,
        };

        match *self.collection.insert_one(doc, None).unwrap_err().kind {
            ErrorKind::WriteError(WriteFailure::WriteError(ref w_error)) => {
                if w_error.code == 11000 {
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Contractor {
    pub id: String,
    pub params: HashMap<String, String>,
}

impl Into<Document> for Contractor {
    fn into(self) -> Document {
        let mut d = Document::new();
        for k in self.params.keys().into_iter() {
            d.insert(k.to_string(), self.params[k].to_string());
        }
        d.insert("_id", self.id);
        d
    }
}

impl From<Document> for Contractor {
    fn from(d: Document) -> Self {
        let id = d.get("_id").and_then(Bson::as_str).unwrap().to_string();

        let doc_params = d.get_document("params").unwrap();
        let mut params: HashMap<String, String> = HashMap::new();
        for k in doc_params.keys().into_iter() {
            let v = d.get_str(k).unwrap();
            params.insert(k.to_string(), v.to_string());
        }
        Self { id, params }
    }
}
