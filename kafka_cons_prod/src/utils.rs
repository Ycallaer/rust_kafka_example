
use clap::{App, Arg};
use rdkafka::config::ClientConfig;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;

pub fn get_kafka_config() -> Result<(String, ClientConfig), Box<dyn std::error::Error>> {
  let app = App::new("rust kafka cli")
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
            .arg(
                Arg::with_name("config")
                    .help("path to kafka config file")
                    .long("config")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("topic")
                    .help("topic")
                    .long("topic")
                    .default_value("topic_rust")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();
  
  let mut clientConfig = ClientConfig::new();
  let file = File::open(app.value_of("config").ok_or("error parsing config")?)?;
  let reader = BufReader::new(&file);
  for line in reader.lines(){
    let curr_line = line.unwrap().trim().to_string();
    let key_value: Vec<&str> = curr_line.split("=").collect();
    println!("Key we are setting is {}",key_value.index(0).to_string());
    println!("value we are setting is {}",key_value.index(1).to_string());
    clientConfig.set(key_value.index(0).to_string(), key_value.index(1).to_string());
  }

  Ok((
    app.value_of("topic")
        .ok_or("error parsing topic")?
        .to_string(),
      clientConfig,
  ))
}