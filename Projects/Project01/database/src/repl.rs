use std::io::{self, Write};

use miette::{IntoDiagnostic, Report, Result};

use database::{
    QueryParser,
    core::{Database, DatabaseKey},
    execution::build_execute_command,
};

/// Starts and runs the interactive Read-Eval-Print Loop (REPL).
pub fn run_repl<K: DatabaseKey>() -> Result<()> {
    let mut database = Database::<K>::default();
    let parser = QueryParser;
    let stdin = io::stdin();
    let mut query_buffer = String::new();

    loop {
        if query_buffer.is_empty() {
            print!("> ");
        } else {
            print!("-> ");
        }
        io::stdout().flush().unwrap();

        let mut line_input = String::new();
        match stdin.read_line(&mut line_input).into_diagnostic()? {
            0 => {
                break;
            }
            _ => {
                query_buffer.push_str(&line_input);
                if query_buffer.trim().is_empty() {
                    query_buffer.clear();
                    continue;
                }

                if query_buffer.trim().ends_with(';') {
                    parse_and_execute(&parser, &mut database, query_buffer.trim());
                    query_buffer.clear();
                } else {
                    continue;
                }
            }
        }
    }

    Ok(())
}

/// A helper function to orchestrate the full query processing pipeline.
///
/// This function takes a raw query string and:
/// 1.  Calls the `parser` to build an AST.
/// 2.  Calls `build_execute_command` to validate the AST and create an executable.
/// 3.  Calls `.execute()` on the command.
///
/// All results (success or error) are printed directly to `stdout` or `stderr`.
/// Errors at any stage are printed, but do not stop the REPL.
fn parse_and_execute<K: DatabaseKey>(
    parser: &QueryParser,
    database: &mut Database<K>,
    query: &str,
) {
    match parser.parse_query(query) {
        Ok(ast) => match build_execute_command(database, ast) {
            Ok(mut executable_command) => {
                println!("Executing query...");
                match executable_command.execute() {
                    Ok(result) => {
                        println!("{result}");
                    }
                    Err(e) => eprintln!("{}", Report::new(e)),
                }
            }
            Err(e) => eprintln!("{}", Report::new(e)),
        },
        Err(e) => eprintln!("{}", Report::new(e)),
    }
}
