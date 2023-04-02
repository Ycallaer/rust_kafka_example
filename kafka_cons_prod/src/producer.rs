use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
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
  // This loop is non blocking: all messages will be sent one after the other, without waiting
  // for the results.
  let futures = (0..person_array.len()-1)
      .map(|i| {
          // The send operation on the topic returns a future, which will be
          // completed once the result or failure from Kafka is received.
          let value = person_array.pop().unwrap();
          let serialized = serde_json::to_string(&value).unwrap().clone();
          let delivery_status = producer
              .send(
                  FutureRecord::to(&topic)
                      .payload("t")
                      .key("key")
                      .headers(OwnedHeaders::new().insert(Header {
                        key: "header_key",
                        value: Some("header_value"),
                    })),
                  Duration::from_secs(10),
              )
              ;
              //producer.send(FutureRecord { topic: (), partition: (), payload: (), key: (), timestamp: (), headers: () }, queue_timeout)

          // This will be executed when the result is received.
          print!("Delivery status for message {} received", serialized);
          delivery_status
      })
      .collect::<Vec<_>>();

  // This loop will wait until all delivery statuses have been received.
  for future in futures {
      print!("Future completed. Result: {:?}", future.await);
  }
}

#[tokio::main]
async fn main() {
 
  print!("Starting the app");

  produce().await;
}