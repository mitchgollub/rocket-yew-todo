use mongodb::{
    bson::{bson, doc, Bson, Document},
    error::Error,
    sync::Client,
};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    #[serde(default)]
    pub _id: String,
    pub completed: bool,
    pub description: String,
    pub editing: bool,
}

impl From<Entry> for Bson {
    fn from(entry: Entry) -> Bson {
        bson!({
            "_id": entry._id,
            "completed": entry.completed,
            "description": entry.description,
            "editing": entry.editing
        })
    }
}

pub fn get() {
    panic!("Not yet implemented");
}

pub fn update_tasks(entries: Vec<Entry>) -> Result<Vec<Entry>, Error> {
    let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())?;
    let database = client.database("mafia-dev");
    let collection = database.collection("rust-todo");

    let mut updates = Vec::new();

    for entry in entries.iter() {
        let query = doc! {
            "_id": entry._id.as_str()
        };

        let update_doc = doc! {
            "completed": entry.completed,
            "description": entry.description.as_str(),
            "editing": entry.editing
        };

        updates.push(update_doc.clone());

        let cursor = collection.find(query.clone(), None)?;
        let mut results: Vec<Result<Document, Error>> = cursor.collect();
        if results.len() == 0 {
            collection.insert_one(update_doc.clone(), None)?;
        } else {
            collection.update_one(results.pop().unwrap().unwrap(), update_doc.clone(), None)?;
        }
    }

    Ok(updates
        .iter()
        .map(|doc| Entry {
            _id: match doc.get("_id").and_then(Bson::as_str) {
                Some(_id) => _id.to_string(),
                None => String::default(),
            },
            completed: doc.get("completed").and_then(Bson::as_bool).unwrap(),
            description: doc
                .get("description")
                .and_then(Bson::as_str)
                .unwrap()
                .to_string(),
            editing: doc.get("editing").and_then(Bson::as_bool).unwrap(),
        })
        .collect())
}
