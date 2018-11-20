extern crate aws_lambda as lambda;


fn main() {
    lambda::gateway::start(|_req| {
        let res = lambda::gateway::response().status(200)
            .body(lambda::gateway::Body::from("Hello, World!"))?;
        Ok(res)        
    })
}
