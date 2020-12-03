use clap::{App, Arg};
use serde::Deserialize;
use std::env::current_dir;
use std::fs::File;
use std::io::Write;

/*
What I want:
gig [keyword] // prints to output
gig --list // lists out all options with keywords
gig [keyword] --write // writes .gitignore; errors out if gitignore is there
gig [keyword] --write-force // writes .gitignore; overwrites if there
*/

const REPO_URL: &str = "https://api.github.com/repos/github/gitignore/contents";
const GITIGNORE_FILE: &str = ".gitignore";

fn main() {
    let matches = App::new("gitignore-cli")
        .about("Generates a commonly-defined gitignore")
        .version("0.1")
        .author("Milan <hey@mdaverde.com>")
        .arg(
            Arg::with_name("keyword")
                .help("<<TBD>>")
                .conflicts_with("list")
                .required_unless("list"),
        )
        .arg(
            Arg::with_name("list")
                .help("<<TBD>>")
                .short("l")
                .long("list"),
        )
        .arg(
            Arg::with_name("write")
                .help("")
                .short("w")
                .long("write")
                .requires("keyword"),
        )
        .arg(
            Arg::with_name("write-force")
                .help("")
                .long("write-force")
                .requires("keyword"),
        )
        .get_matches();

    if let Some(keyword) = matches.value_of("keyword") {
        if matches.is_present("write") || matches.is_present("write-force") {
            let cwd = current_dir().unwrap().join(GITIGNORE_FILE); // TODO: handle error
            if cwd.exists() && !matches.is_present("write-force") {
                eprintln!("Error: .gitignore exists in current working directory. Use `--write-force` to overwrite");
            } else {
                let gitignore_contents = get_gitignore(keyword);
                println!("{}", gitignore_contents);
                File::create(cwd)
                    .unwrap()
                    .write(gitignore_contents.as_bytes())
                    .unwrap();
                println!("Writing {}.gitignore to .gitignore...", "Swift"); // TODO: handle param
            }
        } else {
            let gitignore_contents = get_gitignore(keyword);
            println!("{}", gitignore_contents);
        }
    } else if matches.is_present("list") {
        let list: Vec<String> = list_git_contents().map(|content| content.path).collect();
        println!("To output a gitignore you can write `gitignore [keyword]` (i.e. `gitignore actionscript`)");
        println!(
            "All possible .gitignores ({}): \n\t{}",
            list.len(),
            list.join("\n\t")
        );
    }
}

struct GitContent {
    path: String,
    download_url: String,
}

impl GitContent {
    fn matches_keyword(&self, keyword: &str) -> bool {
        let lowercase_keyword = keyword.to_lowercase();
        self.path
            .replace(GITIGNORE_FILE, "")
            .to_lowercase()
            .contains(&lowercase_keyword)
    }
}

#[derive(Deserialize)]
struct RawGitContent {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    download_url: Option<String>,
}

fn get_gitignore(keyword: &str) -> String {
    let contents = list_git_contents();
    let matched_contents: Vec<GitContent> = contents
        .filter(|content| content.matches_keyword(keyword))
        .collect();

    // TODO: handle case where not one or more than one is found
    let len = matched_contents.len();
    if len < 1 {
        eprintln!("Couldn't find one");
        std::process::exit(1);
    } else if len > 1 {
        eprintln!("Shouldn't be the case");
    }

    let chosen: &GitContent = &matched_contents[0];
    let response = ureq::get(&chosen.download_url).call(); // TODO: unwrap
    return response.into_string().unwrap();
    // if response.ok() {
    //     return response.into_string();
    // }
    // Err(response.into_synthetic_error()) // TODO: is this what we want?
}

fn list_raw_git_contents() -> Vec<RawGitContent> {
    let response = ureq::get(REPO_URL).call();
    return response
        .into_json_deserialize::<Vec<RawGitContent>>()
        .unwrap(); // TODO: handle error
}

fn list_git_contents() -> impl Iterator<Item = GitContent> {
    let raw_contents = list_raw_git_contents();
    raw_contents.into_iter().filter_map(|raw| {
        let path = raw.path?;
        if path.len() < 1 || !path.ends_with(".gitignore") {
            return None;
        }

        let download_url = raw.download_url?;
        if download_url.len() < 1 {
            return None;
        }

        Some(GitContent { path, download_url })
    })
}
