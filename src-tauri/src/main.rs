// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app_lib::get_all_topic;
use app_lib::EMessage;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::BaseProducer;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use std::fmt::Write;
use std::time::Duration;
use tauri::{ipc::Channel, AppHandle};
use rdkafka::util::Timeout;
use tauri::{Config, Manager};
use std::sync::{Arc, Mutex,MutexGuard};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn my_custom_command() -> String {
    println!("I was invoked from JavaScript!");
    format!("I was invoked from JavaScript!")
}

#[tauri::command]
fn get_all_topic_from_server(server: String) -> app_lib::Config {
    let cfg = &mut app_lib::Config {
        brokers: server.to_string(),
        topics: Vec::new(),
        groups: Vec::new(),
        header: false,
        host: false,
        size: false,
        topic_separators: false,
        err: None,
    };
    let res: Result<&mut app_lib::Config, KafkaError> = app_lib::get_all_topic(cfg);
    match res {
        Ok(ok) => {
            return ok.clone();
        }
        Err(err) => {
            return app_lib::Config {
                brokers: server.to_string(),
                topics: Vec::new(),
                groups: Vec::new(),
                header: false,
                host: false,
                size: false,
                topic_separators: false,
                err: Some(format!("{:?}", err)),
            }
        }
    }
    return res.unwrap().clone();
}

#[tauri::command]
 async fn consume_kafka( resource:app_lib::SlotResource, config: app_lib::ConsumerConfig) ->Result<Vec<EMessage>,String> {
    let content_length = 1000;

    // let consumer: BaseConsumer = ClientConfig::new()
    //     .set("group.id", "aaa")
    //     .set("bootstrap.servers", resource.server.clone())
    //     .create()
    //     .unwrap();
    let consumeArc:Arc<BaseConsumer> = app_lib::get_consumer(resource);
    println!("config {:?}", config);
    let _slice: Vec<&str> = vec![config.topic.as_str()];
    println!("water 2------" );
     
    let consumer = consumeArc;

  
    consumer.subscribe(_slice.as_slice());
    consumer.poll(Timeout::After(Duration::new(5, 0)));
    let mut count = 0;
    loop{
        count = count+1;
        if(count > 10){
            break;
        }
      let ass : KafkaResult<rdkafka::topic_partition_list::TopicPartitionList>= consumer.assignment();
      match ass {
          Ok(ok) =>{
            ok.count();
            println!("ass ok {:?}, count {:}" , ok,ok.count());
            if ok.count() > 0{
            break;
            }
            consumer.poll(Timeout::After(Duration::new(5, 0)));
          },
          Err(e)=>{
            println!("err {:?}" ,e)
          }
      }
    }
    let  water_res :Result<(i64, i64),KafkaError> =  consumer.fetch_watermarks(config.topic.as_str(), 0,Timeout::After(Duration::new(5, 0)));
    let water:(i64,i64) = water_res.unwrap();
    let _offset  = if water.1 - 30 > water.0 {  water.1  - 30 } else{ water.0 };
    println!("offset is {:?} " , _offset);
    println!("water t" );
    // if let Err(tem) = water_res{
    //     println!("tem {:?}",tem);
    //     return Err(format!("{:?}",tem ));
    // }
    println!("water {:?}" ,water);
    let seek_res = consumer.seek(config.topic.as_str(), 0, rdkafka::topic_partition_list::Offset::Offset(_offset)    , Timeout::After(Duration::new(5, 0)));
    match seek_res {
        Err(err)=>{
            println!("seek err{:?}" , err);
        },
        Ok(ok)=>{
            println!("ok")
        }
    }
    let mut list: Vec<EMessage> = Vec::new();
    loop{
      let res =   consumer.poll( Timeout::After(Duration::new(5, 0)));
         match res {
            Some(Ok(message)) => {
                
                match message.payload_view::<str>() {
                    Some(Ok(m)) => {
                        let mut temp = 0;
                        unsafe {
                           app_lib::index = app_lib::index +1;
                           temp  = app_lib::index;
                        }
                        list.push(EMessage {
                            index:temp,
                            key: key_helper(&message),
                            value:Some(m.to_string()),
                            header:None,
                            timestamp:getTimeStamp(&message),
                            offset:message.offset(),
                            partition:message.partition()
                        });
                        println!("on message{:?}", m);
                        // on_event.send(list.clone()).unwrap();
                    }
                    Some(Err(e)) => {
                        println!("some err{:?}", e);
                        return  Err(format!("{:?}" ,e));
                        // Err(KafkaError::AdminOpCreation(String::from("err")))
                    }
                    None => {
                        println!("none");
                        return  Ok(Vec::new());
                    }
                }
            }
            Some(Err(err)) => {
                //    Err(err);
                println!("some-- err{:?}", err);
                return  Err(format!("some-- err{:?}", err));

            },
            None =>{
                print!("poll null");
                break;
            }
        }     
    }
    print!("return aaa");
    list.sort_unstable_by(|a,b|  a.timestamp.cmp(&b.timestamp).reverse() );
    return Ok(list)
    //
    // consumer.subscribe(slice.as_slice());
    // let mut list: Vec<EMessage> = Vec::new();
    // loop {
    //     let res = consumer.recv().await;

    //     match res {
    //         Ok(message) => {
                
    //             match message.payload_view::<str>() {
    //                 Some(Ok(m)) => {
    //                     let mut temp = 0;
    //                     unsafe {
    //                        app_lib::index = app_lib::index +1;
    //                        temp  = app_lib::index;
    //                     }
    //                     list.push(EMessage {
    //                         index:temp,
    //                         key: key_helper(&message),
    //                         value:Some(m.to_string()),
    //                         header:None,
    //                         timestamp:getTimeStamp(&message),
    //                         offset:message.offset(),
    //                         partition:message.partition()
    //                     });
    //                     println!("on message{:?}", m);
    //                     // on_event.send(list.clone()).unwrap();
    //                    return  Ok(list);
    //                 }
    //                 Some(Err(e)) => {
    //                     println!("some err{:?}", e);
    //                     return  Err(format!("{:?}" ,e));
    //                     // Err(KafkaError::AdminOpCreation(String::from("err")))
    //                 }
    //                 None => {
    //                     println!("none");
    //                     return  Ok(Vec::new());
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             //    Err(err);
    //             println!("some-- err{:?}", err);
    //             return  Err(format!("some-- err{:?}", err));

    //         }
    //     }
    // }
}
fn key_helper<>(mess :&rdkafka::message::BorrowedMessage )->Option<String>{
    match mess.key_view::<str>(){
        Some(Ok(k))=>{
            Some(String::from(k.clone()))
        }, 
        None => None,
        Some(Err(e))=>{
            Some(format!("{:?}","fmt key err"))
        }
    }

}
fn getTimeStamp(mess :&rdkafka::message::BorrowedMessage) ->Option<i64>{
     mess.timestamp().to_millis()
}
//   return  Ok(list.clone());

