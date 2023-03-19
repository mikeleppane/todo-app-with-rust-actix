use std::collections::HashMap;
use std::env;

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path)
            .unwrap_or_else(|_| panic!("Cannot find file: {}", file_path));
        let mut map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not defined!");
        map.insert(
            String::from("DB_URL"),
            serde_yaml::from_str(database_url.as_str()).unwrap(),
        );
        Config { map }
    }
}
