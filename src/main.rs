use dotenv::dotenv;
use octocrab::params;
use octocrab::Octocrab;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let token = env::var("GITHUB_TOKEN").unwrap_or_default();
    let user = env::var("GITHUB_USER").unwrap_or_default();
    let repo = env::var("GITHUB_REPO").unwrap_or_default();

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let issue = octocrab
        .issues(user, repo)
        .list()
        .state(params::State::All)
        .send()
        .await?;

    issue.items.iter().for_each(|issue| {
        println!("Issue: {} - {:?}", issue.title, issue.body);
    });

    Ok(())
}
