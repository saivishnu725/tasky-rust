use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    active: bool,
    title: String,
    description: String,
    // TODO: use an appropriate type for Date
    updated_on: String,
    created_on: String,
    // TODO: placeholder for when multiple users can co-exist
    #[serde(default)] // default if empty
    created_by: String,
    #[serde(default)] // default if empty
    updated_by: String,
}

fn load_tasks(file_path: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);

    let mut tasks = Vec::new();
    for result in csv_reader.deserialize() {
        let task: Task = result?;
        tasks.push(task);
    }

    Ok(tasks)
}

fn main() -> Result<(), std::io::Error> {
    // will be in a config file later
    // or heck, it'll be a database so no need for a csv file at all
    let file_path: &str = "tmp/tasks.csv";
    let tasks = load_tasks(file_path).expect("Failed comprehending the csv: ");
    println!("{:?}", tasks);
    println!("first: {:?}", tasks[1]);
    Ok(())
}
