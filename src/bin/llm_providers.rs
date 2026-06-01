use llm_providers::export_pararouter_registry;
use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("usage: llm_providers export --format pararouter [--output FILE]");
        process::exit(1);
    };

    if command != "export" {
        eprintln!("unknown command: {command}");
        eprintln!("usage: llm_providers export --format pararouter [--output FILE]");
        process::exit(1);
    }

    let mut format = None;
    let mut output = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" => {
                format = args.next();
            }
            "--output" | "-o" => {
                output = args.next();
            }
            other => {
                eprintln!("unknown argument: {other}");
                process::exit(1);
            }
        }
    }

    if format.as_deref() != Some("pararouter") {
        eprintln!("only --format pararouter is supported");
        process::exit(1);
    }

    let registry = export_pararouter_registry();
    let json = serde_json::to_string_pretty(&registry).unwrap_or_else(|err| {
        eprintln!("failed to serialize export: {err}");
        process::exit(1);
    });

    match output {
        Some(path) => {
            std::fs::write(&path, json).unwrap_or_else(|err| {
                eprintln!("failed to write {path}: {err}");
                process::exit(1);
            });
        }
        None => {
            io::stdout()
                .write_all(json.as_bytes())
                .unwrap_or_else(|err| {
                    eprintln!("failed to write stdout: {err}");
                    process::exit(1);
                });
            println!();
        }
    }
}
