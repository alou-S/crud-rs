use clap::{Args, Parser, Subcommand};
use colored::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new entry
    Create(CreateArgs),
    /// Read an existing entry
    Read(ReadArgs),
    /// Update an existing entry
    Update(CreateArgs),
    /// Delete an existing entry
    Delete(ReadArgs),
}

#[derive(Args)]
struct CreateArgs {
    name: String,
    value: String,
}

#[derive(Args)]
struct ReadArgs {
    name: String,
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new("data.toml");

    // The code below initializes the variable that contains the toml data
    let mut data = if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).unwrap()
    } else {
        toml::value::Table::new()
    };

    // Match to check which subcommand was used
    match &cli.command {
        Commands::Create(args) => {
            data.insert(args.name.clone(), toml::Value::String(args.value.clone()));
            println!("{}", "Created entry:".bold().green());
            println!("  Name : {}", args.name.green());
            println!("  Value: {}", args.value.green());
        }
        Commands::Read(args) => {
            if let Some(value) = data.get(&args.name) {
                println!("{}", "Read entry:".bold().green());
                println!("  Name : {}", args.name.green());
                println!("  Value: {}", value.to_string().green());
            } else {
                println!("{}", "Entry not found:".bold().red());
                println!("  Name : {}", args.name.red());
            }
        }
        Commands::Update(args) => {
            if data.contains_key(&args.name) {
                data.insert(args.name.clone(), toml::Value::String(args.value.clone()));
                println!("{}", "Updated entry:".bold().green());
                println!("  Name : {}", args.name.green());
                println!("  Value: {}", args.value.green());
            } else {
                println!("{}", "Entry not found:".bold().red());
                println!("  Name : {}", args.name.red());
            }
        }
        Commands::Delete(args) => {
            if data.remove(&args.name).is_some() {
                println!("{}", "Deleted entry:".bold().green());
                println!("  Name : {}", args.name.green());
            } else {
                println!("{}", "Entry not found:".bold().red());
                println!("  Name : {}", args.name.red());
            }
        }
    }

    let mut file = File::create(path).unwrap();
    file.write_all(toml::to_string(&data).unwrap().as_bytes())
        .unwrap();
}
