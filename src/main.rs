use ansi_term::Colour;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::error::Error;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "github-cli", about = "A GitHub client written in Rust")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Trending {
        #[structopt(default_value, short, long = "lang")]
        language: String,

        #[structopt(default_value, short)]
        period: String,
    },
}

// #[derive(Serialize, Deserialize)]
struct Repository {
    name: String,
    description: Option<String>,
    language: Option<String>,
}

fn get_trending_url(language: &str, period: &str) -> String {
    format!(
        "https://github.com/trending{}{}",
        if language.is_empty() {
            "".to_string()
        } else {
            format!("/{}", language)
        },
        if period.is_empty() {
            "".to_string()
        } else {
            format!("?since={}", period)
        }
    )
}

fn get_trending_repositories(
    language: &str,
    period: &str,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    let url = get_trending_url(language, period);

    let body = reqwest::blocking::get(&url)?.text()?;

    // let mut body = String::new();

    // {
    //     let mut file = File::open("./trending.html")?;

    //     file.read_to_string(&mut body);
    // }

    // {
    //     let mut file = File::create("./trending.html")?;

    //     file.write_all(body.as_bytes())?;
    // }

    let mut repositories = Vec::new();

    let document = Document::from(body.as_str());

    for node in document.find(Class("Box-row")) {
        let name = node
            .find(Class("h3").child(Name("a")))
            .next()
            .ok_or("could not find repository name")?;

        let description = node.find(Class("col-9")).next();

        let language = node
            .find(Class("f6").child(Class("ml-0")).child(Name("span")))
            .nth(1);

        repositories.push(Repository {
            name: "https://github.com".to_string() + name.attr("href").unwrap(),
            description: description.map(|el| el.text().trim().to_string()),
            language: language.map(|el| el.text().trim().to_string()),
        });
    }

    Ok(repositories)
}

fn main() {
    let opt = Cli::from_args();

    println!("{:?}", opt);

    match opt.command {
        Command::Trending { language, period } => {
            let r = get_trending_repositories(&language, &period);

            match r {
                Ok(repositories) => {
                    println!("length: {}", repositories.len());

                    for repo in repositories {
                        println!(
                            "[{}] {}\n- {}\n",
                            Colour::RGB(150, 150, 150)
                                .paint(repo.language.unwrap_or("unknown".to_string())),
                            Colour::Green.paint(repo.name),
                            Colour::Yellow
                                .paint(repo.description.unwrap_or("no description".to_string()))
                        );
                    }
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
    }
}
