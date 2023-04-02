use rdkafka::producer::{FutureProducer, FutureRecord};
use std::{time::Duration};
use rdkafka::message::{Header, OwnedHeaders};

#[macro_use] extern crate serde_derive;
use postgres_utils::read_sql_data_person;

mod utils;
mod postgres_utils;

async fn produce() {
  let (topic, config) = utils::get_kafka_config().unwrap();
  let producer: FutureProducer = config.create().unwrap();

  let persons = read_sql_data_person().await;
  let mut person_array = persons.unwrap();
  // This loop is non blocking: all messages will be sent one after the other
  for person in person_array {
    let local_value = serde_json::to_string(&person).unwrap();
    let record = FutureRecord::to(&topic).key("").payload(&local_value).headers(OwnedHeaders::new().insert(Header {
      key: "header_key",
      value: Some("header_value"),
    }));
    let ack = producer.send(record, Duration::from_secs(0)).await.unwrap();
    println!("The response is {:#?}",ack);
  }
}

#[tokio::main]
async fn main() {
 
  print!("Starting the app");

  produce().await;
}