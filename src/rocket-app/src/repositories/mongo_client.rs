use super::super::Config;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    error::Error,
    sync::{Client, Collection},
};
use todo_models::Entry;

pub struct MongoClient {
    config: Config,
    collection: Option<Collection>,
}

fn connect(config: Config) -> Collection {
    Client::with_uri_str(&config.mongodb_uri)
        .unwrap()
        .database(&config.mongodb_db)
        .collection(&config.mongodb_collection)
}

impl MongoClient {
    pub fn new(config: Config) -> MongoClient {
        MongoClient {
            config: config.clone(),
            collection: Some(connect(config)),
        }
    }

    fn connect(&mut self) -> Option<&Collection> {
        if self.collection.is_none() {
            self.collection = Some(connect(self.config.clone()));
        }

        self.collection.as_ref()
    }

    pub fn get_tasks(&mut self) -> Result<Vec<Entry>, Error> {
        let collection = self.connect().unwrap();
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

    pub fn update_task(&mut self, mut entry: Entry) -> Result<Entry, Error> {
        let update_doc = doc! {
            "completed": entry.completed,
            "description": entry.description.as_str(),
            "editing": entry.editing
        };
        let collection = self.connect().unwrap();
        let cursor = collection.find(
            doc! {
                "_id": ObjectId::with_string(entry._id.as_str()).unwrap()
            },
            None,
        )?;
        let mut results: Vec<Result<Document, Error>> = cursor.collect();

        // Make sure document exists
        if results.len() == 1 {
            collection.update_one(results.pop().unwrap().unwrap(), update_doc.clone(), None)?;
        } else {
            let insert_response = collection.insert_one(update_doc.clone(), None)?;
            entry._id = insert_response.inserted_id.as_object_id().unwrap().to_hex();
        }

        Ok(entry)
    }

    pub fn add_task(&mut self, mut entry: Entry) -> Result<Entry, Error> {
        let new_doc = doc! {
            "completed": entry.completed,
            "description": entry.description.as_str(),
            "editing": entry.editing
        };
        let collection = self.connect().unwrap();

        let insert_response = collection.insert_one(new_doc, None)?;
        entry._id = insert_response.inserted_id.as_object_id().unwrap().to_hex();

        Ok(entry)
    }

    pub fn delete_task(&mut self, entry_id: String) -> Result<String, Error> {
        let delete_doc = doc! {
            "_id": ObjectId::with_string(&entry_id).unwrap()
        };

        let collection = self.connect().unwrap();
        collection.delete_one(delete_doc, None)?;

        Ok(entry_id)
    }

    pub fn insert_task(&mut self, entry: Entry) -> Result<String, Error> {
        let update_doc = doc! {
            "completed": entry.completed,
            "description": entry.description.as_str(),
            "editing": entry.editing
        };
        let collection = self.connect().unwrap();
        let insert_response = collection.insert_one(update_doc.clone(), None)?;
        Ok(insert_response.inserted_id.as_object_id().unwrap().to_hex())
    }
}
