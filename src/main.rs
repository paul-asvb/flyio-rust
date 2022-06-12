use std::{env, net::SocketAddr};

use axum::{
    http::{Request, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};

mod repo;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/test", get(test));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

fn test() -> Html<String> {
    dotenv::dotenv().ok();
    let client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = redis::cmd("SET")
        .arg("my_key")
        .arg("42")
        .query(&mut con)
        .unwrap();
    let s: String = redis::cmd("GET").arg("my_key").query(&mut con).unwrap();
    Html(s)
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::repo::Peer;

    #[test]
    fn test_struct_deserialise() {
        let data = r#"{"peer_id":"myid","offer":{"type":"type1","sdp":"sdp_example"}}"#;
        let _v: Peer = serde_json::from_str(data).unwrap();
    }
    extern crate redis;

    #[tokio::test]
    async fn test_redis_connection() {
        dotenv::dotenv().ok();

        let client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
        let mut con = client.get_connection().unwrap();
        let _: () = redis::cmd("SET")
            .arg("my_key")
            .arg("42")
            .query(&mut con)
            .unwrap();
        let bar: String = redis::cmd("GET").arg("my_key").query(&mut con).unwrap();
        dbg!(bar);
    }
}
