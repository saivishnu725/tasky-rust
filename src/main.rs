use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    active: bool,
    title: String,
    description: String,
    // TODO: use an appropriate type for Date
    updated_on: String,
    created_on: String,
    // TODO: placeholder for when multiple users can co-exist
    // created_by: String,
    // updated_by: String
}

fn main() -> Result<(), std::io::Error> {
    let file_path: &str = "tmp/tasks.json";
    let data_file = fs::read_to_string(file_path).expect("Error reading file: ");
    let mut data: Task = serde_json::from_str(&data_file).unwrap();

    println!("{:?}", data);

    if data.active {
        data.active = false;
    } else {
        data.active = true;
    }
    fs::write(file_path, serde_json::to_string_pretty(&data).unwrap()).expect("error writing: ");

    Ok(())
}
