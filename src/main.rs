use toy_rusttp::prelude::*;

fn main() {
    let server: HttpServer<_> = HttpServer::new(("127.0.0.1", 8080))
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

    _ = server.run();
}

#[must_use]
pub fn hello_world(req: Request) -> impl Response {
    format!(
        "hello, kind world... I will now proceed to print your headers: {:#?}",
        req.headers
    )
}
