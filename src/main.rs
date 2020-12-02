use clap::{App, Arg};

/*
 What I want:
 gitignore [keyword] // prints to output
 gitignore --list // lists out all options with keywords
 gitignore [keyword] --write // writes .gitignore; errors out if gitignore is there
 gitignore [keyword] --write-force // writes .gitignore; overwrites if there
 gitignore [keyword] --append-force // appends to .gitignore
 */

fn main() {
    let matches = App::new("gitignore-cli")
        .about("Generates a commonly-defined gitignore")
        .version("0.1")
        .author("Milan <hey@mdaverde.com>")
        .arg(
            Arg::with_name("keyword")
                .help("<<TBD>>")
                .conflicts_with("list")
                .required_unless("list")
        )
        .arg(
            Arg::with_name("list")
                .help(("<<TBD>>"))
                .short("l")
                .long("list")
        )

        .get_matches();

    let keyword = matches.value_of("keyword").unwrap();

    println!("keyword {}", keyword);

}
