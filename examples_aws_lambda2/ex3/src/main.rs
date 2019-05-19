extern crate reqwest;
extern crate select;
extern crate dynomite;
extern crate rusoto_core;

mod db;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};


static NAME_ID: &'static str  = "id";
static NAME_TABLE: &'static str  = "test";




fn main() -> Result<(), Box<std::error::Error>> {
    
    let client = db::DummyClientDB::new(NAME_ID.to_string(), NAME_TABLE.to_string());
    let test= db::Test::new("45".to_string(), "param1".to_string());
    let result_put = client.clone().put(&test);
    println!("{:#?}", result_put);

    let result_get = client.clone().get("45");
    println!("{:#?}", result_get);

    let results = client.clone().list();
    println!("{:#?}",results);
    Ok(()) 
}



fn scrap_news() -> Result<(), Box<std::error::Error>>  {
    let web_news = "https://www.lavanguardia.com/internacional/20190518/462299498579/iran-eeuu-armada-china-golfo-persico.html";
    let resp = reqwest::get(web_news)?.text()?;
    let document = Document::from(resp.as_str());
    let to_parse_text = document.find(Class("content-structure")).next().unwrap().text();
    Ok(())
}
fn hacker_news(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    let document = Document::from_read(resp).unwrap();
    
    for node in document.find(Class("athing")) {
        let rank = node.find(Class("rank")).next().unwrap();
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        let url = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap();
        let url_txt = url.attr("href").unwrap();
        let url_trim = url_txt.trim_left_matches('/');
        println!("rank {} story {} url {}", rank.text(), story, url_txt);

    }
}
