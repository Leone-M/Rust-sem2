use std::thread::sleep;
use std::time::Duration;
use client::{generate_data, send_data};

fn main() {
    let mut counter = 0;
    
    loop {
        let tcp_connection = client::establish_connection();
        match &tcp_connection {
            Ok(connection) => {
                let test_data = generate_data(&counter);
                counter = counter + 1;
                
                match send_data(&test_data, connection) {
                    Ok(()) => {
                        println!("Data sent")
                    }
                    Err(e) => {
                        eprintln!("Data can not be sent: {e:#?}")
                    }
                }
                
                sleep(Duration::from_secs(10));
            }
            Err(e) => {
                eprintln!("Connection can not be established successfully: {e:#?}");
                sleep(Duration::from_secs(3));
            }
        }
    }
}
