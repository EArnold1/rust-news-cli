use std::{env, error::Error};

use colored::Colorize;
use dotenv::dotenv;
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
    articles: Vec<Article>, // source: String,
}

fn get_top_headlines(url: &str) -> Result<Articles, Box<dyn Error>> {
    let req = reqwest::blocking::get(url)?;

    let articles: Articles = req.json()?;

    Ok(articles)
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

fn main() {
    dotenv().ok();

    let api_key: String = match env::var("NEWS_API_KEY") {
        Ok(v) => v,
        Err(_) => panic!("news api key is required"),
    };

    // make request to news api
    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=us&apiKey={apiKey}",
        apiKey = api_key
    );

    let data = get_top_headlines(url.as_str());

    let articles = match data {
        Ok(articles) => articles,
        Err(err) => {
            println!("{}, Error: {}", "something went wrong".red(), err);
            return;
        }
    };

    render_top_headlines(&articles);
}
