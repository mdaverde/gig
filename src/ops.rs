use crate::err::CliError;
use std::io::Write;
use serde::{Deserialize};

const REPO_URL: &str = "https://api.github.com/repos/github/gitignore/contents";
const GITIGNORE_FILE: &str = ".gitignore";

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
        // TODO: clarify from the user
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

fn list_git_contents() -> impl Iterator<Item=GitContent> {
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

pub fn list_all() -> Result<(), CliError> {
    let list: Vec<String> = list_git_contents().map(|content| content.path).collect();
    println!(
        "To output a gitignore you can write `{} [keyword]` (i.e. `{} actionscript`)",
        crate::CLI_NAME, crate::CLI_NAME
    );
    println!(
        "All possible .gitignores ({}): \n\n{}",
        list.len(),
        list.join("\n")
    );
    Ok(())
}

pub fn print_single(keyword: &str) -> Result<(), CliError> {
    let gitignore_contents = get_gitignore(keyword);
    println!("{}", gitignore_contents);
    Ok(())
}

pub fn write(keyword: &str, force: bool) -> Result<(), CliError> {
    use std::{fs, env};

    let cwd = env::current_dir()?.join(GITIGNORE_FILE);
    if cwd.exists() && !force {
        return Err(CliError::OverwriteFile);
    } else {
        let gitignore_contents = get_gitignore(&keyword);
        println!("{}", gitignore_contents);
        fs::File::create(cwd)?.write(gitignore_contents.as_bytes())?;
        println!("Writing {} gitignore to .gitignore...", keyword);
    }
    Ok(())
}
