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
                .required(true)
                .value_name("TEMPLATE_NAME")
                .help("Name of template to use"),
        )
        .arg(
            Arg::with_name("title")
                .long("title")
                .value_name("CONTENT_TITLE")
                .help(
                    "The title of the content (e.g., title of an HTML page)",
                ),
        )
        .get_matches();

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
