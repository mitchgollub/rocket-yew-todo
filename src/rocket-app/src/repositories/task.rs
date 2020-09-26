use mongodb::{
    bson::{bson, doc, oid::ObjectId, Bson, Document},
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

pub fn get_tasks() -> Result<Vec<Entry>, Error> {
    let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())?;
    let database = client.database("mafia-dev");
    let collection = database.collection("rust-todo");

    let mut documents = Vec::new();
    let cursor = collection.find(doc! {}, None)?;

    for result in cursor {
        match result {
            Ok(document) => {
                documents.push(Entry {
                    _id: document
                        .get("_id")
                        .unwrap()
                        .as_object_id()
                        .unwrap()
                        .to_hex(),
                    completed: document.get("completed").and_then(Bson::as_bool).unwrap(),
                    description: document
                        .get("description")
                        .and_then(Bson::as_str)
                        .unwrap()
                        .to_string(),
                    editing: document.get("editing").and_then(Bson::as_bool).unwrap(),
                });
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(documents)
}

pub fn update_tasks(entries: Vec<Entry>) -> Result<Vec<Entry>, Error> {
    let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())?;
    let database = client.database("mafia-dev");
    let collection = database.collection("rust-todo");

    let mut updates = Vec::new();

    for entry in entries.iter() {
        let mut new_entry = entry.clone();

        let update_doc = doc! {
            "completed": entry.completed,
            "description": entry.description.as_str(),
            "editing": entry.editing
        };

        // Need to create this only if _id is not an empty string
        let cursor = collection.find(
            match entry._id.is_empty() {
                false => doc! {
                    "_id": ObjectId::with_string(entry._id.as_str()).unwrap()
                },
                true => doc! {},
            },
            None,
        )?;
        let mut results: Vec<Result<Document, Error>> = cursor.collect();
        if results.len() == 1 {
            collection.update_one(results.pop().unwrap().unwrap(), update_doc.clone(), None)?;
            updates.push(new_entry);
        } else {
            let insert_response = collection.insert_one(update_doc.clone(), None)?;
            new_entry._id = insert_response.inserted_id.as_object_id().unwrap().to_hex();
            updates.push(new_entry);
        }
    }

    Ok(updates)
}
