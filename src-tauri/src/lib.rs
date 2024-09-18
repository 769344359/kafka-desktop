use kafka::client::{FetchOffset, KafkaClient};
use serde::Serialize;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize,Clone,Debug)]
pub struct Config {
   pub brokers: Vec<String>,
   pub topics: Vec<String>,
   pub header: bool,
   pub host: bool,
   pub size: bool,
   pub topic_separators: bool,
}
pub fn get_all_topic(cfg :&mut Config) -> Result<&mut Config, String>{
    let mut client = KafkaClient::new(cfg.brokers.clone());
    client.load_metadata_all().map_err(|e| e.to_string())?;
    // ~ determine the list of topics we're supposed to report about
    let topics = if cfg.topics.is_empty() {
        let topics = client.topics();
        let mut names = Vec::with_capacity(topics.len());
        for topic in topics.names() {
            names.push(topic.to_owned());
        }
        names.sort();
        names
    } else {
        cfg.topics.clone()
    };
    
    // println!("topic------:{:?}", topics);
    // topics;
    cfg.topics = topics;
    Ok(cfg)
}

