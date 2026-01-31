# TOy RUSttp

TORUS is a small, synchronous HTTP server written in Rust (by the way).

Built as a learning project and a place to experiment with low-level HTTP handling. Security is minimal, correctness is **good enough**, and polish is not the point.

Do not deploy this on the public internet unless you enjoy consequences.

*Has nice DX though...*

## Roadmap

- [x] Basic static routes  
- [ ] Dynamic routes  
- [ ] State management  
- [ ] Fewer bad ideas (unlikely)

## Non-Goals

- Be good
- Be fast
- Be secure
- Brew coffee

## Motivation

This project exists to better understand how HTTP servers work under the hood and to write flexible but intuitive Rust APIs

## Example usage:

```rust
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

pub fn hello_world(req: HttpRequest) -> impl Response {
    HttpResponse::new()
        .set_body(format!(
            "<h1>hey there from torus!</h1><p>this is a test, your request is: {req:#?}</p>",
        ))
        .insert_header("Content-Type", "text/html")
}
```
