
use tokio_postgres::{NoTls, Error};
extern crate serde;
extern crate serde_json;


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
  println!("Connection is a success");
  let mut persons : Vec<Person> = Vec::new();

  tokio::spawn(async move {
    if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
    }
});

  for row in client.query("SELECT id,name,state,country FROM dataprocessor", &[]).await? {
    println!("Found a record");
    let mut person = Person {
      id: row.get(0),
      name: row.get(1),
      state: row.get(2),
      country: row.get(3),
    };
    persons.push(person);
  }
  Ok((persons))
  
}