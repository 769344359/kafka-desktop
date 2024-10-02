// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app_lib::get_all_topic;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::fmt::Write;
use std::time::Duration;
use tauri::{Config, Manager};
use rdkafka::error::KafkaError;
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
            groups: Vec::new(),
            header: false,
            host: false,
            size: false,
            topic_separators: false,
            err:None
        };
  let res :Result<&mut app_lib::Config,   KafkaError>=  app_lib::get_all_topic(cfg);
  match res{
    Ok(ok)=>{
        return ok.clone();
    },
    Err(err) =>{
        return app_lib::Config{
            brokers:server.to_string(),    
            topics: Vec::new(),
            groups: Vec::new(),
            header: false,
            host: false,
            size: false,
            topic_separators: false,
            err: Some(format!("{:?}",err))
        }
    }
  }
  return  res.unwrap().clone()

}
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
fn get_all_group_from_kafka(server:String) -> app_lib::Config {
    let  cfg = &mut app_lib::Config{
        brokers:server.to_string(),    
            topics: Vec::new(),
            groups: Vec::new(),
            header: false,
            host: false,
            size: false,
            topic_separators: false,
            err:None,
        };
  let res =  app_lib::get_all_group(cfg);
  match res{
    Ok(ok)=>{
        return ok.clone();
    },
    Err(err) =>{
      return   app_lib::Config{
            brokers:server.to_string(),    
            topics: Vec::new(),
            groups: Vec::new(),
            header: false,
            host: false,
            size: false,
            topic_separators: false,
            err:Some(format!("{:?}",err)) 
        }
    }
  }
  return  res.unwrap().clone()
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
        .invoke_handler(tauri::generate_handler![send_kafka,get_all_topic_from_server,get_all_group_from_kafka])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
