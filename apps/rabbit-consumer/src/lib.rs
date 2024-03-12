use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_rabbit_consumer(req: Request) -> anyhow::Result<impl IntoResponse> {
    let req_path = req.header("spin-full-url").unwrap().as_str().unwrap();
    let req_body = String::from_utf8(req.body().to_vec())?;

    println!(
        "Handling request to {:?}, with body {:?}",
        req_path,
        req_body
    );

    Ok(Response::builder().status(200).build())
}
