// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app_lib::get_all_topic;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::fmt::Write;
use std::time::Duration;
use tauri::{Config, Manager};
use rdkafka::producer::BaseProducer;
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
fn get_all_topic_from_server(server:String)->app_lib::Config{
    let  cfg = &mut app_lib::Config{
        brokers:server.to_string(),    
            topics: Vec::new(),
            header: false,
            host: false,
            size: false,
            topic_separators: false,
        };
  let res =  app_lib::get_all_topic(cfg);
  return  res.unwrap().clone()

}
#[tauri::command]
fn send_kafka(server: String, topic: String, message: String) -> String {
    let producer: &BaseProducer = &ClientConfig::new()
    .set("bootstrap.servers", server)
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");
    // let  cfg = &mut app_lib::Config{
    //     brokers:vec!["127.0.0.1:9092".to_string()],    
    //         topics: Vec::new(),
    //         header: false,
    //         host: false,
    //         size: false,
    //         topic_separators: false,
    //     };
    // let topicRes = get_all_topic( cfg);
    // match topicRes {
    //     Ok(re) =>{
    //         println!("--aa{:#?}" ,re )
    //     }
    //     Err(err) =>{
    //         println!("err{}",err)
    //     }
    // }
    // println!(
    //     "I was invoked from JavaScript, with this message: server{} , topic :{} , message{}",
    //     server, topic, message
    // );
    // let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
    //     .with_ack_timeout(Duration::from_secs(1))
    //     .with_required_acks(RequiredAcks::One)
    //     .create()
    //     .unwrap();

    // let mut buf = message;

    // // let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
    // let res = producer.send(&Record::from_value(&topic, buf.as_bytes()));
    // match res {
    //     Ok(_) => {}
    //     Err(err) => {
    //         println!("err:{}", err);
    //         return format!("{}", err);
    //     }
    // }
    // buf.clear();
    return String::from("ok");
}

fn main() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![send_kafka,get_all_topic_from_server])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
