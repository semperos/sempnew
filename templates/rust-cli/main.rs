extern crate clap;
extern crate {{lib-name}};

use clap::{App, Arg};

fn main() {
    let matches = App::new("{{name}}")
        .version("0.1.0")
        .author("Daniel Gregoire <daniel.l.gregoire@gmail.com>")
        .about("{{description}}")
        .arg(
            Arg::with_name("example_arg")
                .short("e")
                .long("example")
                .value_name("EXAMPLE_INTEGER_VALUE")
                .required(true)
                .help("An example command-line argument.")
        )
        .get_matches();

    if let Some(example_value) = matches.value_of("example_arg") {
        {{lib-name}}::example(Some(example_value.parse::<i32>().unwrap()));
    } else {
        {{lib-name}}::example(None);
    }
}
