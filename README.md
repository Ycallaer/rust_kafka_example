# rust_kafka_example
The following repo is an example of a Kafka Producer. (Consumer implementation to come.)
The data is being read from a postgres database and we push the data as JSON to the kafka Topic.
The project is currently trying to do a protobuf implementation

# Prerequisites
The following needs to be installed on your local machine:
* Working kafka stack
* Working postgresql with a table called `dataprocessor`. The sql script can be found in `sql/sql_creation.sql`
* Installation of proto

Before you can run the projec, you need to do a `cargo build`, this also generates the code bindings for the proto file.

# Run command
In order to run the application you need to execute the following command
```
cargo run -- --config /home/yves/PersonalProjects/rust_kafka_example/kafka_cons_prod/producer.config --topic rust_test2
```
Note that the producer config needs to updated as per your environment

