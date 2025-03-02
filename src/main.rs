use std::env;

use colored::Colorize;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Source {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
    source: Source,
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

async fn get_top_headlines(client: &Client, url: &str) -> Result<Articles, String> {
    let response = match client.get(url).send().await {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to send request: {}", e)),
    };

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Articles>().await {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(format!("Failed to parse JSON response: {}", e)),
        },
        other => Err(format!("Unexpected response status: {}", other)),
    }
}

fn render_top_headlines(articles: &Articles) {
    for article in articles.articles.iter() {
        let source = format!(
            "{tag} {source}",
            tag = "> source:".green(),
            source = article.source.name.red().bold()
        );

        println!("{source}");
        println!("> {}", article.title.yellow());
        println!("> {} \n", article.url.blue());
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key: String = match env::var("NEWS_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{}", "Error: NEWS_API_KEY is required.".red());
            return;
        }
    };

    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={apiKey}",
        apiKey = api_key
    );

    let client: reqwest::Client = reqwest::Client::new();

    match get_top_headlines(&client, &url).await {
        Ok(articles) => render_top_headlines(&articles),
        Err(err) => eprintln!("{} {}", "Error:".red(), err),
    }
}
