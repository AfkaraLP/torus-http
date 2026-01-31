use torus_http::prelude::*;

fn main() {
    let server: HttpServer = HttpServer::new()
        .get("/", hello_world)
        .route(
            "/hello",
            HttpMethod::Other("custom".into()),
            |_| "hello from a custom method",
        )
        .add_middleware(|req| {
            println!("got request: {req:#?}");
            req
        });

    server
        .listen(("127.0.0.1", 8080))
        .expect("Failed listening...");
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn hello_world(req: HttpRequest) -> impl Response {
    HttpResponse::new()
        .set_body(format!(
            "<h1>hey there from torus!</h1><p>this is a test, your request is: {req:#?}</p>",
        ))
        .insert_header("Content-Type", "text/html")
}
