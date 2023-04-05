use ansi_term::Colour;
use octocrab::params;

pub async fn run_pulls(repository: &str, limit: u8) {
    let parts = repository.split('/').collect::<Vec<&str>>();

    let owner = parts.first().unwrap().to_owned();
    let repo = parts.get(1).unwrap().to_owned();

    let page = octocrab::instance()
        .pulls(owner, repo)
        .list()
        // Optional Parameters
        .state(params::State::Open)
        .sort(params::pulls::Sort::Updated)
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
                Colour::RGB(150, 150, 150).paint(format!(
                    "[{}]",
                    item.updated_at
                        .map_or("unknown".to_string(), |t| t.to_string())
                )),
                Colour::Green.paint(
                    item.html_url
                        .map_or("no url".to_string(), |u| u.to_string())
                ),
                Colour::Yellow.paint(item.title.unwrap_or("no title".to_string()))
            )
        }
    }
}
