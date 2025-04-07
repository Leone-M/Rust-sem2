use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::env;
use std::error::Error;
use std::time::{Duration, UNIX_EPOCH};
use prost::Message;
use postgres::{Client, NoTls};

mod pb {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub fn db_connect() -> Result<Client, Box<dyn Error>> {
    let db_address = env::var("DB_ADDRESS");
    match db_address {
        Ok(db_address) => {
            let client = Client::connect(db_address.as_str(), NoTls);
            match client {
                Ok(client) => {
                    Ok(client)
                }
                Err(e) => {
                    eprintln!("Can not connect to database: {e:#?}");
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            eprintln!("Database address not specified: {e:#?}");
            Err(e.into())
        }
    }
}

pub fn db_create_table(client: &mut Client) -> Result<(), Box<dyn Error>> {
    match client.batch_execute("
    CREATE TABLE ESP_DATA (
    id SERIAL PRIMARY KEY,
    device_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    humidity INTEGER,
    temperature INTEGER,
    event_data TIMESTAMP NOT NULL)") {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.as_db_error().unwrap().message() == "relation \"esp_data\" already exists" {
                Ok(())
            } else {
                eprintln!("Database table creation failed: {e:#?}");
                Err(e.into())
            }
        }
    }
}

pub fn db_add_data(client: &mut Client, data: &pb::Event) -> Result<(), Box<dyn Error>> {
    let time = UNIX_EPOCH + Duration::new(data.read_time.unwrap().seconds as u64, 
    data.read_time.unwrap().nanos as u32);
    match client.execute("
    INSERT INTO ESP_DATA (device_id, event_id, humidity, temperature, event_data)
    VALUES ($1, $2, $3, $4, $5)", &[&data.event_data.unwrap().device_id, &data.event_id,
        &data.event_data.unwrap().humidity, &data.event_data.unwrap().temperature,
        &time]) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to write data: {e:#?}");
            Err(e.into())
        }
    }
}

// connect establishing
pub fn establish_connect() -> Result<TcpListener, Box<dyn Error>> {
    let con_address = env::var("CON_ADDRESS");
    match con_address {
        Ok(con_address) => {
            let listener = TcpListener::bind(con_address);
            match listener {
                Ok(listener) => {
                    Ok(listener)
                },
                Err(e) => {
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            eprintln!("Connection address isn't specified: {e:#?}");
            Err(e.into())
        }
    }
}

// reading stream data
pub fn handle_connection(stream: TcpStream, client: &mut Client) {
    let mut buf_reader = BufReader::new(&stream);
    let mut buf = vec![];
    let _ = buf_reader.read_to_end(&mut buf);
    let message = pb::Event::decode(&*buf);
    match message {
        Ok(event) => {
            let event_id = event.event_id;
            let event_time = UNIX_EPOCH + Duration::new(event.read_time.unwrap().seconds as u64,
                                                                   event.read_time.unwrap().nanos as u32);
            let humidity = event.event_data.unwrap().humidity;
            let temperature = event.event_data.unwrap().temperature;
            let device_id = event.event_data.unwrap().device_id;
            println!("Info of device: {device_id} \n\
            event: {event_id:#?} time: {event_time:#?}\n\
            Humidity: {humidity:#?}\n\
            Temperature: {temperature:#?}");
            db_add_data(client, &event).unwrap()
        }
        Err(e) => {
            eprintln!("Decoding error: {e:#?}")
        }
    }
    }
