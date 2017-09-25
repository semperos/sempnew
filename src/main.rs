extern crate clap;
extern crate sempnew;

use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Semperos new-ing projects or files")
        .version("0.1.0")
        .author("Daniel Gregoire <daniel.l.gregoire@gmail.com>")
        .about("Project dedicated to generating templates and files")
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .required(false)
                .value_name("TEMPLATE_NAME")
                .help("Name of template to use"),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .value_name("PROJECT_NAME")
                .help("The name of the project and top-level directory"),
        )
        .arg(
            Arg::with_name("description")
                .long("description")
                .short("d")
                .value_name("DESCRIPTION")
                .help("Human-readable description of the project"),
        )
        .arg(
            Arg::with_name("lib_name")
                .long("lib-name")
                .value_name("LIBRARY_NAME")
                .help(
                    "The name to give the library this project will generate. Will use sanitized name value by default.",
                ),
        )
        .arg(
            Arg::with_name("bin_name")
                .long("bin-name")
                .value_name("BINARY_NAME")
                .help(
                    "The name to give the binary executable this project will generate. Will use sanitized name value by default.",
                ),
        )
        .arg(
            Arg::with_name("title")
                .long("title")
                .value_name("CONTENT_TITLE")
                .help("The title of the content (e.g., title of an HTML page)"),
        )
        .arg(
            Arg::with_name("list_templates")
                .long("list")
                .short("l")
                .takes_value(false)
                .help("Prints a list of available templates."),
        )
        .get_matches();

    if matches.is_present("list_templates") {
        println!("Available templates:");
        sempnew::list_templates();
        process::exit(0);
    }

    match matches.value_of("template") {
        Some(template) => {
            match template {
                "html" | "html5" => {
                    let title = matches.value_of("title");
                    if let Err(e) = sempnew::html5_template(title) {
                        eprintln!("[ERROR] HTML template generation failed: {:?}", e);
                        process::exit(1);
                    }
                }
                "rust" | "rust-cli" => {
                    let name = matches.value_of("name");
                    let description = matches.value_of("description");
                    let lib_name = matches.value_of("lib_name");
                    let bin_name = matches.value_of("bin_name");
                    if let Err(e) = sempnew::rust_cli_template(name, description, lib_name, bin_name) {
                        eprintln!("[ERROR] Rust CLI project generation failed: {:?}", e);
                        process::exit(1);
                    }
                }
                _ => {
                    eprintln!("{}", matches.usage());
                    process::exit(1);
                }
            }
        }
        None => {
            eprintln!("{}", matches.usage());
            process::exit(1);
        }
    }
}
