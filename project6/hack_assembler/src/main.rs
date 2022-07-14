use std::path::PathBuf;
use std::fs;

use clap::Parser as ArgParser;

mod tokens;
mod parser;
mod symbol_table;

use tokens::instructions::Code;
use tokens::instructions::Instruction;
use symbol_table::SymbolTable;

/// Assembly to Machine Code for the Hack Machine
#[derive(ArgParser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Source assembly file
    #[clap(value_parser)]
    source_file: PathBuf,

    /// Destination machine code file
    #[clap(value_parser)]
    destination_file: Option<PathBuf>,
}

fn process(contents: String) -> Option<(Vec<Instruction>, SymbolTable)> {
    let tokens = parser::parse(contents)?;
    let mut symbol_table = SymbolTable::new();
    let program = symbol_table.populate(tokens);

    Some((program, symbol_table))
}

fn assemble(instructions: Vec<Instruction>, symbol_table: SymbolTable) -> Option<Vec<String>> {
    instructions.iter().map(|instruction| instruction.encode(&symbol_table)).collect()
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.source_file).map_err(|_| "Error while reading the input file.")?;
    let (instructions, symbol_table) = process(contents).ok_or("Error while parsing the file.")?;
    let binary = assemble(instructions, symbol_table).ok_or("Error while generating the machine code.")?;

    let destination_file = match args.destination_file {
        Some(file) => file,
        None       => {
            let mut dest = args.source_file.clone();
            dest.set_extension("hack");
            dest
        },
    };

    fs::write(destination_file, binary.join("\n")).map_err(|_| "Error while writing to output file.")?;

    Ok(())
}
