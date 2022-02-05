// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt; //this crate automatically parse command line arguments by defining a struct.
use thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64, //because there can be a lot of ids
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>,
}

impl Records {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }

    fn into_vec(mut self) -> Vec<Record> {
        //drain() goes through the hashmap and drains out values to something else
        let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
        //sort_by_key() will sort records based on key provided in closures
        //here records will be sorted based on id
        records.sort_by_key(|rec| rec.id);
        records
    }

    //gives the next id
    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }
    //to search in the file
    fn search(&self, name: &str) -> Vec<&Record> {
        self.inner
            .values()
            //contains() checks is not an exact match function
            .filter(|rec| rec.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    //to remove a record
    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }

    //to edit record
    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.inner.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email,
            },
        );
    }
}

//function to write data files
fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) //when we truncate means we are completely erasing new file
        .open(file_name)?;

    file.write(b"id,name,email\n")?; //b is bytes

    //iterating vector
    for record in records.into_vec().into_iter() {
        //if email is in the record we return it as it is or we return empty string
        let email = match record.email {
            Some(email) => email,
            None => "".to_string(),
        };
        //formatting to save to file
        let line = format!("{},{},{}\n", record.id, record.name, email);
        //writing every line one by one to the file
        file.write(line.as_bytes())?;
    }
    //it makes a request to the system and it won't return until the system has successfully written
    //the data to the disk or it failed. Note: Always use flust after writing to file or the file will
    //not be properly written
    file.flush()?;
    Ok(())
}

//Now here we will get the data from csv file and format it according to our needs. This process is
//called parsing the data. So, here I will create a dedicated error type specifically for my parsing
//function:
#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String),
}

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (num, record) in records.split('\n').enumerate() {
        //enumerate() provides us an index number and return tuple index number and line
        if record != "" {
            match parse_record(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {}\n  > \"{}\"\n",
                            num + 1,
                            e,
                            record
                        );
                    }
                }
            }
        }
    }
    recs
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();
    //get() gets the very first field
    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?, //from_str_radix() converts a string to a number,10 is base
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match fields.get(1).filter(|name| **name != "") {
        //** means vector has first reference to the string and then when we do get() we have another so it has
        //2 &'s. So are dereferencing the 2 refrences using ** - Like C++ pinters
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");
    Ok(Record { id, name, email })
}

fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Project 2: Contact manager")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)] //for optional data
        email: Option<String>,
    },
    List {},
    Search {
        query: String,
    },
    Remove {
        id: i64,
    },
    Edit {
        id: i64,
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
        }
        //{..} is ignoring naything inside List
        Command::List { .. } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record);
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            let results = recs.search(&query);
            if results.is_empty() {
                println!("no records found");
            } else {
                for rec in results {
                    println!("{:?}", rec);
                }
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("record deleted");
            } else {
                println!("record not found");
            }
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("an error occured: {}", e);
    }
}
