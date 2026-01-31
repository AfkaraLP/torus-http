//! # toy-rusttp
//!
//! A small, synchronous http server library with a surprisingly pleasant
//! developer experience and minimal moving parts.
//!
//! ## Example usage:
//!
//! ```no_run
//! use torus_http::prelude::*;
//!
//! fn main() {
//!     let server: HttpServer = HttpServer::new()
//!         .get("/", hello_world)
//!         .route(
//!             "/hello",
//!             HttpMethod::other("custom"),
//!             |_| "hello from a custom method",
//!         )
//!         .add_middleware(|req| {
//!             println!("got request: {req:#?}");
//!             req
//!         });
//!     
//!     server
//!         .listen(("127.0.0.1", 8080))
//!         .expect("Failed listening...");
//! }
//!
//! pub fn hello_world(req: HttpRequest) -> impl Response {
//!     HttpResponse::new()
//!         .set_body(format!(
//!             "<h1>hey there from torus!</h1><p>this is a test, your request is: {req:#?}</p>",
//!         ))
//!         .insert_header("Content-Type", "text/html")
//! }
//! ```

pub mod method;
pub mod prelude;
pub mod request;
pub mod response;
pub mod server;
pub mod status;
