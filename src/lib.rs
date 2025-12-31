//! # toy-rusttp
//!
//! A small, synchronous http server library with a surprisingly pleasant
//! developer experience and minimal moving parts.
//!
//! ## Example usage:
//!
//! ```rust
//! use toy_rusttp::prelude::*;
//!
//! fn main() {
//!     let server: HttpServer = HttpServer::new()
//!         .get("/", hello_world)
//!         .route(
//!             "/hello",
//!             HttpMethod::Other("custom".into()),
//!             |_| "hello from a custom method",
//!         )
//!         .add_middleware(|req| {
//!             println!("got request: {req:#?}");
//!             req
//!         });
//!
//!     _ = server.listen(("127.0.0.1", 8080));
//! }
//!
//! pub fn hello_world(req: Request) -> impl Response {
//!     format!(
//!         "hello, kind world... I will now proceed to print your headers: {:#?}",
//!         req.headers
//!     )
//! }
//! ```

pub mod method;
pub mod prelude;
pub mod request;
pub mod response;
pub mod server;
pub mod status;
