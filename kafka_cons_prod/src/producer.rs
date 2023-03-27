use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

mod utils;

async fn produce() {
  let (topic, config) = utils::get_kafka_config().unwrap();
  let producer: FutureProducer = config.create().unwrap();

  // This loop is non blocking: all messages will be sent one after the other, without waiting
  // for the results.
  let futures = (0..50)
      .map(|i| {
          // The send operation on the topic returns a future, which will be
          // completed once the result or failure from Kafka is received.
          let delivery_status = producer
              .send(
                  FutureRecord::to(&topic)
                      .payload("message")
                      .key("key"),
                  Duration::from_secs(10),
              )
              ;
          print!("Inside the loop");
          // This will be executed when the result is received.
          print!("Delivery status for message {} received", i);
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