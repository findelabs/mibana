//use crate::cache::Cache;
use crate::db;
use crate::tera_build;
use crate::tools;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::net::Ipv4Addr;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error + Send + Sync>>;


// This is the main handler, to catch any failures in the echo fn
pub async fn main_handler(
    req: Request<Body>,
    db: db::DB,
    client: Option<Ipv4Addr>,
) -> BoxResult<Response<Body>> {
    match echo(req, db, client).await {
        Ok(s) => {
            log::debug!("Handler got success");
            Ok(s)
        }
        Err(e) => {
            log::error!("Handler caught error: {}", e);
            let mut response = Response::new(Body::from(format!("{{\"error\" : \"{}\"}}", e)));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(response)
        }
    }
}

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
        (&Method::GET, "/query") => {
            // Split apart request
            let (parts, _body) = req.into_parts();

            // Create queriable hashmap from queries
            let queries = tools::queries(&parts).expect("Failed to generate hashmap of queries");

            let default = "test".to_owned();
            let collection = queries.get("collection").unwrap_or(&default);

            // Get posts based on query param
            let results = match queries.get("query") {
                Some(query) => db.search(query, collection).await?,
                None => {
                    let mut results = Vec::new();
                    let string = "this is a test".to_owned();
                    let collections = db.collections().await?;
                    results.push(string);
                    db::Hits {
                        results,
                        collections 
                    }
                }
            };
            let index = tera_build::tera_create(results, "index.html")?;
            Ok(Response::new(Body::from(index)))
        }
        (&Method::GET, "/favicon.ico") => {
            let bytes: Vec<u8> = include_bytes!("site/images/favicon.ico").to_vec();
            let mut response = Response::new(Body::from(bytes));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/x-icon"));
            Ok(response)
        }
        (&Method::GET, "/search_icon.svg") => {
            let bytes: Vec<u8> = include_bytes!("site/images/search_icon.svg").to_vec();
            let mut response = Response::new(Body::from(bytes));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
            Ok(response)
        }
        (&Method::GET, "/style.css") => {
            let file = include_str!("site/templates/style.css");
            let mut response = Response::new(Body::from(file));
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));
            Ok(response)
        }
        (&Method::GET, "/tools.js") => {
            let file = include_str!("site/templates/tools.js");
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
