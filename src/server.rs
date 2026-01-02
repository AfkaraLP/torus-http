//! The actual http server on which you define your routes
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, ToSocketAddrs},
    str::{FromStr, Utf8Error, from_utf8},
    sync::Arc,
};

use crate::{method::HttpMethod, request::HttpRequest, response::Response};

/// A generic trait to allow many different types of handlers to be passed into our http server
pub trait HandlerFn: Send + Sync {
    fn call(&self, req: HttpRequest) -> Box<dyn Response>;
}

impl<F, T> HandlerFn for F
where
    F: Fn(HttpRequest) -> T + Send + Sync,
    T: Response + 'static,
{
    fn call(&self, req: HttpRequest) -> Box<dyn Response> {
        Box::new(self(req))
    }
}

pub type MiddleWareFn = fn(HttpRequest) -> HttpRequest;
pub type Handler = Box<dyn HandlerFn + Send + Sync>;
/// The struct to initialise your http server and finally listen on some port
///
/// # Example usage:
///
/// ```rust
/// HttpServer::new().listen(("127.0.0.1", 8080)) // no_op http server listening on port 8080
/// ```
#[derive(Default)]
pub struct HttpServer {
    handlers: HashMap<(String, HttpMethod), Handler>,
    middle_ware: Option<MiddleWareFn>,
    state: Option<Box<dyn Send + Sync>>,
}

impl HttpServer {
    /// Initialise an http server on an address
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            middle_ware: None,
            state: None,
        }
    }

    /// Initialises middleware or replaces if there was already some added
    ///
    /// subject to change
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// HttpServer::new().add_middleware(|req| {
    ///     println!("we got request: {req:#?}");
    ///     req
    /// })
    /// ```
    #[must_use]
    pub fn add_middleware(mut self, f: fn(req: HttpRequest) -> HttpRequest) -> Self {
        self.middle_ware.replace(f);
        self
    }

    /// Register a custom route
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// HttpServer::new().route("/some_path", HttpMethod::Other("custom"), |_| {"hi"})
    /// ```
    #[must_use]
    pub fn route<F: HandlerFn + 'static>(
        mut self,
        path: impl Into<String>,
        method: HttpMethod,
        f: F,
    ) -> Self {
        self.handlers.insert((path.into(), method), Box::new(f));
        self
    }

    /// Register a **GET** method
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn home_method(_req: HttpRequest) -> impl Response {
    ///     "hello, world"
    /// }
    /// HttpServer::new().get("/home", )
    /// ```
    ///
    /// ## Note:
    ///
    /// I drop the body for get requests as that is apparently standard
    #[must_use]
    pub fn get<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Get, f)
    }

    /// Register a **POST** method
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn my_post(_req: HttpRequest) -> impl Response {
    ///     // ... Super complex DB activity
    ///     "I'll keep you posted"
    /// }
    /// HttpServer::new().post("/drop/prod/db", my_post)
    /// ```
    #[must_use]
    pub fn post<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Post, f)
    }

    /// Register a **DELETE** method
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn my_delete(_req: HttpRequest) -> impl Response {
    ///     // delete browser history ...
    ///     "Yeah I don't use the internet bro trust me..."
    /// }
    /// HttpServer::new().delete("/homework", my_delete)
    /// ```
    #[must_use]
    pub fn delete<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Delete, f)
    }

    /// Register an **UPDATE** method
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn im_getting_tired_of_writing_these(_req: HttpRequest) -> impl Response {
    ///     // just read the others like .get() and .post() bro
    ///     "Yeah I don't use the internet bro trust me..."
    /// }
    /// HttpServer::new().delete("/homework", im_getting_tired_of_writing_these)
    /// ```
    #[must_use]
    pub fn update<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Update, f)
    }

    /// Register a **PUT** method
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn im_getting_tired_of_writing_these(_req: HttpRequest) -> impl Response {
    ///     "WHY THE HECK DID I ADD SO MANY OF THESE THINGS"
    /// }
    /// HttpServer::new().delete("/us-east1", im_getting_tired_of_writing_these)
    /// ```
    #[must_use]
    pub fn put<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Put, f)
    }

    /// like `.post()` but patch
    #[must_use]
    pub fn patch<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Patch, f)
    }

    /// I just took this one from hoppscotch I never heard of the head method before
    /// read `.post()` and stuff for documentation
    #[must_use]
    pub fn head<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Head, f)
    }

    /// Shoutout to chatgpt for this one:
    /// Register an **OPTIONS** method
    ///
    /// This attaches a handler to the given path that responds to http `OPTIONS`
    /// requests. Typically used for capability discovery, CORS preflight checks,
    /// or politely telling browsers what they are allowed to do.
    ///
    /// # Example usage:
    ///
    /// ```rust
    /// fn options_method(_req: HttpRequest) -> impl Response {
    ///     ""
    /// }
    ///
    /// HttpServer::new().options("/home", options_method);
    /// ```
    ///
    /// ## Note:
    ///
    /// `OPTIONS` requests are generally expected to return headers describing
    /// allowed methods and behaviors. A response body is usually unnecessary and
    /// often ignored, but nothing is stopping you from adding one if you enjoy
    /// disappointing strict HTTP purists.
    #[must_use]
    pub fn options<F: HandlerFn + 'static>(self, path: impl Into<String>, f: F) -> Self {
        self.route(path, HttpMethod::Options, f)
    }

    #[must_use]
    pub fn set_state<T: Send + Sync + 'static>(mut self, state: T) -> Self {
        self.state.replace(Box::new(state));
        self
    }

    /// Start your http server
    ///
    /// # Errors
    ///
    /// - Failed binding listener to address
    /// - Failed reading the stream to the buffer
    /// - Failed getting the stream
    /// - Failed parsing the request
    /// - Failed flushing to the stream
    pub fn listen(self, address: impl ToSocketAddrs) -> Result<(), ServerError> {
        let listener = TcpListener::bind(address)?;
        let handlers = Arc::new(self.handlers);
        let middle_ware = Arc::new(self.middle_ware);
        loop {
            for stream in listener.incoming() {
                let stream = stream?;
                let middle_ware = middle_ware.clone();
                let handlers = handlers.clone();
                std::thread::spawn(move || {
                    _ = Self::handle_connection(&handlers, &middle_ware, stream);
                });
            }
        }
    }

    fn handle_connection(
        // server: &Arc<HttpServer<T>>,
        handlers: &Arc<HashMap<(String, HttpMethod), Handler>>,
        middle_ware: &Arc<Option<MiddleWareFn>>,
        mut stream: std::net::TcpStream,
    ) -> Result<(), ServerError> {
        let mut buf = [0; 4096 * 4];
        let n = stream.read(&mut buf)?;
        let request = {
            let request = HttpRequest::from_str(from_utf8(&buf[..n])?)?;
            if let Some(middle_ware) = **middle_ware {
                middle_ware(request)
            } else {
                request
            }
        };
        let path = request.path.clone();
        let method = request.method.clone();
        if let Some(handler) = handlers.get(&(path, method)) {
            let ret = handler.call(request);
            stream.write_all(ret.to_response().into_bytes().as_slice())?;
        } else {
            stream.write_all(&"no method found".to_response().into_bytes())?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ServerError {
    Utf8Conversion(Utf8Error),
    IoError(std::io::Error),
}

impl From<Utf8Error> for ServerError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Conversion(value)
    }
}
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
