pub mod model;

use std::fs::*;
use std::io::Read;


pub fn read_json(path: &str) -> String {

    let mut file = File::open(path.to_string() + "sdlc-config.json")
        .expect("configuration file not found");

    let mut json_content = String::new();

    file.read_to_string(&mut json_content).expect("error reading configurations");

    json_content
}


pub fn get_task<'a>(arg: &'a str, task: &'a model::Tasks) -> &'a str {

    match arg {
        "run" => task.run.as_str(),
        "test" => task.test.as_str(),
        "build" => task.build.as_str(),
        _ => panic!("Invalid sdlc command")
    }
}