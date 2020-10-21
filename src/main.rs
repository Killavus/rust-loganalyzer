use argh::FromArgs;
use std::{convert::Infallible, net::IpAddr};
use warp::Filter;

fn listen_port() -> usize {
    7667
}

fn listen_addr() -> &'static str {
    "127.0.0.1"
}

#[derive(FromArgs)]
/// Server configuration
struct ServerConfig {
    /// port on which server is listening
    #[argh(option, default = "listen_port()")]
    port: usize,
    /// host on which server is accepting connections
    #[argh(option, default = "String::from(listen_addr())")]
    server_addr: String,
}

const LOG_NAME: &'static str = "loganalyzer";

mod handlers {
    use std::convert::Infallible;

    pub async fn root() -> Result<impl warp::Reply, Infallible> {
        Ok(Ok(warp::reply::html("root")))
    }

    pub async fn collect(body: serde_json::Value) -> Result<impl warp::Reply, Infallible> {
        log::info!("{:#?}", body);
        Ok(warp::reply::html("collect"))
    }
}

mod routes {
    use super::handlers;
    use warp::Filter;

    pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        root().or(collect())
    }

    pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end().and(warp::get()).and_then(handlers::root)
    }

    pub fn collect() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path("collect")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handlers::collect)
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let logger = warp::log(LOG_NAME);

    let api = routes::api().with(logger);
    let config: ServerConfig = argh::from_env();

    let ip_addr = config
        .server_addr
        .parse::<IpAddr>()
        .expect("wrong listen address");

    warp::serve(api).run((ip_addr, config.port as u16)).await;
}
