extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn main() -> Result<(), Box<std::error::Error>> {
    // let title = document.find(Name("header")).next().unwrap().text();
    hacker_news("https://news.ycombinator.com");
    //todo dinamodb  and title, etc.. with REST service
    Ok(()) 
}
fn news()  {
//    let web_news = "https://www.lavanguardia.com/internacional/20190518/462299498579/iran-eeuu-armada-china-golfo-persico.html";
//    let resp = reqwest::get(web_news)?.text()?;
//    let document = Document::from(resp.as_str());

//    let to_parse_text = document.find(Class("content-structure")).next().unwrap().text();
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
