use std::error::Error;
//use reqwest::Method;
use serde::{Serialize, Deserialize};
use url::Url;


#[derive(Deserialize, Debug)]
struct Articles{
    articles: Vec<Article>
}


#[derive(Deserialize, Debug)]
struct Article{
    title: String,
    url: String
}

fn get_articles(url: &str) ->Result<Articles, Box<dyn Error>>
{
    let response = ureq::get(url).call()?.into_string()?;
    let articles = serde_json::from_str(&response)?;
    dbg!(articles);
    todo!()
}


fn main() {
    let url = "http://newsapi.org/v2/top-headlines?country=au&apiKey=1b5095baf0a3400289cc07d46afb1f40";
    let articles = get_articles(url);
    dbg!(articles);
}
/*err
     Running `target/debug/newscli`
[src/main.rs:31] articles = Err(
    Error("invalid type: map, expected unit", line: 1, column: 0),
) */