#[tauri::command]
fn send_kafka(server: String, topic: String, message: String) -> String {
    let producer: &BaseProducer = &ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    return String::from("ok");
}

#[tauri::command]
fn get_all_group_from_kafka(server: String) -> app_lib::Config {
    let cfg = &mut app_lib::Config {
        brokers: server.to_string(),
        topics: Vec::new(),
        groups: Vec::new(),
        header: false,
        host: false,
        size: false,
        topic_separators: false,
        err: None,
    };
    let res = app_lib::get_all_group(cfg);
    match res {
        Ok(ok) => {
            return ok.clone();
        }
        Err(err) => {
            return app_lib::Config {
                brokers: server.to_string(),
                topics: Vec::new(),
                groups: Vec::new(),
                header: false,
                host: false,
                size: false,
                topic_separators: false,
                err: Some(format!("{:?}", err)),
            }
        }
    }
    return res.unwrap().clone();
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        ///// begin debug
        // .setup(|app| {
        //     #[cfg(debug_assertions)] // only include this code on debug builds
        //     {
        //       let window = app.get_webview_window("main").unwrap();
        //       window.open_devtools();
        //       window.close_devtools();
        //     }
        //     Ok(())
        //   })
        ///////end debug
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .invoke_handler(tauri::generate_handler![
            send_kafka,
            get_all_topic_from_server,
            get_all_group_from_kafka,
            consume_kafka
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
