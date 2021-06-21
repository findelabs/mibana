//use crate::cache::Cache;
use crate::db;
use crate::tera_build;
use crate::tools;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::net::Ipv4Addr;

pub async fn echo(
    req: Request<Body>,
    db: db::DB,
    client: Option<Ipv4Addr>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let ip = match client {
        Some(ip) => ip.to_string(),
        None => "unknown".to_owned(),
    };

    // Log info
    log::info!(
        "method={} path={} ip={}",
        req.method(),
        req.uri().path(),
        ip
    );

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            // Split apart request
            let (parts, _body) = req.into_parts();

            // Create queriable hashmap from queries
            let queries = tools::queries(&parts).expect("Failed to generate hashmap of queries");

            // Get posts based on query param
            let results = match queries.get("query") {
                Some(query) => db.search(query).await?,
                None => db::Hits::default(),
            };
            let index = tera_build::tera_create(results, "index.html")?;
            Ok(Response::new(Body::from(index)))
        }
        (&Method::GET, "/favicon.ico") => {
            let bytes: Vec<u8> = std::fs::read("/app/site/images/favicon.ico")?;
            let mut response = Response::new(Body::from(bytes));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/x-icon"));
            Ok(response)
        }
        (&Method::GET, "/search_icon.svg") => {
            let bytes: Vec<u8> = std::fs::read("/app/site/images/search_icon.svg")?;
            let mut response = Response::new(Body::from(bytes));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
            Ok(response)
        }
        (&Method::GET, "/style.css") => {
            let file = std::fs::read_to_string("/app/site/templates/style.css")?;
            let mut response = Response::new(Body::from(file));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));
            Ok(response)
        }
        (&Method::GET, "/tools.js") => {
            let file = std::fs::read_to_string("/app/site/templates/tools.js")?;
            let mut response = Response::new(Body::from(file));
            response.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/javascript"),
            );
            Ok(response)
        }
        // echo transformed card with received variables
        (&Method::GET, "/health") => Ok(Response::new(Body::from("ok".to_string()))),

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}