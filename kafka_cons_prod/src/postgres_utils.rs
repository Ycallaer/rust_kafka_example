
use std::convert::identity;

use rdkafka::producer;
use tokio_postgres::{NoTls, Error};
extern crate serde;
extern crate serde_json;

use prost::Message;



// Include the `customer` module, which is generated from identity.proto.
pub mod identity {
  include!(concat!(env!("OUT_DIR"), "/customer.identity.rs"));
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
  id: i32,
  name: String,
  state: String,
  country: String
}

impl Default for Person {
    fn default() -> Self {
        Self { id: 0, name: String::from(""), state: String::from(""), country: String::from("") }
    }
}

pub async fn read_sql_data_person() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
  println!("Connection to the database");
  let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres port=54320 dbname=postgres", NoTls).await?;
  let mut persons : Vec<Person> = Vec::new();
  let mut persons_proto_vec : Vec<identity::Identity> = Vec::new();


  tokio::spawn(async move {
    if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
    }
    else{
      println!("Connection is successful")
    }
});

  for row in client.query("SELECT id,name,state,country FROM dataprocessor", &[]).await? {
    println!("Found a record {:#?}",row);
    let mut person_proto = identity::Identity::default();


    let mut person = Person {
      id: row.get(0),
      name: row.get(1),
      state: row.get(2),
      country: row.get(3),
    };
    persons.push(person);
    //Protobuf implementation
    person_proto.id = row.get(0);
    person_proto.name = row.get(1);
    person_proto.state = row.get(2);
    person_proto.country = row.get(3);

    persons_proto_vec.push(person_proto);
    
  }
  Ok(persons)
  
}