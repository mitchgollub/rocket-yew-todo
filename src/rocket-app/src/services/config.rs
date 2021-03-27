use std::env;

fn get_env_var(variable_name: String) -> String {
    match env::var(variable_name) {
        Ok(value) => value,
        Err(err) => panic!("{:?}", err),
    }
}

#[derive(Clone)]
pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_db: String,
    pub mongodb_collection: String,
    pub static_files: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            mongodb_uri: get_env_var("MONGODB_URI".into()),
            mongodb_db: get_env_var("MONGODB_DB".into()),
            mongodb_collection: get_env_var("MONGODB_COLLECTION".into()),
            static_files: get_env_var("STATIC_FILES".into()),
        }
    }
}
