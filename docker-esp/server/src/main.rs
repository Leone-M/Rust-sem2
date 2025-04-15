use postgres::Client;
use server::{db_connect, db_create_table, establish_connect, handle_connection};

fn main() {
    let listener_res = establish_connect();
    let mut db_client: Client;
    
    let database_client_connection = db_connect();
    match database_client_connection {
        Ok(client) => {
            println!("Connected to database successfully!");
            db_client = client;
        }
        Err(e) => {
            eprintln!("Can not establish connect to database: {e:#?}");
            panic!()
        }
    }
    match db_create_table(&mut db_client) {
        Ok(_) => {
            println!("Table created successfully!");
        }
        Err(_) => {panic!()}
    }
    match &listener_res {
        Ok(suc_response) => {
            println!("Connection established successfully");
            // keep_connection = false;
            for stream in suc_response.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream, &mut db_client);
            }
        },
        Err(e) => {
        eprintln!("Connection can not be established successfully: {e:#?}");
        panic!()}
    }
}
