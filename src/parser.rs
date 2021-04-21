use clap;

pub enum Op {
    ListAll,
    PrintSingle(String),
    Write(String),
    WriteForce(String),
}

pub fn parse_args() -> Op {
    let matches = clap::App::new(crate::CLI_NAME)
        .about("Generates a commonly-defined gitignore")
        .version("0.1")
        .author("Milan <hey@mdaverde.com>")
        .arg(
            clap::Arg::with_name("keyword")
                .help("<<TBD>>")
                .conflicts_with("list")
                .required_unless("list"),
        )
        .arg(
            clap::Arg::with_name("list")
                .help("<<TBD>>")
                .short("l")
                .long("list"),
        )
        .arg(
            clap::Arg::with_name("write")
                .help("")
                .short("w")
                .long("write")
                .requires("keyword"),
        )
        .arg(
            clap::Arg::with_name("write-force")
                .help("")
                .long("write-force")
                .requires("keyword"),
        )
        .get_matches();

    if let Some(keyword) = matches.value_of("keyword") {
        if matches.is_present("write") || matches.is_present("write-force") {
            if matches.is_present("write") {
                return Op::Write(keyword.into());
            } else if matches.is_present("write-force") {
                return Op::WriteForce(keyword.into());
            }
        } else {
            return Op::PrintSingle(keyword.into());
        }
    } else if matches.is_present("list") {
        return Op::ListAll;
    }

    todo!()
}