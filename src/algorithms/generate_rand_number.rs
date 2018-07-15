use app::CliModule;
use std::error;

pub struct GenerateRandNumber<'a> {
    name: &'a str,
    desc: &'a str
}

impl<'a> GenerateRandNumber<'a> {
    pub fn create() -> GenerateRandNumber<'a> {
        GenerateRandNumber {
            name: "Alg1",
            desc: "Generate random numbers"
        }
    }
}

impl<'a> CliModule for GenerateRandNumber<'a> {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_desc(&self) -> String {
        self.desc.to_string()
    }

    fn run(&self) -> Result<(), Box<error::Error>> {
        println!("Hello from Alg1!");
        Ok(())
    }
}