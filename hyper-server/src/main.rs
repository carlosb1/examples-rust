extern crate hyper:
extern crate futures;

use std::{thread, time};
use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

fn heavy_work () -> String {
    let duration = time::Duration::from_millis(200);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
}
/*
fn call(&self, req: Request) -> Self::Future {
    futures::future::ok(match(req.method(), req.path())) {
    
    }
}
*/

fn main() {
    println!("Hello, world!");
}
