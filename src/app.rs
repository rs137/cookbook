use std::error;

pub trait CliModule {
    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;
    fn run(&self) -> Result<(), Box<error::Error>>;
}