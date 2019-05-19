extern crate reqwest;
extern crate select;
extern crate dynomite;
extern crate rusoto_core;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use rusoto_core::Region;
use futures::{Stream, Future};
use tokio::runtime::Runtime;
use std::collections::HashMap;

use dynomite::{
    dynamodb::{
        AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, ScanInput,
    },
    retry::Policy,
    DynamoDbExt, FromAttributes, Item, Retries,
};



static NAME_ID: &'static str  = "id";
static NAME_TABLE: &'static str  = "test";

#[derive(Item, Debug, Clone)]
pub struct Test {
    id: String,
    param1: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    // let title = document.find(Name("header")).next().unwrap().text();
    //hacker_news("https://news.ycombinator.com");
    //todo dinamodb  and title, etc.. with REST service
    
  
    let test= Test {id: "45".to_string(), param1: "param1".to_string()};
    let result_put = put_element(&test);
    println!("{:#?}", result_put);

    let result_get = get_element("45");
    println!("{:#?}", result_get);

    let results = get_elements();
    println!("{:#?}",results);
    Ok(()) 
}




fn get_elements() -> Vec<Test> {
    let client = DynamoDbClient::new(Region::UsEast1).with_retries(Policy::default());
    let mut rt = Runtime::new().expect("failed to initialize futures runtime");
    let scanInput = ScanInput {limit: Some(1), table_name: NAME_TABLE.to_string(), ..ScanInput::default()};
    let values = rt.block_on(client.clone().scan_pages(scanInput).map(|item| { Test::from_attrs(item) }).collect());
    
    let result_values = match values {
        Ok(found_values) => { 
            found_values.into_iter().filter_map(Result::ok).collect()
        },
        Err(e) => {
            println!("{:#?}",e);
            Vec::new()
        },
    };
    result_values
}

fn get_element(id: &str) -> Option<Test>{

    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();
    query_key.insert(String::from(NAME_ID), AttributeValue{s: Some(id.to_string()), ..Default::default()});

    let client = DynamoDbClient::new(Region::UsEast1).with_retries(Policy::default());
    let mut rt = Runtime::new().expect("failed to initialize futures runtime");
    
    let getItemInput = GetItemInput{table_name: NAME_TABLE.to_string(), key: query_key.clone(), ..GetItemInput::default()};
    let value = rt.block_on(client.get_item(getItemInput).map(|result| result.item.map(Test::from_attrs)));
    
    let result = match value {
        Ok(val) => {
            let tmp_val = val.unwrap();
            tmp_val.ok()
        },
        Err(e) => { 
            println!("{:#?}",e);
            None }
    };
    result
}

fn put_element(test: &Test) -> bool {
    let client = DynamoDbClient::new(Region::UsEast1).with_retries(Policy::default());

    let mut rt = Runtime::new().expect("failed to initialize futures runtime");
    let putItemInput = PutItemInput{table_name: NAME_TABLE.to_string(), item: test.clone().into(), ..PutItemInput::default()};
    let values = rt.block_on(client.put_item(putItemInput));
    let result = match values {
        Ok(val) => {
            true
        }
        Err(e) => { 
            println!("{:#?}",e);
            false }
    };
    result
}


fn scrap_news() -> Result<(), Box<std::error::Error>>  {
    let web_news = "https://www.lavanguardia.com/internacional/20190518/462299498579/iran-eeuu-armada-china-golfo-persico.html";
    let resp = reqwest::get(web_news)?.text()?;
    let document = Document::from(resp.as_str());

    let to_parse_text = document.find(Class("content-structure")).next().unwrap().text();
    //TODO do things text
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
