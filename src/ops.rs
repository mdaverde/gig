use serde::{Serialize, Deserialize};
use std::io::Write;

use crate::err::CliError;

const REPO_URL: &str = "https://api.github.com/repos/github/gitignore/contents";
const GITIGNORE_FILE: &str = ".gitignore";

#[derive(Serialize, Deserialize, Debug)]
struct RawGitContent {
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    download_url: Option<String>,
}

impl RawGitContent {
    fn list() -> Result<Vec<RawGitContent>, CliError> {
        let response = ureq::get(REPO_URL).call()?;
        Ok(response.into_json::<Vec<RawGitContent>>()?)
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

    fn list() -> Result<impl Iterator<Item=GitContent>, CliError> {
        let raw_contents = RawGitContent::list()?;
        let iter = raw_contents.into_iter().filter_map(|raw| {
            let path = raw.path.unwrap();
            if path.len() < 1 || !path.ends_with(".gitignore") {
                return None;
            }

            let download_url = raw.download_url.unwrap();
            if download_url.len() < 1 {
                return None;
            }

            Some(GitContent { path, download_url })
        });
        Ok(iter)
    }

    fn get(keyword: &str) -> Result<String, CliError> {
        let contents = Self::list()?;
        let matched_contents: Vec<GitContent> = contents
            .filter(|content| content.matches_keyword(keyword))
            .collect();

        // TODO: handle case where not one or more than one is found
        let len = matched_contents.len();
        if len < 1 {
            return Err(CliError::GitIgnoreNotFound(keyword.into()));
        } else if len > 1 {
            // TODO: clarify from the user
        }

        let chosen: &GitContent = &matched_contents[0];
        let response = ureq::get(&chosen.download_url).call()?;
        Ok(response.into_string()?)
    }
}

pub fn list_all() -> Result<(), CliError> {
    let list: Vec<String> = GitContent::list()?.map(|content| content.path).collect();
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
    let contents = GitContent::get(keyword)?;
    println!("{}", contents);
    Ok(())
}

pub fn write(keyword: &str, force: bool) -> Result<(), CliError> {
    use std::{fs, env};

    let cwd = env::current_dir()?.join(GITIGNORE_FILE);
    if cwd.exists() && !force {
        return Err(CliError::OverwriteFile);
    } else {
        let contents = GitContent::get(&keyword)?;
        println!("Writing {} gitignore to .gitignore...", keyword);
        fs::File::create(cwd)?.write(contents.as_bytes())?;
    }
    Ok(())
}
