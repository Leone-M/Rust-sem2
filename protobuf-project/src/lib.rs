use std::fs;
use std::io::{BufReader, Read, Write, BufWriter};
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
use prost::Message;
use prost_types::Timestamp;
use std::time;

const DB_FILE_PATH: &str = "addressbook.db";

mod pb {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // The first value in the return value of env::args is the name of the program.
        // parse `add` and `list` commands.
        let command = match args.next(){
            Some(command) => command,
            None => return Err("Command not found"),
        };

        // parse `--arg value`
        let mut params: HashMap<String, String> = HashMap::new();
        while let Some(arg) = args.next(){
            if arg.starts_with("--"){
                match args.next(){
                    Some(param) => params.insert(arg, param),
                    None => return Err("Missing parameter after --arg"),
                };
            } else { return Err("Expected arg starts with --"); }
        };
        Ok(Config { command, params })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = open_db_file(DB_FILE_PATH);
    match config.command.as_ref() {
        "add" => {
            if config.params["--kind"] == "per" || config.params["--kind"] == "person"{
                add_person(&mut f, &config.params["--name"], &config.params["--email"],  &config.params["--phone"], &config.params["--type"]); // Will panic
            }
            else if config.params["--kind"] == "cie" || config.params["--kind"] == "company"{
                add_company(&mut f, &config.params["--name"], &config.params["--email"], &config.params["--dep"], &config.params["--phone"], &config.params["--type"]);
            }
            Ok(())
        },
        "list" => {
            list_contacts(&mut f, &config.params["--redact"]); // Will panic
            Ok(())
        },
        _ => Err("Command not found")?,
    }
}

pub struct Config {
    pub command: String,
    pub params: HashMap<String, String>
}

pub fn read_from_db(f: &fs::File) -> pb::AddressBook {
    let mut buf_reader = BufReader::new(f);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents).unwrap();
    pb::AddressBook::decode(contents.as_slice()).unwrap()
}

fn write_to_db(f: &mut fs::File, book: pb::AddressBook) {
    let mut buf_writer = BufWriter::new(f);
    let contents = book.encode_to_vec();
    buf_writer.write_all(&contents).unwrap();
    buf_writer.flush().unwrap();
}

pub fn open_db_file(file_path: &str) -> fs::File {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap()
}

fn str_to_phone_type(s: &str) ->i32 {
    match s {
        "home" => 2,
        "mobile" => 1,
        "work" => 3,
        _ => 0,
    }
}
fn str_to_department(s: &str) -> i32 {
    match s {
        "hr" => 1,
        "cs" => 2,
        _ => 0
    }
}

fn redact_private_info(cont: &mut pb::Contact) -> pb::Contact {
    let redacted_contact = if let pb::contact::Kind::Company(company) = cont.clone().kind.unwrap() {
        let mut redacted_company = company.clone();
        let mut redacted_phones = Vec::new();
        let mut redacted_emails = Vec::new();
        for mut phone_number in company.phones {
            phone_number.number = "*".repeat(phone_number.number.len());
            redacted_phones.push(phone_number);
        }
        for mut email_address in company.emails {
            email_address.email = "*".repeat(email_address.email.len());
            redacted_emails.push(email_address)
        }
        redacted_company.emails = redacted_emails;
        redacted_company.phones = redacted_phones;
        let mut redacted_contact = cont.clone();
        redacted_contact.kind = Some(pb::contact::Kind::Company(redacted_company));
        redacted_contact
    } else if let pb::contact::Kind::Person(person) = cont.clone().kind.unwrap() {
        let mut redacted_person = person.clone();
        let mut redacted_phones = Vec::new();
        let redacted_email = "*".repeat(person.email.len());
        for mut phone_number in person.phones {
            phone_number.number = "*".repeat(phone_number.number.len());
            redacted_phones.push(phone_number);
        }
        redacted_person.phones = redacted_phones;
        redacted_person.email = redacted_email;
        let mut redacted_contact = cont.clone();
        redacted_contact.kind = Some(pb::contact::Kind::Person(redacted_person));
        redacted_contact
    } else {
        // делаю так потому что if let требует полной определенности
        // однако иных вариантов чем перечисленные выше не предвидется
        // поэтому данный рукав просто заглушка
        cont.clone()
    };
    redacted_contact
}
fn add_person(f: &mut fs::File, name: &str, email: &str, phone: &str, phone_type: &str) {
    let mut book = read_from_db(f);
    let mut person: pb::Person;
    if book.contacts.contains_key(name) {
        // If Kind exists
        let kind = book.contacts.get(name).unwrap().kind.clone().unwrap();
        match kind {
            pb::contact::Kind::Person(p) => { person = p },
            pb::contact::Kind::Company(_) => {panic!("Company")},
        }
    }
    else {
        person = pb::Person::default();
    }
    person.email = email.to_string();
    let mut nb = pb::person::PhoneNumber::default();
    nb.number = phone.to_string();
    nb.r#type = str_to_phone_type(phone_type);
    person.phones.push(nb);

    let mut contact = pb::Contact::default();
    let mut update_ts = Timestamp::default();
    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    update_ts.seconds = duration.as_secs() as i64;
    update_ts.nanos = duration.subsec_nanos() as i32;

    contact.last_updated = Some(update_ts);
    contact.kind = Some(pb::contact::Kind::Person(person));
    book.contacts.insert(String::from(name), contact);

    write_to_db(f, book);
}

fn add_company(f: &mut fs::File, name: &str, email: &str, email_dep: &str, phone: &str, phone_dep: &str) {
    let mut book = read_from_db(f);

    let mut company: pb::Company;
    if book.contacts.contains_key(name) {
        // If Kind exists
        let kind = book.contacts.get(name).unwrap().kind.clone().unwrap(); // .clone
        match kind {
            pb::contact::Kind::Company(comp) => {company = comp },
            pb::contact::Kind::Person(_) => { panic!("Person") },
        }
    }
    else {
        company = pb::Company::default();
    }

    let mut addr = pb::company::EmailAddress::default();
    addr.email = email.to_string();
    addr.department = str_to_department(email_dep);
    company.emails.push(addr);

    let mut nb = pb::company::PhoneNumber::default();
    nb.number = phone.to_string();
    nb.department = str_to_department(phone_dep);
    company.phones.push(nb);

    let mut contact = pb::Contact::default();
    let mut update_ts = Timestamp::default();
    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    update_ts.seconds = duration.as_secs() as i64;
    update_ts.nanos = duration.subsec_nanos() as i32;

    contact.last_updated = Some(update_ts);
    contact.kind = Some(pb::contact::Kind::Company(company));
    book.contacts.insert(String::from(name), contact);

    write_to_db(f, book);
}

fn list_contacts(f: &mut fs::File, _redact: &str) {
    let book = read_from_db(f);
    let mut keys: Vec<&String>  = book.contacts.keys().collect();
    keys.sort();
    for name in keys {
        let contact = book.contacts.get(name).unwrap();
        // TODO if redact
        if _redact == "true" {
            let redacted_contact = redact_private_info(&mut contact.clone());
            println!("name: {}", name);
            println!("last_updated: {:?}", contact.last_updated.unwrap());
            println!("{:#?}", redacted_contact);
            println!("-----------------------");
        } else {
            println!("name: {}", name);
            println!("last_updated: {:?}", contact.last_updated.unwrap());
            println!("{:#?}", contact);
            println!("-----------------------");
        }
    }
}
