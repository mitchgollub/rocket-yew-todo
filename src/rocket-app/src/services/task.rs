use super::super::MongoClient;
use mongodb::error::Error;
use todo_models::Entry;

pub struct TaskService {
    mongo_client: MongoClient,
}

impl TaskService {
    pub fn new(mongo_client: MongoClient) -> TaskService {
        TaskService {
            mongo_client: mongo_client,
        }
    }

    pub fn get_tasks(&mut self) -> Result<Vec<Entry>, Error> {
        self.mongo_client.get_tasks()
    }

    pub fn update_task(&mut self, entry: Entry) -> Result<Entry, Error> {
        let mut new_entry = entry.clone();

        // _id will be an empty string for new entries from front-end
        match entry._id.is_empty() {
            false => {
                // Update Document
                new_entry = self.mongo_client.update_task(entry)?;
            }
            true => {
                // Insert new document
                new_entry._id = self.mongo_client.insert_task(entry)?;
            }
        }

        Ok(new_entry)
    }

    pub fn update_tasks(&mut self, entries: Vec<Entry>) -> Result<Vec<Entry>, Error> {
        let mut updates = Vec::new();

        for entry in entries.iter() {
            let mut new_entry = entry.clone();

            // _id will be an empty string for new entries from front-end
            match entry._id.is_empty() {
                false => {
                    // Update Document
                    new_entry = self.mongo_client.update_task(new_entry)?;
                }
                true => {
                    // Insert new document
                    new_entry._id = self.mongo_client.insert_task(new_entry.clone())?;
                }
            }

            updates.push(new_entry);
        }

        Ok(updates)
    }

    pub fn delete_task(&mut self, entry_id: String) -> Result<String, Error> {
        self.mongo_client.delete_task(entry_id)
    }
}
