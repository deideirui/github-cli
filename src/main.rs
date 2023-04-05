use structopt::StructOpt;

pub mod features;

#[derive(Debug, StructOpt)]
#[structopt(name = "github-cli", about = "A GitHub client written in Rust")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "trending popular projects")]
    Trending {
        #[structopt(default_value, short, long = "lang")]
        language: String,

        #[structopt(default_value, short)]
        period: String,
    },
    #[structopt(about = "listing pull requests")]
    Pulls {
        repository: String,

        #[structopt(default_value = "20", short)]
        limit: u8,
    },
    #[structopt(about = "listing issues")]
    Issues {
        repository: String,

        #[structopt(default_value = "20", short)]
        limit: u8,
    },
}

#[tokio::main]
async fn main() {
    let opt = Cli::from_args();

    match opt.command {
        Command::Trending { language, period } => {
            features::trending::run_trending(&language, &period).unwrap()
        }
        Command::Pulls { repository, limit } => {
            features::pulls::run_pulls(&repository, limit).await
        }
        Command::Issues { repository, limit } => {
            features::issues::run_issues(&repository, limit).await
        }
    }
}
