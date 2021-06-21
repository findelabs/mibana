//use cache::Cache;
use chrono::Local;
use clap::{crate_version, App, Arg};
use db::DB;
use env_logger::Builder;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use log::LevelFilter;
use std::io::Write;

//mod cache;
mod db;
mod server;
mod tera_build;
mod tools;

use tools::get_client_ip;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = App::new("mongodb-frontpage")
        .version(crate_version!())
        .author("Daniel F. <dan@findelabs.com>")
        .about("Frontpage to a MongoDB Database")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(true)
                .value_name("URL")
                .help("MongoDB URL")
                .env("MONGODB_URL")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Set port to listen on")
                .required(false)
                .env("LISTEN_PORT")
                .default_value("8080")
                .takes_value(true),
        )
        .get_matches();

    let url = &opts.value_of("url").unwrap();
    let port: u16 = opts.value_of("port").unwrap().parse().unwrap_or_else(|_| {
        eprintln!("specified port isn't in a valid range, setting to 8080");
        8080
    });

    // Initialize log Builder
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{{\"date\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"}}",
                Local::now().format("%Y-%m-%dT%H:%M:%S:%f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let db = DB::init(&url).await?;

    // Get initial cache
//    let mut cache = Cache::new();
//    cache.posts(db.clone()).await?;

    let addr = ([0, 0, 0, 0], port).into();
    let service = make_service_fn(move |socket: &AddrStream| {
        let db = db.clone();
        let client = get_client_ip(socket.remote_addr());
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                server::echo(req, db.clone(), client)
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    log::info!(
        "Starting mongodb-frontpage:{} on http://{}",
        crate_version!(),
        addr
    );

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
