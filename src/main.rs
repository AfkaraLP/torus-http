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

    server.listen(("127.0.0.1", 8080)).unwrap();
}

#[must_use]
pub fn hello_world(req: Request) -> impl Response {
    HttpResponse::new()
        .set_body(format!(
            "<h1>hey there from me</h1><p>this is a test, your headers are: {:#?}</p>",
            req.headers
        ))
        .insert_header("Content-Type", "text/html")
}
