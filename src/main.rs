mod err;
mod ops;
mod parser;

const CLI_NAME: &str = "gig";

fn main() {
    use parser::Op;

    match parser::parse_args() {
        Op::ListAll => {
            if let Err(err) = ops::list_all() {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Op::PrintSingle(keyword) => {
            if let Err(err) = ops::print_single(&keyword) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Op::Write(keyword) => {
            if let Err(err) = ops::write(&keyword, false) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Op::WriteForce(keyword) => {
            if let Err(err) = ops::write(&keyword, true) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
