use rdkafka::client;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
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

#[derive(Debug,Clone,Serialize)]
pub struct EGroupsInfo{
    pub state :String,
    pub members: Vec<Member>,
    pub name:String

}
#[derive(Debug,Clone,Serialize)]
pub struct Member{
    pub client_id : String,

}

#[derive(Serialize,Clone,Debug)]
pub struct Config {
   pub brokers: String,
   pub topics: Vec<String>,
   pub groups: Vec<EGroupsInfo>,
   pub header: bool,
   pub host: bool,
   pub size: bool,
   pub topic_separators: bool,
   pub err : Option<String>
}

pub fn get_all_group(cfg :&mut Config) -> Result<&mut Config,KafkaError>{
    let producer: &BaseProducer = &ClientConfig::new()
    .set("bootstrap.servers", cfg.brokers.clone())
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");
    let mut list : Vec<EGroupsInfo> = Vec::new();
  let res =   producer.client().fetch_group_list(None, Timeout::After(Duration::new(5,0)));
    match res {
        Ok(ok) =>{
           let groupList =  ok.groups();

           println!("aaaabbb{:?}: empty" ,groupList);

           for  val in groupList {
            // println!("---{:?}", val.members());
            println!("state: {:?}",val.state());
            if val.state() == "Empty"{
                list.push(EGroupsInfo{
                    state:String::from("Empty"),
                    members: Vec::new(),
                    name:String::from(val.name()),
                });
            }else{
                let mut members = Vec::new();
                for  a in  val.members(){
                    members.push(Member{
                        client_id:String::from(a.client_id())
                    });
                    println!("clientid :{:?}", a.client_id())
                }
                list.push(EGroupsInfo{
                    state: String::from("NotEmpty"),
                    members:members,
                    name:String::from(val.name()),
                })
            }
        
           }
           cfg.groups = list;
        },
        Err(err)=>{
           return Err(err);
        }
    }
    println!("all group is {:?}" , cfg.groups);
        Ok(cfg)
}
pub fn get_all_topic(cfg :&mut Config) -> Result<&mut Config,   KafkaError>{

    let consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", cfg.brokers.clone())
    .create()?;
let metadata = consumer
.fetch_metadata(None,  Duration::new(5, 0))?;
for topic in metadata.topics() {
    println!("  Topic: {}  Err: {:?}", topic.name(), topic.error());
    cfg.topics.push(String::from(topic.name()))
}


    Ok(cfg)
}

