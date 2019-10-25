#![allow(unused)]

#[macro_use]
extern crate log;

use std::net::SocketAddr;
use futures::{future, future::Either, Future};
use env_logger::{Builder, Env};
use b_error::{BResult, BError};
use http::status::StatusCode;
use http::Uri;
use hyper::{header, service::service_fn, Body, Request, Response, Server};
use structopt::StructOpt;

fn main() {
    b_error::main(run)
}

#[derive(Clone, StructOpt)]
#[structopt(about = "A basic HTTP file server")]
pub struct Config {
    /// Sets the IP:PORT combination
    #[structopt(
        name = "ADDR",
        short = "a",
        long = "addr",
        parse(try_from_str),
        default_value = "127.0.0.1:4000"
    )]
    addr: SocketAddr,
}

fn run() -> BResult<()> {
    env_logger::init();

    let config = Config::from_args();

    info!("ff-web {}", env!("CARGO_PKG_VERSION"));
    info!("addr: http://{}", config.addr);

    let server = Server::bind(&config.addr)
        .serve(move || {
            service_fn(move |req| {
                serve(req).map_err(|e| {
                    error!("request handler error: {}", e);
                    e
                })
            })
        })
        .map_err(|e| {
            error!("server error: {}", e);
            ()
        });

    tokio::run(server);

    Ok(())
}

fn serve(req: Request<Body>) -> Box<dyn Future<Item = Response<Body>, Error = BError> + Send> {
    use hyper::Method;
    let method = req.method();
    let uri = req.uri();
    let path = uri.path();

    match (method, path) {
        (&Method::GET, "/random-match") => {
            make_match()
        }
        _ => {
            make_404()
        }
    }
}

fn make_match() -> Box<dyn Future<Item = Response<Body>, Error = BError> + Send> {
    panic!()
}

fn make_404() -> Box<dyn Future<Item = Response<Body>, Error = BError> + Send> {
    Box::new(future::result(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .map_err(|e| BError::from_source(e, "making response"))
    ))
}
