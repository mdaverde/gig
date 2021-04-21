use clap;

pub enum Op {
    ListAll,
    PrintSingle(String),
    Write(String),
    WriteForce(String),
}

pub fn parse_args() -> Op {
    let matches = clap::App::new(crate::CLI_NAME)
        .about("Generates a commonly-defined .gitignore")
        .version(clap::crate_version!())
        .author("Milan <hey@mdaverde.com>")
        .arg(
            clap::Arg::with_name("keyword")
                .help("Used to specify .gitignore to generate")
                .conflicts_with("list")
                .required_unless("list"),
        )
        .arg(
            clap::Arg::with_name("list")
                .help("Lists all possible .gitignores")
                .short("l")
                .long("list"),
        )
        .arg(
            clap::Arg::with_name("write")
                .help("Writes to .gitignore in current working directory (does NOT overwrite if already exists)")
                .short("w")
                .long("write")
                .requires("keyword"),
        )
        .arg(
            clap::Arg::with_name("write-force")
                .help("Overwrites .gitignore in current working directory")
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