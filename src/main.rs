extern crate clap;

pub mod app;
mod cli_parsing;
pub mod algorithms;

use clap::{ Arg, App, SubCommand, ArgMatches };
use algorithms::generate_rand_number::GenerateRandNumber;
use cli_parsing::CommandNames;
use std::result;

static APP_NAME: &str = "Cookbook";
static VERSION : &str = "0.1.0";

/// This is a comment
fn main() {
    let modules = prepare_modules_list();
    let command_names = CommandNames::create();
    let (help_message, arg_matches) = prepare_cli_parsing(&command_names);

    match handle_arguments(&modules, &command_names, arg_matches, help_message) {
        Ok(module) => {
            let run_result = match module {
                Some(m) => m.run(),
                None => Ok(())
            };
            if let Err(message) = run_result {
                eprintln!("{}", message)
            }
        }
        Err(message) => eprintln!("{}", message)
    }
}

fn prepare_cli_parsing<'a>(command_names: &CommandNames<'a>) -> (String, ArgMatches<'a>) {
    let mut app = App::new("Cookbook")
        .version("0.1.0")
        .about("Test application for various crates")
        .arg(Arg::with_name(command_names.module_command_name)
            .short(command_names.module_command_name_short)
            .long(command_names.module_command_name_long)
            .takes_value(true)
            .required(false)
            .help("Type name of the module to run"))
        .subcommand(SubCommand::with_name(command_names.list_command_name)
            .about("Lists available modules"));

    let mut help_message = Vec::new();
    app.write_long_help(&mut help_message).unwrap();
    let help_message_string = String::from_utf8(help_message).unwrap();

    let matches = app.get_matches();

    (help_message_string, matches)
}

fn handle_arguments<'a>(modules: &'a Vec<Box<app::CliModule>>, command_names: &CommandNames,
                    args: ArgMatches, help_message: String)
    -> result::Result<Option<&'a Box<app::CliModule>>, String> {
    let module = args.value_of(command_names.module_command_name);
    match args.subcommand_matches(command_names.list_command_name) {
        Some(_) => {
            print_header();
            println!("List of available modules:");
            for module in modules.iter() {
                println!("\t{}\t\t{}", module.get_name(), module.get_desc());
            }
            Ok(None)
        },
        _ => match module {
            Some(m) => {
                print_header();
                let module_candidate = modules.iter()
                    .find(|e| e.get_name().to_lowercase() == m.to_lowercase());

                match module_candidate {
                    Some(info) => Ok(Some(&info)),
                    None => Err(format!("No module '{}' found", m.to_string()))
                }
            }
            None => Err(help_message)
        }
    }
}

fn prepare_modules_list() -> Vec<Box<app::CliModule>> {
    let mut list: Vec<Box<app::CliModule>> = Vec::new();
    list.push(Box::new(GenerateRandNumber::create()));
    list
}

fn print_header() {
    println!("{} [Version {}]\n", APP_NAME, VERSION);
}