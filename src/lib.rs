use anyhow::{Result, Ok};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn advent_challange_1(req: Request) -> Result<Response> {
    if req.method() != http::Method::POST {
        return method_not_allowed()
    }
    
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

fn method_not_allowed() -> Result<Response> {
    Ok(http::Response::builder()
        .status(405)
        .body(Some("Method not allowed.".into()))?)
}