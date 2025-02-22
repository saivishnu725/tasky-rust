use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::vec;

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

fn add_task(tasks: &mut Vec<Task>, new_task: Task) -> Result<(), Box<dyn Error>> {
    tasks.push(new_task);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // will be in a config file later
    // or heck, it'll be a database so no need for a csv file at all
    let file_path: &str = "tmp/tasks.csv";
    let mut tasks = load_tasks(file_path).expect("Failed comprehending the csv: ");
    println!("{:?}", tasks);
    println!("first: {:?}", tasks[1]);

    let new_task = Task {
        active: true,
        title: "string3".to_string(),
        description: "string3".to_string(),
        updated_on: "2024-04-20".to_string(),
        created_on: "2024-02-20".to_string(),
        created_by: "21".to_string(),
        updated_by: "12".to_string(),
    };

    match add_task(&mut tasks, new_task) {
        Ok(_) => println!("added successfully."),
        Err(e) => println!("error adding task {e}"),
    }

    println!("tasks after adding: {:?}", tasks);
    Ok(())
}
