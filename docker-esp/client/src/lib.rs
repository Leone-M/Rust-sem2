use std::io::{BufWriter, Write};
use std::io;
use std::net::TcpStream;
use std::error::Error;
use std::time;
use prost;
use std::env;
use prost::{Message};
use prost_types;
use rand::random_range;

mod pb {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub fn establish_connection() -> Result<TcpStream, Box<dyn Error>>{
    let con_address = env::var("CON_ADDRESS");
    match con_address {
        Ok(con_address) => {
            let connector = TcpStream::connect(con_address);
            match connector {
                Ok(connector) => {
                    Ok(connector)
                }
                Err(e) => {
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            Err(e.into())
        }
    }
    
}

pub fn send_data(data: &pb::Event, connector: &TcpStream) -> Result<(), io::Error> {
    let mut buf_writer = BufWriter::new(connector);
    let mut buf = vec![];
    match data.encode(&mut buf) {
        Ok(()) => {
            let buf_len = buf.len().to_string();
            println!("buf len: {buf_len:#?}")
        }
        Err(e) => {
            eprintln!("Encoding data error: {e:#?}")
        }
    }
    match buf_writer.write_all(&buf) {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("Error writing data to stream: {e:#?}");
        }
    }
    buf_writer.flush()
}

pub fn generate_data(counter: &i32) -> pb::Event {
    let device_id = 1337;
    let humidity = random_range(47-20..47+20);
    let temperature = random_range(27-20..27+20);
    let mut time_stamp = prost_types::Timestamp::default();
    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    time_stamp.seconds = duration.as_secs() as i64;
    time_stamp.nanos = duration.as_nanos() as i32;
    let mut esp_data = pb::event::EspData::default();
    esp_data.device_id = device_id;
    esp_data.temperature = temperature;
    esp_data.humidity = humidity;
    let mut event = pb::Event::default();
    event.event_id = *counter;
    event.read_time = Some(time_stamp);
    event.event_data = Some(esp_data);
    event
}