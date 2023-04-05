use ansi_term::Colour;
use octocrab::params;

pub async fn run_issues(repository: &str, limit: u8) {
    let parts = repository.split('/').collect::<Vec<&str>>();

    let owner = parts.first().unwrap().to_owned();
    let repo = parts.get(1).unwrap().to_owned();

    let page = octocrab::instance()
        .issues(owner, repo)
        .list()
        // Optional Parameters
        .state(params::State::Open)
        .sort(params::issues::Sort::Updated)
        .direction(params::Direction::Descending)
        .per_page(limit)
        .page(1u32)
        // Send the request
        .send()
        .await;

    if let Ok(page) = page {
        for item in page.items {
            println!(
                "{} {} {}",
                Colour::RGB(150, 150, 150).paint(format!("[{}]", item.updated_at)),
                Colour::Green.paint(item.html_url.to_string()),
                Colour::Yellow.paint(item.title)
            )
        }
    }
}
