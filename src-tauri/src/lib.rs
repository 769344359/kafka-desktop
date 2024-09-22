use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::groups::GroupList;
use rdkafka::util::Timeout;
use std::time::Duration;
use serde::Serialize;
use rdkafka::producer::{BaseProducer, Producer};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize,Clone,Debug)]
pub struct Config {
   pub brokers: String,
   pub topics: Vec<String>,
   pub groups: Vec<String>,
   pub header: bool,
   pub host: bool,
   pub size: bool,
   pub topic_separators: bool,
}

pub fn get_all_group(cfg :&mut Config) -> Result<&mut Config,String>{
    let producer: &BaseProducer = &ClientConfig::new()
    .set("bootstrap.servers", cfg.brokers.clone())
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");
  let res =   producer.client().fetch_group_list(None, Timeout::After(Duration::new(5,0)));
    match res {
        Ok(ok) =>{
           let groupList =  ok.groups();
           let mut  l = Vec::new();
           for  val in groupList {
             l.push(String::from(val.name()))
           }
           cfg.groups = l;
        },
        Err(_)=>{}
    }
        Ok(cfg)
}
pub fn get_all_topic(cfg :&mut Config) -> Result<&mut Config, String>{

    let consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", cfg.brokers.clone())
    .create()
    .expect("Consumer creation failed");
let metadata = consumer
.fetch_metadata(None,  Duration::new(5, 0))
.expect("Failed to fetch metadata");
for topic in metadata.topics() {
    println!("  Topic: {}  Err: {:?}", topic.name(), topic.error());
    cfg.topics.push(String::from(topic.name()))
    // for partition in topic.partitions() {
    //     println!(
    //         "     Partition: {}  Leader: {}  Replicas: {:?}  ISR: {:?}  Err: {:?}",
    //         partition.id(),
    //         partition.leader(),
    //         partition.replicas(),
    //         partition.isr(),
    //         partition.error()
    //     );
    //     if fetch_offsets {
    //         let (low, high) = consumer
    //             .fetch_watermarks(topic.name(), partition.id(), Duration::from_secs(1))
    //             .unwrap_or((-1, -1));
    //         println!(
    //             "       Low watermark: {}  High watermark: {} (difference: {})",
    //             low,
    //             high,
    //             high - low
    //         );
    //         message_count += high - low;
    //     }
    // }
    // if fetch_offsets {
    //     println!("     Total message count: {}", message_count);
    // }
}
    // let mut client = KafkaClient::new(cfg.brokers.clone());
    // client.load_metadata_all().map_err(|e| e.to_string())?;
    // // ~ determine the list of topics we're supposed to report about
    // let topics = if cfg.topics.is_empty() {
    //     let topics = client.topics();
    //     let mut names = Vec::with_capacity(topics.len());
    //     for topic in topics.names() {
    //         names.push(topic.to_owned());
    //     }
    //     names.sort();
    //     names
    // } else {
    //     cfg.topics.clone()
    // };
    
    // // println!("topic------:{:?}", topics);
    // // topics;
    // cfg.topics = topics;

    Ok(cfg)
}

